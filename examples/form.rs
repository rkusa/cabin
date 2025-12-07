use std::net::SocketAddr;

use cabin::context::take_event;
use cabin::prelude::*;
use cabin::{Event, basic_document};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    let data = take_event::<Data>();

    let values = data.clone().unwrap_or_default();
    basic_document(
        h::fragment()
            .child(
                h::form()
                    .on_submit::<Data>()
                    .child(h::input().type_text().name("comment").value(values.comment))
                    .child(
                        h::input()
                            .type_checkbox()
                            .name("highlighted")
                            .with_checked(values.highlighted),
                    )
                    .child(h::button().type_submit().child("submit")),
            )
            .child(data.map(|data| {
                h::any(h::text!(
                    "Submitted: comment={}; highlighted={}",
                    data.comment,
                    data.highlighted
                ))
            })),
    )
}

#[derive(Default, Clone, Event, Serialize, Deserialize)]
struct Data {
    comment: String,
    #[serde(default, deserialize_with = "cabin::serde::de::checkbox")]
    highlighted: bool,
}

cabin::BOUNDARIES!();

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
        .layer(cabin_service::redirects::layer())
        .layer(cabin_service::boundaries::layer(&BOUNDARIES))
        .layer(cabin_service::livereload::layer())
        .layer(cabin_service::assets::layer());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        server.into_make_service(),
    )
    .await
    .unwrap();
}
