use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait FromPrevious<T>: Send {
    fn next_from_previous(self, previous: Option<T>) -> T;
}

impl<T> FromPrevious<T> for T
where
    T: Serialize + DeserializeOwned + Send,
{
    fn next_from_previous(self, _previous: Option<T>) -> T {
        self
    }
}

mod internal {
    use std::marker::PhantomData;

    use super::*;

    pub struct PreviousFn<F, T> {
        f: F,
        marker: PhantomData<T>,
    }

    impl<F, T> PreviousFn<F, T> {
        pub fn new(f: F) -> Self {
            Self {
                f,
                marker: PhantomData,
            }
        }
    }

    impl<F, T> FromPrevious<T> for PreviousFn<F, T>
    where
        T: Default + Serialize + DeserializeOwned + Send,
        F: FnOnce(T) -> T + Send,
    {
        fn next_from_previous(self, previous: Option<T>) -> T {
            (self.f)(previous.unwrap_or_default())
        }
    }

    pub struct PreviousOr<T> {
        initial: T,
    }

    impl<T> PreviousOr<T> {
        pub fn new(initial: T) -> Self {
            Self { initial }
        }
    }

    impl<T> FromPrevious<T> for PreviousOr<T>
    where
        T: Serialize + DeserializeOwned + Send,
    {
        fn next_from_previous(self, previous: Option<T>) -> T {
            previous.unwrap_or(self.initial)
        }
    }
}

pub fn previous<T>(f: impl FnOnce(T) -> T + Send) -> impl FromPrevious<T>
where
    T: Default + Serialize + DeserializeOwned + Send,
{
    internal::PreviousFn::new(f)
}

pub fn previous_or<T>(initial: T) -> impl FromPrevious<T>
where
    T: Serialize + DeserializeOwned + Send,
{
    internal::PreviousOr::new(initial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_new_value_by_default() {
        assert_eq!(42u32.next_from_previous(Some(1)), 42);
    }

    #[test]
    fn test_previous() {
        assert_eq!(previous::<u32>(|n| n + 1).next_from_previous(None), 1); // starts at default
        assert_eq!(previous::<u32>(|n| n + 1).next_from_previous(Some(42)), 43);
    }
}
