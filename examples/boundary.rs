use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::event;
use cabin::view::boundary::Boundary;
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (
        counter(1),
        counter(2),
        button(on_click(()), "triger whole page update"),
    )
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Increment(usize);

#[cabin::boundary]
fn counter(count: usize) -> Boundary<usize> {
    let count = event::<Increment>().unwrap_or(Increment(count)).0;

    boundary(
        count,
        button(on_click(Increment(count + 1)), text!("{}", count)),
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
