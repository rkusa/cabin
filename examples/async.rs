#![feature(type_alias_impl_trait)]

use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;

use crabweb::component::registry::ComponentRegistry;
use crabweb::component::{ComponentId, ServerComponent};
use crabweb::{html, render, View, SERVER_COMPONENT_JS};
use html::events::InputEvent;
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

// TODO: return impl IntoView ?
async fn app() -> impl View {
    search(Search::new("Ge")).await
}

#[derive(Default, Serialize, Deserialize)]
struct Search {
    query: Cow<'static, str>,
}

impl Search {
    fn new(query: &'static str) -> Self {
        Self {
            query: query.into(),
        }
    }
}

// #[component]
async fn search(state: Search) -> impl View {
    static ID: ComponentId = ComponentId::new(module_path!(), "search");

    async fn search(state: Search) -> impl View<Search> {
        #[::linkme::distributed_slice(crabweb::component::registry::COMPONENT_FACTORIES)]
        fn __register_search_component(r: &mut ComponentRegistry) {
            r.register::<Search, InputEvent, _, _, _>(ID, "set_query", set_query, search);
        }

        // #[component::init]
        async fn init(_state: &mut Search) {
            // state.results = search(&state.query).await;
        }

        // TODO: allow both async and sync (convert sync to async?)
        // #[component::action] ?
        async fn set_query(mut state: Search, ev: InputEvent) -> Search {
            state.query = ev.value.into();
            state
        }
        // const set_query: Action = Action::new(module_path!(), "set_query", set_query);

        // TODO: memo?
        let items = search_countries(&state.query).await;

        // TODO: wrap in <server-component>
        (
            html::div(html::input().attr("value", state.query).on_input(set_query)),
            html::div(html::ul(items.into_iter().map(html::li))),
        )
    }
    // TODO: remove possible r# prefix
    ServerComponent::new(ID, state, search)
}

async fn search_countries(query: &str) -> Vec<Cow<'static, str>> {
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
