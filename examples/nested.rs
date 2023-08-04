use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::take_event;
use cabin::view::Boundary;
use http::Request;
use serde::{Deserialize, Serialize};

// FIXME: restore example
async fn app() -> impl View {
    level(1, 1, true)
}

#[cabin::boundary]
fn level(n: usize, count: usize, has_next_level: bool) -> Boundary<(usize, usize, bool)> {
    // Important to take the event here so that it is not available for the nested components
    // anymore
    let count = take_event::<Increment>()
        .map(|_| count + 1)
        .unwrap_or(count);
    let has_next_level = take_event::<ToggleChild>()
        .map(|_| !has_next_level)
        .unwrap_or(has_next_level);

    fieldset(
        (),
        (
            button(on_click(Increment), text!("{}", count)),
            button(on_click(ToggleChild), "toggle child"),
            has_next_level.then(|| level(n + 1, n + 1, n < 3).boxed()),
        ),
    )
    .boundary((n, count, has_next_level))
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment;

#[derive(Clone, Copy, Serialize, Deserialize)]
struct ToggleChild;

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
