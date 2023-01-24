use std::borrow::Cow;
use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::{html, view, View};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(app);
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}

async fn app() -> impl View {
    items(Items(vec![
        Item {
            id: 1,
            name: "first".into(),
        },
        Item {
            id: 2,
            name: "second".into(),
        },
    ]))
    .await
}

#[derive(Serialize, Deserialize)]
struct Item {
    id: usize,
    name: Cow<'static, str>,
}

#[derive(Default, Serialize, Deserialize)]
struct Items(Vec<Item>);

#[rustend::component]
async fn items(items: Items) -> impl View {
    async fn add(mut items: Items, _: ()) -> Items {
        let max_id = items.0.iter().map(|i| i.id).max().unwrap_or(0);
        items.0.push(Item {
            id: max_id + 1,
            name: "new item 1".into(),
        });
        items.0.push(Item {
            id: max_id + 2,
            name: "new item 2".into(),
        });
        items
    }

    // TODO: concurrent deletes race each other
    async fn delete(mut items: Items, id: usize) -> Items {
        items.0.retain(|i| i.id != id);
        items
    }

    view![
        html::ul(
            items
                .0
                .into_iter()
                .map(|item| html::li![item.name, html::button("x").on_click(delete, item.id)])
        ),
        html::div(html::button("add").on_click(add, ())),
    ]
}
