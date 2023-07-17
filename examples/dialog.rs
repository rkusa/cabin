use std::net::SocketAddr;

use cabin::html;
use cabin::prelude::*;
use cabin::state::State;
use http::Request;
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
    let open = State::id("dialog")
        .update(|open, event: DialogEvent| match event {
            DialogEvent::Open => *open = true,
            DialogEvent::Close => *open = false,
        })
        .restore_or(false);

    (
        open.then_some(html::dialog(
            dialog::with_open(open),
            (content, button(on_click(DialogEvent::Close), "close")),
        )),
        button(on_click(DialogEvent::Open), "open"),
    )
}

#[tokio::main]
async fn main() {
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
