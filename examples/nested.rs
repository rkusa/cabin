use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::take_event;
use cabin::view::Boundary;
use cabin::{Event, basic_document};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    basic_document(level(1, 1, true))
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

    h::fieldset((
        h::button(h::text!("{}", count)).on_click(Increment),
        h::button("toggle child").on_click(ToggleChild),
        has_next_level.then(|| level(n + 1, n + 1, n < 3).boxed()),
    ))
    .boundary((n, count, has_next_level))
}

#[derive(Clone, Copy, Event, Serialize, Deserialize)]
struct Increment;

#[derive(Clone, Copy, Event, Serialize, Deserialize)]
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
        .layer(cabin_service::redirects::layer())
        .layer(cabin_service::boundaries::layer())
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
