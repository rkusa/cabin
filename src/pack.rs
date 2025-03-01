use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub enum Bound<T: Pack<C>, C> {
    Packed(T::Packed),
    Unpacked(T),
}

impl<T: Pack<C>, C> Bound<T, C> {
    pub async fn unpack(self, conn: &C) -> Result<T, T::Error> {
        match self {
            Bound::Packed(packed) => T::unpack(packed, conn).await,
            Bound::Unpacked(unpacked) => Ok(unpacked),
        }
    }
}

pub trait Pack<C>: Sized {
    type Packed: Serialize + DeserializeOwned;
    type Error;

    fn pack(&self) -> Self::Packed;
    fn unpack(pack: Self::Packed, conn: &C) -> impl Future<Output = Result<Self, Self::Error>>;

    fn packed(&self) -> Bound<Self, C> {
        Bound::Packed(self.pack())
    }

    fn unpacked(self) -> Bound<Self, C> {
        Bound::Unpacked(self)
    }
}

impl<T: Pack<C>, C> Serialize for Bound<T, C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Bound::Packed(packed) => packed.serialize(serializer),
            Bound::Unpacked(unpacked) => unpacked.pack().serialize(serializer),
        }
    }
}

impl<'de, T: Pack<C>, C> Deserialize<'de> for Bound<T, C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Bound::Packed(T::Packed::deserialize(deserializer)?))
    }
}

impl<T, C> std::fmt::Debug for Bound<T, C>
where
    T: Pack<C> + std::fmt::Debug,
    T::Packed: std::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> std::fmt::Result {
        match self {
            Bound::Packed(packed) => f.debug_tuple("Packed").field(packed).finish(),
            Bound::Unpacked(unpacked) => f.debug_tuple("Unpacked").field(unpacked).finish(),
        }
    }
}

impl<T, C> Clone for Bound<T, C>
where
    T: Pack<C> + Clone,
    T::Packed: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Bound::Packed(packed) => Bound::Packed(packed.clone()),
            Bound::Unpacked(unpacked) => Bound::Unpacked(unpacked.clone()),
        }
    }
}
