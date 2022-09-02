pub trait Action<S> {
    fn apply(self, state: S) -> S;
}
