#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::net::SocketAddr;

use cabin::signal::Signal;
use cabin::{event, html, signal, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    counter(0).await
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment;

// TODO: needs to be mapped to signal
async fn counter(start_at: usize) -> impl View {
    let mut count = signal!(start_at);
    if let Some(Increment) = event() {
        *count += 1;
    }

    (
        html::div(html::text!("Count: {}", count)),
        html::button("inc").on_click(Increment), // TODO: how to tie Increment to this instance
    )
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
