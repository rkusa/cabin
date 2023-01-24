use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait FromPrevious<T>: Send {
    fn next_from_previous(self, previous: Option<T>) -> T;
}

impl<T> FromPrevious<T> for T
where
    T: Serialize + DeserializeOwned + Send,
{
    fn next_from_previous(self, previous: Option<T>) -> T {
        previous.unwrap_or(self)
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
}

pub fn previous<T>(f: impl FnOnce(T) -> T + Send) -> impl FromPrevious<T>
where
    T: Default + Serialize + DeserializeOwned + Send,
{
    internal::PreviousFn::new(f)
}
