use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::state::State;
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    counter(0).await
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment;

async fn counter(start_at: usize) -> impl View {
    let count = State::id(())
        .update::<Increment>(|count: &mut usize, _| *count += 1)
        .restore_or(start_at);

    (
        div((), text!("Count: {}", count)),
        button(on_click(Increment), "inc"),
    )
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|req: Request<axum::body::Body>| cabin::put_page(req, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
