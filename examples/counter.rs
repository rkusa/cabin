#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use cabin::signal::{Signal, SignalMut};
use cabin::{cabin_scripts, cabin_stylesheets, html, signal, View};
use http::Response;

async fn app() -> impl View {
    (cabin_stylesheets(), cabin_scripts(), counter(0).await)
}

// TODO: needs to be mapped to signal
#[cabin::component]
async fn counter(start_at: usize) -> impl View {
    let count = signal!(start_at);

    // macro todos:
    // TODO: check in-scope/type
    fn increment(mut count: SignalMut<u32>) {
        *count += 1;
    }

    (
        html::div(html::text!("Count: {}", count)),
        html::button("inc").on_click(increment),
        // html::button("inc").on_click(action!(|count: SignalMut<u32>| *count = *count + 1)),
    )
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = cabin::render_to_response(app).await;
                let (parts, body) = res.into_parts();
                Response::from_parts(parts, Full::new(body).boxed())
            }),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
