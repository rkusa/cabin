#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::borrow::Cow;
use std::net::SocketAddr;

use cabin::html::events::InputValue;
use cabin::scope::take_event;
use cabin::signal::Signal;
use cabin::{html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    search().await
}

#[derive(Hash, Serialize, Deserialize)]
struct Search(InputValue);

async fn search() -> impl View {
    let mut query = Signal::restore_or("query", Cow::Borrowed("Ge"));
    if let Some(Search(v)) = take_event() {
        *query = v.into();
    }

    let items = search_countries(&query).await;

    html::div((
        html::div(
            html::input()
                .attr("value", query)
                .on_input(|ev| Search(ev.value)),
        ),
        html::div(html::ul(items.into_iter().map(html::li))),
    ))
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
        .route("/", cabin::page(app))
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
