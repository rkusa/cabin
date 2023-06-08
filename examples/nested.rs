use std::net::SocketAddr;

use axum::Json;
use cabin::signal::Signal;
use cabin::{event, html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    level(1)
}

fn level(n: usize) -> impl View {
    let mut count = Signal::restore_or(("count", n), n);
    let mut has_next_level = Signal::restore_or(("has_next_level", n), n < 3);

    match event::<LevelEvent>() {
        Some(LevelEvent::Increment(l)) if l == n => *count += 1,
        Some(LevelEvent::ToggleChild(l)) if l == n => *has_next_level = !*has_next_level,
        _ => {}
    }

    html::fieldset((
        html::button(html::text!("{}", count)).on_click(LevelEvent::Increment(n)),
        html::button("toggle child").on_click(LevelEvent::ToggleChild(n)),
        has_next_level.then(|| level(n + 1).boxed()),
    ))
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum LevelEvent {
    Increment(usize),
    ToggleChild(usize),
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
