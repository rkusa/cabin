#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::component::{Component, PublicComponent};
use rustend::view::IteratorExt;
use rustend::{html, rustend_scripts, rustend_stylesheets, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    (
        rustend_stylesheets(),
        rustend_scripts(),
        Items::restore_or_else((), || Items(vec![Item { id: 1 }, Item { id: 2 }])),
    )
}

#[derive(Hash, Serialize, Deserialize)]
struct Item {
    id: usize,
}

#[derive(Default, Hash, Serialize, Deserialize, PublicComponent)]
struct Items(Vec<Item>);

#[derive(Serialize, Deserialize)]
enum ItemsEvent {
    AddAbove,
    AddBelow,
    Delete(usize),
}

impl Component for Items {
    type Event = ItemsEvent;
    type Error = Infallible;

    async fn update(&mut self, event: Self::Event) {
        let max_id = || self.0.iter().map(|i| i.id).max().unwrap_or(0);

        match event {
            ItemsEvent::AddAbove => {
                self.0.insert(0, Item { id: max_id() + 1 });
            }
            ItemsEvent::AddBelow => {
                self.0.push(Item { id: max_id() + 1 });
            }
            ItemsEvent::Delete(id) => {
                self.0.retain(|i| i.id != id);
            }
        }
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok((
            html::div(html::button("add above").on_click(ItemsEvent::AddAbove)),
            html::ul(
                self.0
                    .into_iter()
                    .enumerate()
                    .map(|(i, item)| {
                        html::li((
                            Counter::restore_or(item.id, Counter(i + 1)),
                            html::button("x").on_click(ItemsEvent::Delete(item.id)),
                        ))
                    })
                    .into_view(),
            ),
            html::div(html::button("add below").on_click(ItemsEvent::AddBelow)),
        ))
    }
}

#[derive(Hash, Serialize, Deserialize, PublicComponent)]
struct Counter(usize);
impl Component for Counter {
    type Event = ();
    type Error = Infallible;

    async fn update(&mut self, _event: Self::Event) {
        self.0 += 1;
    }

    async fn view(self) -> Result<impl View<Self::Event>, Self::Error> {
        Ok(html::button(html::text!("{}", self.0)).on_click(()))
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
