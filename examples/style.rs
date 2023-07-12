use std::net::SocketAddr;

use axum::Json;
use cabin::html::Common;
use cabin::state::State;
use cabin::{html, View};
use cabin_css::{self as css, css, Style};

async fn app() -> impl View {
    let count = State::id(())
        .update(|count, _: ()| *count += 1)
        .restore_or(0);

    html::button(
        html::attributes::default().on_click(()).class(
            // TODO: modifier groups?
            // TODO: autocomplate after XZY. (for modifiers)
            // TODO: autocomplete after text::
            css![
                css::BLOCK,
                css::text::BLACK,
                css::text::SM,
                css::bg::BLACK.hover(),
                css::text::WHITE.hover(),
                css::text::XS.hover().focus(),
            ]
            .append_when(count == 0, css![css::text::color("red")]),
        ),
        html::text!("{}", count),
    )
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| cabin::get_page(app))
                .put(|Json(event): Json<cabin::Event>| cabin::put_page(event, app)),
        )
        .layer(cabin_service::framework());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
