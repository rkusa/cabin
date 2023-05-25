use serde::{Deserialize, Serialize};

use crate::component::id::NanoId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewHashTree(pub(crate) Vec<Marker>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Marker {
    Start,
    End(u32),
    Component(NanoId),
}

impl Serialize for Marker {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Marker::Start => s.serialize_i32(-1),
            Marker::End(hash) => s.serialize_u32(*hash),
            Marker::Component(id) => id.serialize(s),
        }
    }
}

mod deserialize {
    use std::fmt;
    use std::str::FromStr;

    use serde::de::{self, Visitor};
    use serde::Deserialize;

    use super::Marker;
    use crate::component::id::NanoId;

    struct MarkerVisitor;

    impl<'de> Visitor<'de> for MarkerVisitor {
        type Value = Marker;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(r#"an i64"#)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_i32(v as i32)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                -1 => Ok(Marker::Start),
                _ => Ok(Marker::End(v.try_into().map_err(de::Error::custom)?)),
            }
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match v {
                -1 => Ok(Marker::Start),
                _ => Ok(Marker::End(v.try_into().map_err(de::Error::custom)?)),
            }
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_u32(v as u32)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Marker::End(v))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Marker::End(v.try_into().map_err(de::Error::custom)?))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Marker::Component(
                NanoId::from_str(v).map_err(de::Error::custom)?,
            ))
        }
    }

    impl<'de> Deserialize<'de> for Marker {
        fn deserialize<D>(d: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            d.deserialize_any(MarkerVisitor)
        }
    }
}

#[cfg(test)]
impl From<Vec<Marker>> for ViewHashTree {
    fn from(v: Vec<Marker>) -> Self {
        Self(v)
    }
}

#[test]
fn test_serde() {
    use std::str::FromStr;

    let hash_tree = vec![
        Marker::Start,
        Marker::End(1),
        Marker::Start,
        Marker::Start,
        Marker::Component(NanoId::from_str("1234567890abcdefghijk").unwrap()),
        Marker::End(2),
        Marker::Start,
        Marker::End(3),
        Marker::End(4),
        Marker::End(5),
    ];
    let serialized = serde_json::to_string(&hash_tree).unwrap();
    assert_eq!(
        serialized,
        r#"[-1,1,-1,-1,"1234567890abcdefghijk",2,-1,3,4,5]"#
    );
    let deserialized: Vec<Marker> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, hash_tree);
}
