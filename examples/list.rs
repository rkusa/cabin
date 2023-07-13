use std::net::SocketAddr;

use axum::Json;
use cabin::prelude::*;
use cabin::state::State;
use cabin::view::IteratorExt;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    list(|| vec![Item { id: 1, count: 1 }, Item { id: 2, count: 2 }]).await
}

#[derive(Hash, Serialize, Deserialize)]
struct Item {
    id: usize,
    count: usize,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
enum ItemsEvent {
    AddAbove,
    AddBelow,
    Delete(usize),
    Increment(usize),
}

async fn list(initial: impl FnOnce() -> Vec<Item>) -> impl View {
    let items = State::<Vec<Item>>::id("list")
        .update(|items, event: ItemsEvent| match event {
            ItemsEvent::AddAbove => {
                let id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
                let count = items.len() + 1;
                items.insert(0, Item { id, count });
            }
            ItemsEvent::AddBelow => {
                let id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
                let count = items.len() + 1;
                items.push(Item { id, count });
            }
            ItemsEvent::Delete(id) => {
                items.retain(|i| i.id != id);
            }
            ItemsEvent::Increment(id) => {
                if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                    item.count += 1;
                }
            }
        })
        .restore_or_else(initial);

    (
        div((), button(on_click(ItemsEvent::AddAbove), "add above")),
        ul(
            (),
            items.into_iter().keyed(|item| item.id).map(|item| {
                li(
                    (),
                    (
                        button(
                            on_click(ItemsEvent::Increment(item.id)),
                            text!("{}", item.count),
                        ),
                        button(on_click(ItemsEvent::Delete(item.id)), "x"),
                    ),
                )
            }),
        ),
        div((), button(on_click(ItemsEvent::AddBelow), "add below")),
    )
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
