use std::borrow::Cow;
use std::net::SocketAddr;

use axum::Json;
use cabin::html::attributes::default;
use cabin::html::events::InputValue;
use cabin::state::State;
use cabin::{html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    search().await
}

#[derive(Hash, Serialize, Deserialize)]
struct Search(InputValue);

async fn search() -> impl View {
    let query = State::id("query")
        .update_take(|query, Search(v): Search| {
            *query = v.into();
        })
        .restore_or(Cow::Borrowed("Ge"));

    let items = search_countries(&query).await;

    html::div(
        (),
        (
            html::div(
                (),
                html::input(default().on_input(|ev| Search(ev.value)).value(query)),
            ),
            html::div(
                (),
                html::ul((), items.into_iter().map(|item| html::li((), item))),
            ),
        ),
    )
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
            axum::routing::get(|| cabin::get_page(app))
                .put(|Json(event): Json<cabin::Event>| cabin::put_page(event, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
