use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use crabweb::component::registry::ComponentRegistry;
use crabweb::{action, component, html, render, View, SERVER_COMPONENT_JS};
use solarsail::hyper::body::Buf;
use solarsail::hyper::{self, header, StatusCode};
use solarsail::response::json;
use solarsail::route::{get, post};
use solarsail::{http, IntoResponse, Request, RequestExt, Response, SolarSail};

#[tokio::main]
async fn main() {
    let registry = ComponentRegistry::default();

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let app = SolarSail::new(Arc::new(registry), handle_request);
    app.run(&addr).await.unwrap();
}

async fn handle_request(registry: Arc<ComponentRegistry>, mut req: Request) -> Response {
    match req.route().as_tuple() {
        get!("health") => "Ok".into_response(),

        get!("server-component.js") => http::Response::builder()
            .header(header::CONTENT_TYPE, "text/javascript")
            .body(SERVER_COMPONENT_JS.into())
            .unwrap(),

        get!() => {
            let view = items(vec!["first".into(), "second".into()]);
            let html = render(view).unwrap();
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

            let whole_body = hyper::body::aggregate(body).await.unwrap();
            let rd = whole_body.reader();

            let update = registry.handle(&id, rd, &action).expect("known component");
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

#[action]
fn add_item(mut items: Vec<Cow<'static, str>>) -> Vec<Cow<'static, str>> {
    items.push("new item 1".into());
    items.push("new item 2".into());
    items
}

type Items = Vec<Cow<'static, str>>;

#[component]
fn items(items: Items) -> impl View<Items> {
    (
        html::ul(items.into_iter().map(html::li)),
        html::div(html::button("add").on_click(add_item)),
    )
}
