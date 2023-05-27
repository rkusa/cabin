#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    arbitrary_self_types
)]
#![allow(incomplete_features)]

use std::borrow::Cow;
use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::component::{Component, PublicComponent};
use rustend::view::IteratorExt;
use rustend::{html, rustend_scripts, rustend_stylesheets, Restored, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (rustend_stylesheets(), rustend_scripts(), Root::restore(()))
}

#[derive(Debug, Hash, Serialize, Deserialize, PublicComponent)]
struct Root {
    count: u32,
}

impl Default for Root {
    fn default() -> Self {
        Self { count: 3 }
    }
}

impl Component for Root {
    type Event = ();
    type Error = Infallible;

    async fn update(&mut self, _: Self::Event) {
        self.count += 1;
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok((
            html::button(html::text!("{}", self.count))
                .on_click(())
                .attr("style", "min-width:40px"),
            Dropdown::restore(()).with_items(
                (0..self.count)
                    .map(|i| format!("Item {i}").into())
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug, Default, Hash, Serialize, Deserialize, PublicComponent)]
struct Dropdown {
    items: Vec<Cow<'static, str>>,
    opened: bool,
}

impl Dropdown {
    fn with_items(self: Restored<Self>, items: Vec<Cow<'static, str>>) -> Restored<Self> {
        self.map(|dropdown| Dropdown { items, ..dropdown })
    }
}

impl Component for Dropdown {
    type Event = ();
    type Error = Infallible;

    async fn update(&mut self, _: Self::Event) {
        self.opened = !self.opened;
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok(html::div((
            html::button("open").on_click(()),
            if self.opened {
                html::ul(
                    self.items
                        .into_iter()
                        .map(|item| html::li(item).attr("style", "white-space:nowrap;"))
                        .into_view(),
                )
                .attr(
                    "style",
                    "position:absolute;top:20px;right:0;background:#ddd;\
                    list-style-type:none;padding:4px;",
                )
                .boxed()
            } else {
                ().boxed()
            },
        ))
        .attr("style", "display:inline;position:relative"))
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
