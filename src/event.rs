use crate::Exposed;

/// An event that can be sent from the client, usually derived via `#[derive(Event)]`.
///
/// Since the client controls the payload, events must implement [`Exposed`]. Its `validate` runs
/// before the event is handled. This only works for derived events: payloads of events that
/// implement `Event` manually are not validated.
///
/// `#[derive(Event)]` also derives `Exposed` (and accepts its `#[exposed(..)]` attributes). To
/// implement `Exposed` manually instead, add `#[event(manual_exposed)]`.
pub trait Event: Exposed {
    // TODO: enforce restrictions on compile time?
    /// Must not contain a comma (',').
    const ID: &'static str;
}

pub const fn event_id<T: Event>() -> &'static str {
    T::ID
}

impl Event for () {
    const ID: &'static str = "()";
}

impl Event for usize {
    const ID: &'static str = "usize";
}

impl Event for String {
    const ID: &'static str = "str";
}

impl Event for &'static str {
    const ID: &'static str = "str";
}

impl Event for bool {
    const ID: &'static str = "bool";
}
