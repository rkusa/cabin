use std::borrow::Cow;
use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use html::events::InputEvent;
use rustend::{html, rustend_scripts, rustend_stylesheets, view, IntoView, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    view![
        rustend_stylesheets(),
        rustend_scripts(),
        search(Search::new("Ge")).await
    ]
}

#[derive(Default, Hash, Serialize, Deserialize)]
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

#[rustend::component]
async fn search(state: Search) -> Result<impl View, Infallible> {
    async fn set_query(mut state: Search, ev: InputEvent) -> Search {
        state.query = ev.value.into();
        state
    }

    let items = search_countries(&state.query).await;

    Ok(html::div![
        html::div(html::input().attr("value", state.query).on_input(set_query)),
        html::div(html::ul(items.into_iter().map(html::li))),
    ])
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

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = rustend::render_to_response(app).await;
                let (parts, body) = res.into_parts();
                Response::from_parts(parts, Full::new(body).boxed())
            }),
        )
        .layer(rustend_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
