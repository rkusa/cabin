use serde::Deserialize;

#[derive(Default, Deserialize)]
#[non_exhaustive]
pub struct InputEvent {
    pub value: String,
}
