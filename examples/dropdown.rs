use std::borrow::Cow;
use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::previous::previous;
use rustend::view::fragment;
use rustend::{html, rustend_scripts, rustend_stylesheets, IntoView, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    fragment() >> rustend_stylesheets() >> rustend_scripts() >> root(2).await
}

#[rustend::component]
async fn root(count: u32) -> Result<impl View, Infallible> {
    async fn incr(count: u32, _: ()) -> u32 {
        count + 1
    }

    Ok(fragment()
        >> {
            html::button()
                .on_click(incr, ())
                .attr("style", "min-width:40px")
                >> html::text!("{}", count)
        }
        >> dropdown(previous((), move |s: DropdownState| {
            s.with_items((0..count).map(|i| format!("Item {i}").into()).collect())
        }))
        .await)
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
async fn dropdown(state: DropdownState) -> Result<impl View, Infallible> {
    async fn toggle(mut state: DropdownState, _: ()) -> DropdownState {
        state.opened = !state.opened;
        state
    }

    Ok(
        html::div().attr("style", "display:inline;position:relative")
            >> { html::button().on_click(toggle, ()) >> "open" }
            >> if state.opened {
                (html::ul().attr(
                    "style",
                    "position:absolute;top:20px;right:0;\
                    background:#ddd;list-style-type:none;padding:4px;",
                ) >> state
                    .items
                    .into_iter()
                    .map(|item| html::li().attr("style", "white-space:nowrap;") >> item)
                    .into_view())
                .boxed()
            } else {
                ().boxed()
            },
    )
}

#[tokio::main]
async fn main() {
    let server = axum::Router::new()
        .route(
            "/",
            axum::routing::get(|| async {
                let res = rustend::render_to_response(app().await).await;
                let (parts, body) = res.into_parts();
                Response::from_parts(parts, Full::new(body).boxed())
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
