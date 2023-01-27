use std::convert::Infallible;
use std::future::ready;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::make_service_fn;
use rustend::previous::previous;
use rustend::{html, view, View};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = rustend_service::app(app);
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let server = hyper::Server::bind(&addr)
        .serve(make_service_fn(|_| ready(Ok::<_, Infallible>(app.clone()))));
    println!("Listening on http://{addr}");
    server.await.unwrap();
}

async fn app() -> impl View {
    level1(Entry::default()).await
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
