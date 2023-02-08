use std::net::SocketAddr;

use rustend::previous::previous;
use rustend::{html, rustend_scripts, rustend_stylesheets, view, IntoView, View};

async fn app() -> impl View {
    view![rustend_stylesheets(), rustend_scripts(), root(true).await]
}

#[rustend::component]
async fn root(enabled: bool) -> impl View {
    async fn toggle(enabled: bool, _: ()) -> bool {
        !enabled
    }

    // Just a way to test a rerender without actually chaning anything
    async fn reset(enabled: bool, _: ()) -> bool {
        enabled
    }

    view![
        // TODO: toggle doesn't work
        html::div(if enabled {
            view![
                html::div![counter(0), " (state reset on parent rerender)"],
                html::div![
                    counter(previous(|c| c)),
                    " (state restored on parent rerender)"
                ],
            ]
            .boxed()
        } else {
            "...".boxed()
        }),
        view![
            html::button(if enabled {
                "remove counter"
            } else {
                "add counter"
            })
            .on_click(toggle, ()),
            html::button("force rerender").on_click(reset, ()),
        ]
    ]
}

#[rustend::component]
async fn counter(count: u32) -> impl View {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    html::button(html::text!("👍 {}", count)).on_click(incr, ())
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
