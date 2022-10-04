use std::net::SocketAddr;
use std::str::FromStr;

use crabweb::component::registry::ComponentRegistry;
use crabweb::{html, render, IntoView, View, SERVER_COMPONENT_JS};
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
    counter(0).await
}

#[crabweb::component]
async fn counter(count: u32) -> impl View<u32> {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    (
        //     (self.0 == 0).then(|| ),
        //     (self.0 > 0).then(move || html::div(html::text!("Count: {}", self.0))),
        if count > 0 {
            // TODO: reintroduce html::text!
            html::div(format!("Count: {}", count)).boxed()
        } else {
            html::div("Hit `incr` to start counting ...").boxed()
        },
        html::button("incr").on_click(incr, ()),
    )
}
