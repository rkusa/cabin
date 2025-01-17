use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::event;
use cabin::{basic_document, html, Event};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    basic_document(dialog("Hello World"))
}

#[derive(Default, Clone, Copy, Event, Serialize, Deserialize)]
struct Toggle(bool);

fn dialog(content: impl View) -> impl View {
    let open = event::<Toggle>().unwrap_or_default();

    (
        open.0.then_some(
            html::dialog((content, h::button("close").on_click(Toggle(false)))).with_open(open.0),
        ),
        h::button("open").on_click(Toggle(true)),
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
