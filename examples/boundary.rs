use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::view::boundary::Boundary;
use cabin::{Event, basic_document};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app(c: &Context) -> impl View<'_> {
    basic_document(
        c,
        c.fragment()
            .child(counter(c, 1))
            .child(counter(c, 2))
            .child(c.button().on_click(()).child("triger whole page update")),
    )
}

#[derive(Default, Clone, Copy, Event, Serialize, Deserialize)]
struct Increment(usize);

#[cabin::boundary(Increment)]
fn counter(c: &Context, count: usize) -> Boundary<'_, usize> {
    let count = c.event::<Increment>().unwrap_or(Increment(count)).0;

    c.button()
        .on_click(Increment(count + 1))
        .child(text!("{}", count))
        .boundary(count)
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
