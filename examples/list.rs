use std::borrow::Cow;
use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::{html, rustend_scripts, rustend_stylesheets, view, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    view![
        rustend_stylesheets(),
        rustend_scripts(),
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
    ]
}

#[derive(Hash, Serialize, Deserialize)]
struct Item {
    id: usize,
    name: Cow<'static, str>,
}

#[derive(Default, Hash, Serialize, Deserialize)]
struct Items(Vec<Item>);

#[rustend::component]
async fn items(items: Items) -> Result<impl View, Infallible> {
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

    Ok(view![
        html::ul(
            items
                .0
                .into_iter()
                .map(|item| html::li![item.name, html::button("x").on_click(delete, item.id)])
        ),
        html::div(html::button("add").on_click(add, ())),
    ])
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = rustend::render_to_response(app().await).await;
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
