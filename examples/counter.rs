use std::net::SocketAddr;

use rustend::{html, view, IntoView, View};

async fn app() -> impl View {
    counter(0).await
}

#[rustend::component]
async fn counter(count: u32) -> impl View {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    view![
        //     (self.0 == 0).then(|| ),
        //     (self.0 > 0).then(move || html::div(html::text!("Count: {}", self.0))),
        if count > 0 {
            html::div(html::text!("Count: {}", count)).boxed()
        } else {
            html::div("Hit `incr` to start counting ...").boxed()
        },
        html::button("incr").on_click(incr, ()),
    ]
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
