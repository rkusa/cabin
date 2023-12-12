use std::net::SocketAddr;

use cabin::cabin_scripts;
use cabin::prelude::*;
use cabin::scope::event;
use cabin_tailwind::cabin_stylesheets;
use cabin_tailwind::prelude::*;
use http::Request;
use tokio::net::TcpListener;

async fn app() -> impl View {
    let count = event::<usize>().unwrap_or(0);

    document(
        h::button(h::text!("{}", count)).on_click(count + 1).class(
            // TODO: modifier groups?
            // TODO: autocomplate after XZY. (for modifiers)
            // TODO: autocomplete after text::
            tw![
                tw::BLOCK,
                tw::text::BLACK,
                tw::text::SM,
                tw::bg::BLACK.hover(),
                tw::text::WHITE.hover(),
                tw::text::XS.hover().focus(),
            ]
            .append_when(count == 0, tw![tw::text::color("red")]),
        ),
    )
}

fn document(content: impl View) -> impl View {
    (
        h::doctype(),
        h::html((
            h::head((cabin_stylesheets(), cabin_scripts())),
            h::body(content),
        )),
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
    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        server.into_make_service(),
    )
    .await
    .unwrap();
}
