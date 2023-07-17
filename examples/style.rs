use std::net::SocketAddr;

use cabin::prelude::*;
use cabin::state::State;
use cabin_tailwind::prelude::*;
use http::Request;

async fn app() -> impl View {
    let count = State::id(())
        .update(|count, _: ()| *count += 1)
        .restore_or(0);

    button(
        on_click(()).class(
            // TODO: modifier groups?
            // TODO: autocomplate after XZY. (for modifiers)
            // TODO: autocomplete after text::
            tw![
                BLOCK,
                text::BLACK,
                text::SM,
                bg::BLACK.hover(),
                text::WHITE.hover(),
                text::XS.hover().focus(),
            ]
            .append_when(count == 0, tw![text::color("red")]),
        ),
        text!("{}", count),
    )
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
