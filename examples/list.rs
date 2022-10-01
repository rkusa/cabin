#![feature(type_alias_impl_trait)]

use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use crabweb::component::registry::ComponentRegistry;
use crabweb::component::Render;
use crabweb::{html, render, Component, IntoView, View, SERVER_COMPONENT_JS};
use serde::{Deserialize, Serialize};
use solarsail::hyper::body::to_bytes;
use solarsail::hyper::{header, StatusCode};
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
            let view = app();
            let html = render(view).unwrap();
            let html = format!(
                r#"<script src="/server-component.js" async></script>{}"#,
                html
            );
            html.into_response()
        }

        post!("dispatch" / component) => {
            // TODO: get rid of to_string()
            let id = component.to_string();

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
            let update = registry.handle(&id, &data).expect("unknown component");
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

// TODO: return impl IntoView ?
fn app() -> impl View {
    Items(vec!["first".into(), "second".into()]).into_view()
}

#[derive(Serialize, Deserialize, Component)]
struct Items(Vec<Cow<'static, str>>);

#[derive(Serialize, Deserialize)]
enum ItemsAction<'v> {
    Add,
    Delete(&'v str),
}

impl Render for Items {
    type Message<'v> = ItemsAction<'v>;
    type View<'v> = impl View<Self::Message<'v>> + 'v;

    fn update(&mut self, message: Self::Message<'_>) {
        match message {
            ItemsAction::Add => {
                self.0.push("new item 1".into());
                self.0.push("new item 2".into());
            }
            ItemsAction::Delete(item) => self.0.retain(|i| i != item),
        }
    }

    fn render(&self) -> Self::View<'_> {
        (
            html::ul(self.0.iter().map(|item| {
                html::li((item, html::button("x").on_click(ItemsAction::Delete(item))))
            })),
            html::div(html::button("add").on_click(ItemsAction::Add)),
        )
    }
}
