use std::net::SocketAddr;

use cabin::basic_document;
use cabin::html;
use cabin::prelude::*;
use cabin::scope::event;
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    basic_document(dialog("Hello World"))
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Toggle(bool);

fn dialog(content: impl View) -> impl View {
    let open = event::<Toggle>().unwrap_or_default();

    (
        open.0.then_some(
            html::dialog((content, button("close").on_click(Toggle(false)))).with_open(open.0),
        ),
        button("open").on_click(Toggle(true)),
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
