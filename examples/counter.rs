use rust_html_over_wire::{html, Action, Component, View};
use serde::{Deserialize, Serialize};

fn main() {}

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
