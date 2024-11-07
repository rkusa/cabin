use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::take_event;
use cabin::{basic_document, Event};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    let data = take_event::<Data>();

    let values = data.clone().unwrap_or_default();
    basic_document((
        h::form((
            h::input().type_text().name("comment").value(values.comment),
            h::input()
                .type_checkbox()
                .name("highlighted")
                .with_checked(values.highlighted),
            h::button("submit").type_submit(),
        ))
        .on_submit::<Data>(),
        data.map(|data| {
            h::text!(
                "Submitted: comment={}; highlighted={}",
                data.comment,
                data.highlighted
            )
        }),
    ))
}

#[derive(Default, Clone, Event, Serialize, Deserialize)]
struct Data {
    comment: String,
    #[serde(default, deserialize_with = "cabin::serde::de::checkbox")]
    highlighted: bool,
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
