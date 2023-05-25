use std::convert::Infallible;
use std::net::SocketAddr;

use axum::body::{Full, HttpBody};
use axum::response::Response;
use rustend::previous::previous;
use rustend::{html, rustend_scripts, rustend_stylesheets, view, View};
use serde::{Deserialize, Serialize};

async fn app() -> impl View {
    view![
        rustend_stylesheets(),
        rustend_scripts(),
        level1(Entry::default()).await
    ]
}

#[derive(Debug, Default, Hash, Serialize, Deserialize)]
struct Entry {
    count: u32,
    has_child: bool,
}

#[rustend::component]
async fn level1(state: Entry) -> Result<impl View, Infallible> {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    async fn toggle_child(mut state: Entry, _: ()) -> Entry {
        state.has_child = !state.has_child;
        state
    }

    Ok(html::fieldset![
        html::button(html::text!("{}", state.count)).on_click(incr, ()),
        html::button("toggle child").on_click(toggle_child, ()),
        state.has_child.then(|| level2(previous(2, |e| e)))
    ])
}

#[rustend::component]
async fn level2(state: Entry) -> Result<impl View, Infallible> {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    async fn toggle_child(mut state: Entry, _: ()) -> Entry {
        state.has_child = !state.has_child;
        state
    }

    Ok(html::fieldset![
        html::button(html::text!("{}", state.count)).on_click(incr, ()),
        html::button("toggle child").on_click(toggle_child, ()),
        state.has_child.then(|| level3(previous(3, |e| e)))
    ])
}

#[rustend::component]
async fn level3(state: Entry) -> Result<impl View, Infallible> {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    async fn toggle_child(mut state: Entry, _: ()) -> Entry {
        state.has_child = !state.has_child;
        state
    }

    Ok(html::fieldset![
        html::button(html::text!("{}", state.count)).on_click(incr, ()),
        html::button("toggle child").on_click(toggle_child, ()),
        state.has_child.then(|| level4(previous(4, |e| e)))
    ])
}

#[rustend::component]
async fn level4(state: Entry) -> Result<impl View, Infallible> {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    Ok(html::fieldset![html::button(html::text!(
        "{}",
        state.count
    ))
    .on_click(incr, ())])
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
