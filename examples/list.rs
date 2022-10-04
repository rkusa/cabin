use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;

use crabweb::component::registry::ComponentRegistry;
use crabweb::{html, render, View, SERVER_COMPONENT_JS};
use serde::{Deserialize, Serialize};
use solarsail::hyper::body::to_bytes;
use solarsail::hyper::{header, StatusCode};
use solarsail::response::json;
use solarsail::route::{get, post};
use solarsail::{http, IntoResponse, Request, RequestExt, Response, SolarSail};

#[tokio::main]
async fn main() {
    // ensure registry is initialized
    ComponentRegistry::global();

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let app = SolarSail::new((), handle_request);
    app.run(&addr).await.unwrap();
}

async fn handle_request(_: (), mut req: Request) -> Response {
    match req.route().as_tuple() {
        get!("health") => "Ok".into_response(),

        get!("server-component.js") => http::Response::builder()
            .header(header::CONTENT_TYPE, "text/javascript")
            .body(SERVER_COMPONENT_JS.into())
            .unwrap(),

        get!() => {
            let view = app().await;
            let html = render(view).await.unwrap();
            let html = format!(
                r#"<script src="/server-component.js" async></script>{}"#,
                html
            );
            html.into_response()
        }

        post!("dispatch" / component / action) => {
            // TODO: get rid of to_string()
            let id = component.to_string();
            let action = action.to_string();

            // TODO: unwrap()
            let (body, _mime_type) = req.body_mut().take().unwrap();
            // TODO: test mime type
            // if let Some(mime_type) = mime_type {
            //     if mime_type != mime::APPLICATION_JSON {
            //         return Err(BodyError::WrongContentType("application/json"));
            //     }
            // }

            // let whole_body = hyper::body::aggregate(body).await.unwrap();
            // let rd = whole_body.reader();
            let data = to_bytes(body).await.unwrap();
            let update = ComponentRegistry::global()
                .handle(&id, &action, data)
                .await
                .expect("unknown component");
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn app() -> impl View {
    items(Items(vec![
        Item {
            id: 1,
            name: "first".into(),
        },
        Item {
            id: 2,
            name: "second".into(),
        },
    ]))
    .await
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: usize,
    name: Cow<'static, str>,
}

#[derive(Serialize, Deserialize)]
struct Items(Vec<Item>);

#[crabweb::component]
async fn items(items: Items) -> impl View<Items> {
    async fn add(mut items: Items, _: ()) -> Items {
        let max_id = items.0.iter().map(|i| i.id).max().unwrap_or(0);
        items.0.push(Item {
            id: max_id + 1,
            name: "new item 1".into(),
        });
        items.0.push(Item {
            id: max_id + 2,
            name: "new item 2".into(),
        });
        items
    }

    // TODO: concurrent deletes race each other
    async fn delete(mut items: Items, id: usize) -> Items {
        items.0.retain(|i| i.id != id);
        items
    }

    (
        html::ul(
            items
                .0
                .into_iter()
                .map(|item| html::li((item.name, html::button("x").on_click(delete, item.id)))),
        ),
        html::div(html::button("add").on_click(add, ())),
    )
}
