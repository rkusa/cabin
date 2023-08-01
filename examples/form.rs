use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::take_event;
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    let data = take_event::<Data>();

    let values = data.clone().unwrap_or_default();
    (
        form(
            form::on_submit::<Data>(),
            (
                input(input::type_text().name("comment").value(values.comment)),
                input(
                    input::type_checkbox()
                        .name("highlighted")
                        .with_checked(values.highlighted),
                ),
                button(button::type_submit(), "submit"),
            ),
        ),
        data.map(|data| {
            text!(
                "Submitted: comment={}; highlighted={}",
                data.comment,
                data.highlighted
            )
        }),
    )
}

#[derive(Default, Clone, Serialize, Deserialize)]
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
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
