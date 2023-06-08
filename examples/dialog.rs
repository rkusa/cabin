use std::net::SocketAddr;

use axum::Json;
use cabin::signal::Signal;
use cabin::{event, html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    // Dialog::new("Hello World").opened(true)
    dialog("Hello World")
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum DialogEvent {
    Open,
    Close,
}

fn dialog(content: impl View) -> impl View {
    let mut open = Signal::restore_or("dialog", false);
    match event() {
        Some(DialogEvent::Open) => *open = true,
        Some(DialogEvent::Close) => *open = false,
        None => {}
    };

    (
        open.then_some(
            html::dialog((content, html::button("close").on_click(DialogEvent::Close))).open(*open),
        ),
        html::button("open").on_click(DialogEvent::Open),
    )
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|Json(event): Json<cabin::Event>| cabin::put_page(event, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
