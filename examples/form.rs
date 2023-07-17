use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::state::State;
use http::Request;
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    let data: Option<Data> = State::id(())
        .update_take::<Data>(|data: &mut Option<Data>, new_data: Data| *data = Some(new_data))
        .restore_or_default();

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
