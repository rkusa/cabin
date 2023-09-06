use std::net::SocketAddr;

use cabin::basic_document;
use cabin::prelude::*;
use cabin::scope::event;
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    basic_document(counter(0).await)
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Increment(usize);

async fn counter(start_at: usize) -> impl View {
    let count = event::<Increment>().unwrap_or(Increment(start_at)).0;

    (
        h::div(h::text!("Count: {}", count)),
        h::button("inc").on_click(Increment(count + 1)),
    )
}

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
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
