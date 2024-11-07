use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::take_event;
use cabin::view::{Boundary, IteratorExt};
use cabin::{basic_document, Event};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    basic_document(list(vec![Item { id: 1, count: 1 }, Item { id: 2, count: 2 }]).await)
}

#[derive(Clone, Hash, Serialize, Deserialize)]
struct Item {
    id: usize,
    count: usize,
}

#[derive(Clone, Copy, Event, Serialize, Deserialize)]
enum ItemsEvent {
    AddAbove,
    AddBelow,
    Delete(usize),
    Increment(usize),
}

#[cabin::boundary]
async fn list(mut items: Vec<Item>) -> Boundary<Vec<Item>> {
    if let Some(event) = take_event::<ItemsEvent>() {
        match event {
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
        }
    }

    (
        h::div(h::button("add above").on_click(ItemsEvent::AddAbove)),
        h::ul(items.clone().into_iter().keyed(|item| item.id).map(|item| {
            h::li((
                h::button(h::text!("{}", item.count)).on_click(ItemsEvent::Increment(item.id)),
                h::button("x").on_click(ItemsEvent::Delete(item.id)),
            ))
        })),
        h::div(h::button("add below").on_click(ItemsEvent::AddBelow)),
    )
        .boundary(items.clone())
}

#[tokio::main]
async fn main() {
    let filter =
        tracing_subscriber::filter::filter_fn(|metadata| metadata.target().starts_with("cabin"));
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::new().pretty())
        .with(filter)
        .init();

    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|req: Request<axum::body::Body>| cabin::put_page(req, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        server.into_make_service(),
    )
    .await
    .unwrap();
}
