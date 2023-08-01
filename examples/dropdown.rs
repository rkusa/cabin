use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::event;
use cabin::view::IteratorExt;
use http::Request;
use serde::{Deserialize, Serialize};

// FIXME: fix example to keep both states
async fn app() -> impl View {
    let count = event::<Increment>().unwrap_or(Increment(3)).0;

    (
        button(
            on_click(Increment(count + 1)).style("min-width:40px"),
            text!("{}", count),
        ),
        dialog(count),
    )
}

fn dialog(count: usize) -> impl View {
    let open = event::<Toggle>().unwrap_or_default();

    div(
        style("display:inline;position:relative"),
        (
            button(on_click(Toggle(!open.0)), "open"),
            open.0.then(|| {
                ul(
                    style(
                        "position:absolute;top:20px;right:0;background:#ddd;\
                         list-style-type:none;padding:4px;",
                    ),
                    (0..count)
                        .keyed(|item| *item)
                        .map(|item| li(style("white-space:nowrap;"), text!("Item {}", item))),
                )
            }),
        ),
    )
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
struct Toggle(bool);

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment(usize);

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
