use std::net::SocketAddr;

use rustend::{html, View};
use rustend_css::{self as css, css, Style};

async fn app() -> impl View {
    counter(0).await
}

#[rustend::component]
async fn counter(count: u32) -> impl View {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    html::button(html::text!("{count}"))
        .on_click(incr, ())
        .class(
            // TODO: modifier groups?
            // TODO: autocomplate after XZY. (for modifiers)
            // TODO: autocomplete after text::
            css!(
                css::BLOCK,
                css::text::BLACK,
                css::text::SM,
                css::bg::BLACK.hover(),
                css::text::WHITE.hover(),
                css::text::XS.hover().focus(),
            ) + (count == 0).then_some(css!(css::text::color("red"))),
        )
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                axum::response::Html(rustend::render(app().await).await.unwrap())
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
