use std::net::SocketAddr;

use axum::Json;
use cabin::state::State;
use cabin::{event, html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    counter(0).await
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment;

// TODO: needs to be mapped to state
async fn counter(start_at: usize) -> impl View {
    let mut count = State::restore_or((), start_at);
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
