pub trait Event {
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
