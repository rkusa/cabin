pub mod registry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Action<S> {
    pub module: &'static str,
    pub name: &'static str,
    pub action: fn(S) -> S,
}

impl<S> Action<S> {
    pub const fn new(module: &'static str, name: &'static str, action: fn(S) -> S) -> Self {
        Self {
            module,
            name,
            action,
        }
    }
}

#[derive(Clone, Copy)]
pub struct EventAction<S, E> {
    pub module: &'static str,
    pub name: &'static str,
    pub action: fn(S, E) -> S,
}

impl<S, E> EventAction<S, E> {
    pub const fn new(module: &'static str, name: &'static str, action: fn(S, E) -> S) -> Self {
        Self {
            module,
            name,
            action,
        }
    }
}
