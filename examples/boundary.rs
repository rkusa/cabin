use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::event;
use cabin::view::boundary::Boundary;
use cabin::{basic_document, Event};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    basic_document((
        counter(1),
        counter(2),
        h::button("triger whole page update").on_click(()),
    ))
}

#[derive(Default, Clone, Copy, Event, Serialize, Deserialize)]
struct Increment(usize);

#[cabin::boundary(Increment)]
fn counter(count: usize) -> Boundary<usize> {
    let count = event::<Increment>().unwrap_or(Increment(count)).0;

    h::button(h::text!("{}", count))
        .on_click(Increment(count + 1))
        .boundary(count)
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
