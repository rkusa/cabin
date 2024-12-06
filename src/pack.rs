use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub enum Bound<T: Pack> {
    Packed(T::Packed),
    Unpacked(T),
}

impl<T: Pack> Bound<T> {
    pub async fn unpack(self) -> Result<T, T::Error> {
        match self {
            Bound::Packed(packed) => T::unpack(packed).await,
            Bound::Unpacked(unpacked) => Ok(unpacked),
        }
    }
}

pub trait Pack: Sized {
    type Packed: Serialize + DeserializeOwned;
    type Error;

    fn pack(&self) -> Self::Packed;
    #[allow(async_fn_in_trait)]
    async fn unpack(pack: Self::Packed) -> Result<Self, Self::Error>;

    fn packed(&self) -> Bound<Self> {
        Bound::Packed(self.pack())
    }

    fn unpacked(self) -> Bound<Self> {
        Bound::Unpacked(self)
    }
}

impl<T: Pack> Serialize for Bound<T> {
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

impl<'de, T: Pack> Deserialize<'de> for Bound<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Bound::Packed(T::Packed::deserialize(deserializer)?))
    }
}
