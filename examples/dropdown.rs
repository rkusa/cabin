use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::scope::event;
use cabin::view::{Boundary, IteratorExt};
use cabin::{basic_document, Event};
use http::Request;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

async fn app() -> impl View {
    let count = event::<Increment>().unwrap_or(Increment(3)).0;

    basic_document((
        // Incrementing the counter will cause the dialog to change outside of its boundary, which
        // causes its internal state to revert to its default (closed). This is intentional.
        h::button(h::text!("{}", count))
            .on_click(Increment(count + 1))
            .style("min-width:40px"),
        dialog(count, false),
    ))
}

#[cabin::boundary]
fn dialog(count: usize, open: bool) -> Boundary<(usize, bool)> {
    let open = event::<Toggle>().unwrap_or(Toggle(open)).0;

    h::div((
        h::button("open").on_click(Toggle(!open)),
        open.then(|| {
            h::ul(
                (0..count)
                    .keyed(|item| *item)
                    .map(|item| h::li(h::text!("Item {}", item)).style("white-space:nowrap;")),
            )
            .style(
                "position:absolute;top:20px;right:0;background:#ddd;list-style-type:none;padding:\
                 4px;",
            )
        }),
    ))
    .style("display:inline;position:relative")
    .boundary((count, open))
}

#[derive(Default, Clone, Copy, Event, Serialize, Deserialize)]
struct Toggle(bool);

#[derive(Clone, Copy, Event, Serialize, Deserialize)]
struct Increment(usize);

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
        .layer(cabin_service::boundaries::layer())
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
