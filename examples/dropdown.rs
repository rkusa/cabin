use std::net::SocketAddr;

use axum::Json;
use cabin::state::State;
use cabin::view::IteratorExt;
use cabin::{html, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    let count = State::id("count")
        .update(|count: &mut usize, _: Increment| *count += 1)
        .restore_or(3);

    (
        html::button(html::text!("{}", count))
            .on_click(Increment)
            .attr("style", "min-width:40px"),
        dialog(count),
    )
}

fn dialog(count: usize) -> impl View {
    let opened = State::<bool>::id("dialog")
        .update(|opened, _: ToggleDropdown| *opened = !*opened)
        .restore_or(false);

    html::div((
        html::button("open").on_click(ToggleDropdown),
        opened.then(|| {
            html::ul((0..count).keyed(|item| *item).map(|item| {
                html::li(html::text!("Item {}", item)).attr("style", "white-space:nowrap;")
            }))
            .attr(
                "style",
                "position:absolute;top:20px;right:0;background:#ddd;\
                list-style-type:none;padding:4px;",
            )
        }),
    ))
    .attr("style", "display:inline;position:relative")
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct ToggleDropdown;

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Increment;

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
