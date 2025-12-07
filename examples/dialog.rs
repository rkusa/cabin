use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::{Event, basic_document};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app(c: &Context) -> impl View {
    basic_document(c, dialog(c, "Hello World"))
}

#[derive(Default, Clone, Copy, Event, Serialize, Deserialize)]
struct Toggle(bool);

fn dialog(c: &Context, content: impl View) -> impl View {
    let open = c.event::<Toggle>().unwrap_or_default();

    c.fragment()
        .child(
            open.0.then_some(
                c.dialog()
                    .with_open(open.0)
                    .child(content)
                    .child(c.button().on_click(Toggle(false)).child("close")),
            ),
        )
        .child(c.button().on_click(Toggle(true)).child("open"))
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
