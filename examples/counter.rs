use std::net::SocketAddr;
use std::str::FromStr;

use rust_html_over_wire::action::Action;
use rust_html_over_wire::view::ViewHash;
use rust_html_over_wire::{html, render, Component, View, SERVER_COMPONENT_JS};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use solarsail::hyper::{header, StatusCode};
use solarsail::response::json;
use solarsail::route::{get, post};
use solarsail::{http, IntoResponse, Request, RequestExt, Response, SolarSail};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let app = SolarSail::new((), handle_request);
    app.run(&addr).await.unwrap();
}

async fn handle_request(_state: (), mut req: Request) -> Response {
    match req.route().as_tuple() {
        get!("health") => "Ok".into_response(),

        get!("server-component.js") => http::Response::builder()
            .header(header::CONTENT_TYPE, "text/javascript")
            .body(SERVER_COMPONENT_JS.into())
            .unwrap(),

        get!() => {
            let view = counter_component(0);
            let html = render(view).unwrap();
            let html = format!(
                r#"<script src="/server-component.js" async></script>{}"#,
                html
            );
            html.into_response()
        }

        post!("dispatch" / component) => {
            // TODO: remove to_string()
            #[allow(clippy::unnecessary_to_owned)]
            let update = handle_component(&component.to_string(), &mut req).await;
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

// #[action]
fn increment_counter(count: u32) -> u32 {
    count + 1
}

// #[component::server]
fn counter(count: u32) -> impl View<u32> {
    (
        html::div().content(format!("Count: {}", count)),
        html::button()
            .on_click(increment_counter_action)
            .content("incr"),
    )
}

// result of #[action]
#[allow(non_upper_case_globals)]
const increment_counter_action: Action<u32> =
    Action::new("counter::increment_counter", increment_counter);

// result of #[component]
pub fn counter_component(count: u32) -> Component<u32, impl View<u32>> {
    Component::new("counter::counter", count, counter)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Update {
    state: Box<RawValue>,
    view_hash: ViewHash,
    html: String,
}

fn handle_u32_action(state: u32, action: &str) -> u32 {
    match action {
        "counter::increment_counter" => (increment_counter_action.action)(state),
        _ => panic!("unknown u32 action with id: {}", action),
    }
}

#[allow(unused)]
async fn handle_component(id: &str, req: &mut Request) -> Update {
    match id {
        "counter::counter" => {
            #[derive(Deserialize)]
            struct Dispatch {
                state: u32,
                action: String,
            }
            // TODO: unwrap
            let payload: Dispatch = req.body_mut().json().await.unwrap();
            let after = handle_u32_action(payload.state, &payload.action);
            let state = serde_json::value::to_raw_value(&after).unwrap();
            let component = counter_component(after);
            let (html, view_hash) = component.render_update().unwrap();
            Update {
                state,
                view_hash,
                html,
            }
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}
