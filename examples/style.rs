#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use cabin::component::{Component, PublicComponent};
use cabin::{cabin_scripts, cabin_stylesheets, html, View};
use cabin_css::{self as css, css, Style};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (cabin_stylesheets(), cabin_scripts(), Counter::restore(()))
}

#[derive(Debug, Default, Hash, Serialize, Deserialize, PublicComponent)]
struct Counter(u32);

impl Component for Counter {
    type Event = ();
    type Error = Infallible;

    async fn update(&mut self, _: Self::Event) {
        self.0 += 1;
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok(html::button(html::text!("{}", self.0)).on_click(()).class(
            // TODO: modifier groups?
            // TODO: autocomplate after XZY. (for modifiers)
            // TODO: autocomplete after text::
            css!(
                css::BLOCK,
                css::text::BLACK,
                css::text::SM,
                css::bg::BLACK.hover(),
                css::text::WHITE.hover(),
                css::text::XS.hover().focus(),
            ) + (self.0 == 0).then_some(css!(css::text::color("red"))),
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
