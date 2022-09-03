use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;

use rust_html_over_wire::action::EventAction;
use rust_html_over_wire::html::InputEvent;
use rust_html_over_wire::view::ViewHash;
use rust_html_over_wire::{html, render, Component, View, SERVER_COMPONENT_JS};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use solarsail::hyper::{header, StatusCode};
use solarsail::response::json;
use solarsail::route::{get, post};
use solarsail::{http, IntoResponse, Request, RequestExt, Response, SolarSail};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    let app = SolarSail::new((), handle_request);
    app.run(&addr).await.unwrap();
}

async fn handle_request(_state: (), mut req: Request) -> Response {
    match req.route().as_tuple() {
        get!("health") => "Ok".into_response(),

        get!("server-component.js") => http::Response::builder()
            .header(header::CONTENT_TYPE, "text/javascript")
            .body(SERVER_COMPONENT_JS.into())
            .unwrap(),

        get!() => {
            let view = input_component("".into());
            let html = render(view).unwrap();
            let html = format!(
                r#"<script src="/server-component.js" async></script>{}"#,
                html
            );
            html.into_response()
        }

        post!("dispatch" / component) => {
            // TODO: remove to_string()
            #[allow(clippy::unnecessary_to_owned)]
            let update = handle_component(&component.to_string(), &mut req).await;
            json(update).into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

// #[action::event]
fn handle_input(_state: Cow<'static, str>, ev: InputEvent) -> Cow<'static, str> {
    ev.value.into()
}

pub fn input(value: Cow<'static, str>) -> impl View<Cow<'static, str>> {
    (
        html::div().content(format!("Value: {}", value)),
        html::custom("input")
            .attr("value", value)
            .on_input(handle_input_action),
    )
}

// result of #[action]
#[allow(non_upper_case_globals)]
const handle_input_action: EventAction<Cow<'static, str>, InputEvent> =
    EventAction::new("input::handle_input", handle_input);

// result of #[component]
pub fn input_component(
    value: Cow<'static, str>,
) -> Component<Cow<'static, str>, impl View<Cow<'static, str>>> {
    Component::new("input::input", value, input)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Update {
    state: Box<RawValue>,
    view_hash: ViewHash,
    html: String,
}

fn handle_cow_str_input_action(
    state: Cow<'static, str>,
    action: &str,
    event: InputEvent,
) -> Cow<'static, str> {
    match action {
        "input::handle_input" => (handle_input_action.action)(state, event),
        _ => panic!("unknown u32 action with id: {}", action),
    }
}

#[allow(unused)]
async fn handle_component(id: &str, req: &mut Request) -> Update {
    match id {
        "input::input" => {
            #[derive(Deserialize)]
            struct Dispatch {
                state: Cow<'static, str>,
                action: String,
                event: InputEvent,
            }
            // TODO: unwrap
            let payload: Dispatch = req.body_mut().json().await.unwrap();
            let after = handle_cow_str_input_action(payload.state, &payload.action, payload.event);
            let state = serde_json::value::to_raw_value(&after).unwrap();
            let component = input_component(after);
            let (html, view_hash) = component.render_update().unwrap();
            Update {
                state,
                view_hash,
                html,
            }
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}
