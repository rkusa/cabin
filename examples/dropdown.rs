use std::borrow::Cow;
use std::net::SocketAddr;

use rustend::previous::previous;
use rustend::{html, view, IntoView, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    root(2).await
}

#[rustend::component]
async fn root(count: u32) -> impl View {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    view![
        html::button(html::text!("{}", count))
            .on_click(incr, ())
            .attr("style", "min-width:40px"),
        dropdown(previous(move |s: DropdownState| s.with_items(
            (0..count).map(|i| format!("Item {i}").into()).collect()
        )))
    ]
}

#[derive(Debug, Default, Hash, Serialize, Deserialize)]
struct DropdownState {
    items: Vec<Cow<'static, str>>,
    opened: bool,
}

impl DropdownState {
    fn with_items(mut self, items: Vec<Cow<'static, str>>) -> Self {
        self.items = items;
        self
    }
}

#[rustend::component]
async fn dropdown(state: DropdownState) -> impl View {
    async fn toggle(mut state: DropdownState, _: ()) -> DropdownState {
        state.opened = !state.opened;
        state
    }

    html::div![
        html::button("open").on_click(toggle, ()),
        if state.opened {
            html::ul(state.items.into_iter().map(|item| html::li(item).attr("style", "white-space:nowrap;")))
                .attr(
                    "style",
                    "position:absolute;top:20px;right:0;background:#ddd;list-style-type:none;padding:4px;",
                )
                .boxed()
        } else {
            ().boxed()
        },
    ]
    .attr("style", "display:inline;position:relative")
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
