#[derive(Clone, Copy)]
pub struct Action<S> {
    pub id: &'static str,
    pub action: fn(S) -> S,
}

impl<S> Action<S> {
    pub const fn new(id: &'static str, action: fn(S) -> S) -> Self {
        Self { id, action }
    }
}

#[derive(Clone, Copy)]
pub struct EventAction<S, E> {
    pub id: &'static str,
    pub action: fn(S, E) -> S,
}

impl<S, E> EventAction<S, E> {
    pub const fn new(id: &'static str, action: fn(S, E) -> S) -> Self {
        Self { id, action }
    }
}
