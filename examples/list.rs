use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::take_event;
use cabin::view::{Boundary, IteratorExt};
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    list(vec![Item { id: 1, count: 1 }, Item { id: 2, count: 2 }]).await
}

#[derive(Clone, Hash, Serialize, Deserialize)]
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

    boundary(
        items.clone(),
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
        ),
    )
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
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
