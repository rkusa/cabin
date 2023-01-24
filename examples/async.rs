use std::borrow::Cow;
use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use html::events::InputEvent;
use hyper::service::make_service_fn;
use rustend::{html, view, View};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(app);
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}

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

#[rustend::component]
async fn search(state: Search) -> impl View {
    async fn set_query(mut state: Search, ev: InputEvent) -> Search {
        state.query = ev.value.into();
        state
    }

    let items = search_countries(&state.query).await;

    html::div![
        html::div(html::input().attr("value", state.query).on_input(set_query)),
        html::div(html::ul(items.into_iter().map(html::li))),
    ]
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
