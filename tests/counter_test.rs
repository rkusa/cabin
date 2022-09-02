use rust_html_over_wire::{html, render, Action, Component, View};
use serde::{Deserialize, Serialize};

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

#[test]
fn test_counter() {
    let count = 42;
    let view = (counter_component(count), html::div());
    let html = render(view).unwrap();
    assert_eq!(html, "<div>Count: 42</div><button>incr</button><div/>");
}
