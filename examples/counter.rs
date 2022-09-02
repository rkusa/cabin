use std::net::SocketAddr;
use std::str::FromStr;

use rust_html_over_wire::{html, render, Action, Component, View};
use serde::{Deserialize, Serialize};
use solarsail::hyper::StatusCode;
use solarsail::route::get;
use solarsail::{IntoResponse, Request, RequestExt, Response, SolarSail};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let app = SolarSail::new((), handle_request);
    app.run(&addr).await.unwrap();
}

async fn handle_request(_state: (), req: Request) -> Response {
    match req.route().as_tuple() {
        get!("health") => "Ok".into_response(),

        get!() => {
            let view = counter_component(0);
            let html = render(view).unwrap();
            html.into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Serialize, Deserialize)]
pub enum CountAction {
    Increment,
}

impl Action<u32> for CountAction {
    fn apply(self, state: u32) -> u32 {
        match self {
            CountAction::Increment => state + 1,
        }
    }
}

pub fn counter(count: u32) -> impl View<CountAction> {
    (
        html::div().content(format!("Count: {}", count)),
        html::button::<CountAction>()
            .on_click(CountAction::Increment)
            .content("incr"),
    )
}

// result of #[component]
pub fn counter_component(count: u32) -> impl View<()> {
    Component::new(count, counter)
}

#[allow(unused)]
fn handle_component(id: &str, state: &str, action: &str) {
    match id {
        "crate::counter" => {
            let before: u32 = serde_json::from_str(state).unwrap();
            let action: CountAction = serde_json::from_str(action).unwrap();
            let after = action.apply(before);
            let _component = counter(after);
            // TODO: rerender
            // let _ = component.render(out)
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}
