use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::future::Future;

use crate::Error;

/// A type that can be used as a boundary argument or event.
///
/// Boundary arguments are sent to the client and back, and event payloads come from the client, so
/// a client can send arbitrary values for them (e.g. via `PUT /__boundary/{id}`). Once they are
/// deserialized from such a request, [`Exposed::validate`] is called before the boundary or page
/// runs. Use it to reject values the current user must not access (e.g. by returning a `403`). The
/// returned error's status code is kept.
///
/// `validate` is not called when a boundary is rendered as part of a page, only when its
/// arguments come from the client.
///
/// `#[derive(Exposed)]` validates each field. Use `#[exposed(skip)]` on fields whose type doesn't
/// implement `Exposed`. Recursive types need their `validate` future to be boxed. The derive does
/// that automatically if a field's type refers to the type itself (e.g. `Vec<Self>`). For types
/// that are recursive through another type (`A` contains `B` contains `A`), add
/// `#[exposed(boxed)]` to one of them; without it, compilation fails with a cycle error.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is exposed to the client and must implement `cabin::Exposed`",
    label = "`{Self}` does not implement `cabin::Exposed`",
    note = "boundary arguments and events come from the client and must implement `cabin::Exposed`",
    note = "use `#[derive(Exposed)]` or implement it manually to validate client-supplied values"
)]
pub trait Exposed: Sync {
    /// Validates a value deserialized from client-supplied boundary state.
    fn validate(&self) -> impl Future<Output = Result<(), Error>> + Send {
        async { Ok(()) }
    }
}

macro_rules! impl_noop {
    ($($ty:ty),* $(,)?) => {
        $(impl Exposed for $ty {})*
    };
}

impl_noop!(
    (),
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    String,
    &'static str,
    Cow<'static, str>,
    crate::html::events::InputValue,
    crate::html::events::InputChecked,
);

#[cfg(feature = "time")]
impl_noop!(
    time::Date,
    time::Time,
    time::PrimitiveDateTime,
    time::OffsetDateTime,
    time::UtcOffset,
    time::Duration,
);

#[cfg(feature = "uuid")]
impl_noop!(uuid::Uuid);

impl<T: Exposed> Exposed for Option<T> {
    async fn validate(&self) -> Result<(), Error> {
        match self {
            Some(value) => value.validate().await,
            None => Ok(()),
        }
    }
}

impl<T: Exposed> Exposed for Box<T> {
    async fn validate(&self) -> Result<(), Error> {
        T::validate(self).await
    }
}

macro_rules! impl_seq {
    ($($ty:ident),*) => {
        $(
            impl<T: Exposed> Exposed for $ty<T> {
                async fn validate(&self) -> Result<(), Error> {
                    for value in self {
                        value.validate().await?;
                    }
                    Ok(())
                }
            }
        )*
    };
}

impl_seq!(Vec, VecDeque, BTreeSet);

impl<T: Exposed, S: Sync> Exposed for HashSet<T, S> {
    async fn validate(&self) -> Result<(), Error> {
        for value in self {
            value.validate().await?;
        }
        Ok(())
    }
}

impl<T: Exposed, const N: usize> Exposed for [T; N] {
    async fn validate(&self) -> Result<(), Error> {
        for value in self {
            value.validate().await?;
        }
        Ok(())
    }
}

impl<K: Exposed, V: Exposed> Exposed for BTreeMap<K, V> {
    async fn validate(&self) -> Result<(), Error> {
        for (key, value) in self {
            key.validate().await?;
            value.validate().await?;
        }
        Ok(())
    }
}

impl<K: Exposed, V: Exposed, S: Sync> Exposed for HashMap<K, V, S> {
    async fn validate(&self) -> Result<(), Error> {
        for (key, value) in self {
            key.validate().await?;
            value.validate().await?;
        }
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($name:ident)+) => {
        impl<$($name: Exposed),+> Exposed for ($($name,)+) {
            async fn validate(&self) -> Result<(), Error> {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                $($name.validate().await?;)+
                Ok(())
            }
        }
    };
}

impl_tuple!(A);
impl_tuple!(A B);
impl_tuple!(A B C);
impl_tuple!(A B C D);
impl_tuple!(A B C D E);
impl_tuple!(A B C D E F);
impl_tuple!(A B C D E F G);
impl_tuple!(A B C D E F G H);
impl_tuple!(A B C D E F G H I);
impl_tuple!(A B C D E F G H I J);
impl_tuple!(A B C D E F G H I J K);
impl_tuple!(A B C D E F G H I J K L);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn assert_exposed<T: Exposed>() {}

    #[test]
    #[cfg(feature = "time")]
    fn time() {
        assert_exposed::<time::Date>();
        assert_exposed::<time::Time>();
        assert_exposed::<time::PrimitiveDateTime>();
        assert_exposed::<time::OffsetDateTime>();
        assert_exposed::<time::UtcOffset>();
        assert_exposed::<time::Duration>();
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn uuid() {
        assert_exposed::<uuid::Uuid>();
    }
}
