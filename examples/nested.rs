use std::net::SocketAddr;

use axum::Json;
use cabin::html::attributes::default;
use cabin::html::Common;
use cabin::state::State;
use cabin::{html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    level(1)
}

fn level(n: usize) -> impl View {
    let count = State::id(("count", n))
        .update(|count, Increment(l): Increment| {
            if l == n {
                *count += 1;
            }
        })
        .restore_or(n);
    let has_next_level = State::<bool>::id(("has_next_level", n))
        .update(|has_next_level, ToggleChild(l): ToggleChild| {
            if l == n {
                *has_next_level = !*has_next_level;
            }
        })
        .restore_or(n < 3);

    html::fieldset(
        (),
        (
            html::button(default().on_click(Increment(n)), html::text!("{}", count)),
            html::button(default().on_click(ToggleChild(n)), "toggle child"),
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
