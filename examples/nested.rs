use std::net::SocketAddr;

use cabin::prelude::*;
use http::Request;
use serde::{Deserialize, Serialize};

// FIXME: restore example
async fn app() -> impl View {
    level(1)
}

fn level(n: usize) -> impl View {
    let count = 2;
    // State::id(("count", n))
    // .update(|count, Increment(l): Increment| {
    //     if l == n {
    //         *count += 1;
    //     }
    // })
    // .restore_or(n);
    let has_next_level = false;
    // State::<bool>::id(("has_next_level", n))
    // .update(|has_next_level, ToggleChild(l): ToggleChild| {
    //     if l == n {
    //         *has_next_level = !*has_next_level;
    //     }
    // })
    // .restore_or(n < 3);

    fieldset(
        (),
        (
            button(on_click(Increment(n)), text!("{}", count)),
            button(on_click(ToggleChild(n)), "toggle child"),
            has_next_level.then(|| level(n + 1).boxed()),
        ),
    )
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment(usize);

#[derive(Clone, Copy, Serialize, Deserialize)]
struct ToggleChild(usize);

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
