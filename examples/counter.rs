#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use cabin::component::{Component, PublicComponent};
use cabin::{cabin_scripts, cabin_stylesheets, html, View};

async fn app() -> impl View {
    (
        cabin_stylesheets(),
        cabin_scripts(),
        Counter::restore_or((), Counter(0)),
    )
}

#[derive(Default, Hash, serde::Serialize, serde::Deserialize, PublicComponent)]
struct Counter(pub u32);

#[derive(serde::Serialize, serde::Deserialize)]
enum CounterEvent {
    Increment,
}

impl Component for Counter {
    type Event = CounterEvent;
    type Error = Infallible;

    async fn update(&mut self, event: Self::Event) {
        match event {
            CounterEvent::Increment => self.0 += 1,
        }
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok((
            // (self.0 == 0).then(|| ),
            // (self.0 > 0).then(move || html::div(html::text!("Count: {}", self.0))),
            if self.0 > 0 {
                html::div(html::text!("Count: {}", self.0)).boxed()
            } else {
                html::div("Hit `incr` to start counting ...").boxed()
            },
            html::button("incr").on_click(CounterEvent::Increment),
        ))
    }
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
