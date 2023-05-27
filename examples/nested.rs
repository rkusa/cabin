#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]
use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::component::{Component, PublicComponent};
use rustend::{html, rustend_scripts, rustend_stylesheets, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (
        rustend_stylesheets(),
        rustend_scripts(),
        Level::restore_or_else(1, || Level::new(1)),
    )
}

#[derive(Debug, Default, Hash, Serialize, Deserialize, PublicComponent)]
struct Level {
    level: u32,
    count: u32,
    has_child: bool,
}

impl Level {
    fn new(level: u32) -> Self {
        Self {
            level,
            count: level,
            has_child: level < 4,
        }
    }
}

#[derive(Serialize, Deserialize)]
enum LevelEvent {
    Increment,
    ToggleChild,
}

impl Component for Level {
    type Event = LevelEvent;
    type Error = Infallible;

    async fn update(&mut self, event: Self::Event) {
        match event {
            LevelEvent::Increment => self.count += 1,
            LevelEvent::ToggleChild => self.has_child = !self.has_child,
        }
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok(html::fieldset((
            html::button(html::text!("{}", self.count)).on_click(LevelEvent::Increment),
            html::button("toggle child").on_click(LevelEvent::ToggleChild),
            self.has_child.then(|| {
                let next_level = self.level + 1;
                Level::restore_or_else(next_level, || Level::new(next_level)).boxed()
            }),
        )))
    }
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = rustend::render_to_response(app).await;
                let (parts, body) = res.into_parts();
                Response::from_parts(parts, Full::new(body).boxed())
            }),
        )
        .layer(rustend_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
