use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use crabweb::component::registry::ComponentRegistry;
use crabweb::html::InputEvent;
use crabweb::{component, event, html, render, View, SERVER_COMPONENT_JS};
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
            let view = input("".into());
            let html = render(view).unwrap();
            let html = format!(
                r#"<script src="/server-component.js" async></script>{}"#,
                html
            );
            html.into_response()
        }

        post!("dispatch" / component / action / "input") => {
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

            let update = registry
                .handle_event(&id, rd, &action, "input")
                .expect("known component");
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

#[event]
fn handle_input(_state: Cow<'static, str>, ev: InputEvent) -> Cow<'static, str> {
    ev.value.into()
}

#[component]
pub fn input(value: Cow<'static, str>) -> impl View<Cow<'static, str>> {
    (
        html::div().content(format!("Value: {}", value)),
        html::custom("input")
            .attr("value", value)
            .on_input(handle_input),
    )
}
