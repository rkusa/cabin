use std::borrow::Cow;
use std::net::SocketAddr;

use cabin::context::take_event;
use cabin::html::events::InputValue;
use cabin::prelude::*;
use cabin::{Event, basic_document};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    basic_document(search().await)
}

#[derive(Hash, Event, Serialize, Deserialize)]
struct Search(InputValue);

async fn search() -> impl View {
    let query: Cow<'static, str> = take_event::<Search>()
        .map(|e| e.0.take())
        .unwrap_or(Cow::Borrowed("Ge"));
    let items = search_countries(&query).await;

    h::div()
        .child(
            h::div().child(
                h::input()
                    .on_input(Search(InputValue::placeholder()))
                    .value(query),
            ),
        )
        .child(h::div().child(h::ul().child(items.into_iter().map(|i| h::li().child(i)))))
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

cabin::BOUNDARIES!();

#[tokio::main]
async fn main() {
    let filter =
        tracing_subscriber::filter::filter_fn(|metadata| metadata.target().starts_with("cabin"));
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::new().pretty())
        .with(filter)
        .init();

    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|req: Request<axum::body::Body>| cabin::put_page(req, app)),
        )
        .layer(cabin_service::redirects::layer())
        .layer(cabin_service::boundaries::layer(&BOUNDARIES))
        .layer(cabin_service::livereload::layer())
        .layer(cabin_service::assets::layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        server.into_make_service(),
    )
    .await
    .unwrap();
}
