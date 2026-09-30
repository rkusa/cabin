use std::time::Duration;

use bytes::Bytes;
use cabin::boundary_registry::BoundaryRegistry;
use cabin::prelude::*;
use cabin::scope::event;
use cabin::view::Boundary;
use cabin::{Error, Event, Exposed, StatusCode};
use http::Request;
use http_body_util::Full;
use serde::{Deserialize, Serialize};

/// Only user `1` is allowed.
#[derive(Clone, Copy, Serialize, Deserialize)]
struct UserId(u32);

impl Exposed for UserId {
    async fn validate(&self) -> Result<(), Error> {
        tokio::task::yield_now().await;
        if self.0 == 1 {
            Ok(())
        } else {
            Err(Error::from_status_code(StatusCode::FORBIDDEN))
        }
    }
}

#[derive(Clone, Exposed, Serialize, Deserialize)]
struct Filter {
    id: UserId,
    other: Option<UserId>,
    #[exposed(skip)]
    timeout: Duration,
}

#[derive(Clone, Exposed, Serialize, Deserialize)]
enum Kind {
    All,
    User(UserId),
    Named { id: UserId },
}

#[derive(Clone, Exposed, Serialize, Deserialize)]
struct Wrap<T>(T);

#[derive(Clone, Copy, Event, Serialize, Deserialize)]
struct Select(UserId);

/// Not deserializable, can only be used to trigger a re-render.
#[derive(Clone, Copy, Event, Serialize)]
struct Refresh;

/// Validated manually, only non-zero values are allowed.
#[derive(Clone, Copy, Event, Serialize, Deserialize)]
#[event(manual_exposed)]
struct Resize(u32);

impl Exposed for Resize {
    async fn validate(&self) -> Result<(), Error> {
        if self.0 == 0 {
            Err(Error::from_status_code(StatusCode::UNPROCESSABLE_ENTITY))
        } else {
            Ok(())
        }
    }
}

#[cabin::boundary(Select, Refresh, Resize)]
fn user(
    filter: Filter,
    kind: Kind,
    wrapped: Wrap<Vec<UserId>>,
) -> Boundary<(Filter, Kind, Wrap<Vec<UserId>>)> {
    let id = event::<Select>().map(|Select(id)| id).unwrap_or(filter.id);
    h::text!("{} {:?}", id.0, filter.timeout).boundary((filter, kind, wrapped))
}

cabin::BOUNDARIES!();

const VALID_STATE: &str = r#"[{"id":1,"other":null,"timeout":{"secs":0,"nanos":0}},"All",[1]]"#;

async fn put(state: &str, event_id: &str, payload: &str) -> StatusCode {
    let mut registry = BoundaryRegistry::default();
    registry.add(&BOUNDARIES);
    let req = Request::builder()
        .method("PUT")
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(format!(
            r#"{{"eventId":"{event_id}","state":{state},"payload":{payload}}}"#
        ))))
        .unwrap();
    registry
        .handle("boundary_validation::user", req)
        .await
        .status()
}

async fn put_state(state: &str) -> StatusCode {
    put(state, "", "null").await
}

#[tokio::test]
async fn valid_args() {
    assert_eq!(put_state(VALID_STATE).await, StatusCode::OK);
}

#[tokio::test]
async fn validation_keeps_status() {
    let state = VALID_STATE.replace(r#""id":1"#, r#""id":2"#);
    assert_eq!(put_state(&state).await, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn validates_option_field() {
    let state = VALID_STATE.replace(r#""other":null"#, r#""other":2"#);
    assert_eq!(put_state(&state).await, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn validates_enum_variants() {
    for (kind, status) in [
        (r#"{"User":1}"#, StatusCode::OK),
        (r#"{"User":2}"#, StatusCode::FORBIDDEN),
        (r#"{"Named":{"id":1}}"#, StatusCode::OK),
        (r#"{"Named":{"id":2}}"#, StatusCode::FORBIDDEN),
    ] {
        let state = VALID_STATE.replace(r#""All""#, kind);
        assert_eq!(put_state(&state).await, status, "{kind}");
    }
}

#[tokio::test]
async fn validates_generic_field() {
    let state = VALID_STATE.replace(r#""All",[1]"#, r#""All",[1,2]"#);
    assert_eq!(put_state(&state).await, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn invalid_json_is_bad_request() {
    let state = VALID_STATE.replace(r#""id":1"#, r#""id":"x""#);
    assert_eq!(put_state(&state).await, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn valid_event() {
    let status = put(VALID_STATE, "boundary_validation::Select", "1").await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn event_validation_keeps_status() {
    let status = put(VALID_STATE, "boundary_validation::Select", "2").await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn invalid_event_json_is_bad_request() {
    let status = put(VALID_STATE, "boundary_validation::Select", r#""x""#).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn manually_validated_event() {
    let status = put(VALID_STATE, "boundary_validation::Resize", "1").await;
    assert_eq!(status, StatusCode::OK);
    let status = put(VALID_STATE, "boundary_validation::Resize", "0").await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn non_deserializable_event() {
    let status = put(VALID_STATE, "boundary_validation::Refresh", "null").await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn validates_nested_derived_types() {
    #[derive(Exposed)]
    struct Outer {
        inner: Inner,
    }

    #[derive(Exposed)]
    enum Inner {
        Filters(Vec<Wrap<Filter>>),
    }

    let outer = |id| Outer {
        inner: Inner::Filters(vec![Wrap(Filter {
            id: UserId(1),
            other: Some(UserId(id)),
            timeout: Duration::ZERO,
        })]),
    };
    assert!(outer(1).validate().await.is_ok());
    let err = outer(2).validate().await.unwrap_err();
    assert_eq!(
        http_error::HttpError::status_code(&err),
        StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn validates_recursive_types() {
    #[derive(Exposed)]
    struct Node {
        id: UserId,
        children: Vec<Node>,
    }

    let tree = |id| Node {
        id: UserId(1),
        children: vec![Node {
            id: UserId(1),
            children: vec![Node {
                id: UserId(id),
                children: vec![],
            }],
        }],
    };
    assert!(tree(1).validate().await.is_ok());
    let err = tree(2).validate().await.unwrap_err();
    assert_eq!(
        http_error::HttpError::status_code(&err),
        StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn validates_mutually_recursive_types() {
    // Only one type in the cycle needs to be boxed.
    #[derive(Exposed)]
    #[exposed(boxed)]
    struct A {
        id: UserId,
        b: Vec<B>,
    }

    #[derive(Exposed)]
    struct B {
        a: Option<Box<A>>,
    }

    let a = |id| A {
        id: UserId(1),
        b: vec![B {
            a: Some(Box::new(A {
                id: UserId(id),
                b: vec![],
            })),
        }],
    };
    assert!(a(1).validate().await.is_ok());
    let err = a(2).validate().await.unwrap_err();
    assert_eq!(
        http_error::HttpError::status_code(&err),
        StatusCode::FORBIDDEN
    );
}
