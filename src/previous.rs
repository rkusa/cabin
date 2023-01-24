pub trait Previous: serde::de::DeserializeOwned {
    fn previous() -> Self;
}

impl<D> Previous for D
where
    D: Default + serde::de::DeserializeOwned,
{
    fn previous() -> Self {
        Self::default()
    }
}
