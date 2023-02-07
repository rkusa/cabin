use std::net::SocketAddr;

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
async fn level1(state: Entry) -> impl View {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    async fn toggle_child(mut state: Entry, _: ()) -> Entry {
        state.has_child = !state.has_child;
        state
    }

    html::fieldset![
        html::button(html::text!("{}", state.count)).on_click(incr, ()),
        html::button("toggle child").on_click(toggle_child, ()),
        state.has_child.then(|| level2(previous(|e| e)))
    ]
}

#[rustend::component]
async fn level2(state: Entry) -> impl View {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    async fn toggle_child(mut state: Entry, _: ()) -> Entry {
        state.has_child = !state.has_child;
        state
    }

    html::fieldset![
        html::button(html::text!("{}", state.count)).on_click(incr, ()),
        html::button("toggle child").on_click(toggle_child, ()),
        state.has_child.then(|| level3(previous(|e| e)))
    ]
}

#[rustend::component]
async fn level3(state: Entry) -> impl View {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    async fn toggle_child(mut state: Entry, _: ()) -> Entry {
        state.has_child = !state.has_child;
        state
    }

    html::fieldset![
        html::button(html::text!("{}", state.count)).on_click(incr, ()),
        html::button("toggle child").on_click(toggle_child, ()),
        state.has_child.then(|| level4(previous(|e| e)))
    ]
}

#[rustend::component]
async fn level4(state: Entry) -> impl View {
    async fn incr(mut state: Entry, _: ()) -> Entry {
        state.count += 1;
        state
    }

    html::fieldset![html::button(html::text!("{}", state.count)).on_click(incr, ())]
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
