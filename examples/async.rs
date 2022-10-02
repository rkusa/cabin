#![feature(type_alias_impl_trait)]

use std::borrow::Cow;
use std::future::{ready, Future, Ready};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use crabweb::component::registry::ComponentRegistry;
use crabweb::component::Render;
use crabweb::html::events::InputValue;
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
            let html = render(view).await.unwrap();
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
            let update = registry.handle(&id, data).await.expect("unknown component");
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

// TODO: return impl IntoView ?
fn app() -> impl View {
    Search::new("G").into_view()
}

#[derive(Default, Serialize, Deserialize, Component)]
struct Search {
    query: Cow<'static, str>,
}

impl Search {
    fn new(query: &'static str) -> Self {
        Search {
            query: query.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
enum SearchAction {
    Search(InputValue),
}

impl Render for Search {
    type Message<'v> = SearchAction;
    type View<'v> = impl View<Self::Message<'v>> + 'v;

    type UpdateFuture<'v> = Ready<()>;
    type RenderFuture<'v> = impl Future<Output = Self::View<'v>> + Send + 'v;

    fn update(&mut self, message: Self::Message<'_>) -> Self::UpdateFuture<'_> {
        match message {
            SearchAction::Search(v) => self.query = v.into(),
        };
        ready(())
    }

    fn render(&self) -> Self::RenderFuture<'_> {
        async move {
            let items = search(&self.query).await;
            (
                html::div(
                    html::input()
                        .attr("value", "G")
                        .on_input(|ev| SearchAction::Search(ev.value)),
                ),
                html::div(html::ul(items.into_iter().map(html::li))),
            )
        }
    }
}

async fn search(query: &str) -> Vec<Cow<'static, str>> {
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Source: https://github.com/umpirsky/country-list/blob/master/data/en_US/country.txt
    const COUNTRIES: &str = include_str!("./countries.txt");
    let query = query.to_lowercase();
    COUNTRIES
        .lines()
        .filter(|country| country.to_lowercase().contains(&query))
        .map(Cow::Borrowed)
        .collect()
}
