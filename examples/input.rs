use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;

use rust_html_over_wire::html::InputValue;
use rust_html_over_wire::{html, render, Action, Component, View, SERVER_COMPONENT_JS};
use serde::{Deserialize, Serialize};
use solarsail::hyper::{header, StatusCode};
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
            let html = handle_component(&component.to_string(), &mut req).await;
            html.into_response()
        }

        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Serialize, Deserialize)]
pub enum InputAction {
    SetValue(InputValue),
}

impl Action<Cow<'static, str>> for InputAction {
    fn apply(self, _state: Cow<'static, str>) -> Cow<'static, str> {
        match self {
            InputAction::SetValue(new_value) => new_value.take(),
        }
    }
}

pub fn input(value: Cow<'static, str>) -> impl View<InputAction> {
    (
        html::div().content(format!("Value: {}", value)),
        html::custom("input")
            .attr("value", value)
            .on_input(|e| InputAction::SetValue(e.value)),
    )
}

// result of #[component]
pub fn input_component(value: Cow<'static, str>) -> impl View<()> {
    Component::new("input::input", value, input)
}

#[allow(unused)]
async fn handle_component(id: &str, req: &mut Request) -> String {
    match id {
        "input::input" => {
            #[derive(Deserialize)]
            struct Dispatch {
                state: Cow<'static, str>,
                action: InputAction,
            }
            // TODO: unwrap
            let payload: Dispatch = req.body_mut().json().await.unwrap();
            let after = payload.action.apply(payload.state);
            let component = input_component(after);
            let html = render(component).unwrap();
            html
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}