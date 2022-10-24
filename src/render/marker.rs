use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewHashTree(pub(crate) Vec<Marker>);

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Marker {
    Start,
    End(u32),
    Component,
}

impl Serialize for Marker {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Marker::Start => s.serialize_str("s"),
            Marker::End(hash) => s.serialize_u32(*hash),
            Marker::Component => s.serialize_str("c"),
        }
    }
}

mod deserialize {
    use std::fmt;

    use serde::de::{self, Visitor};
    use serde::Deserialize;

    use super::Marker;

    struct MarkerVisitor;

    impl<'de> Visitor<'de> for MarkerVisitor {
        type Value = Marker;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str(r#"an u32 or "s""#)
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
            Ok(Marker::End(v.try_into().map_err(de::Error::custom)?))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Marker::End(v.try_into().map_err(de::Error::custom)?))
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
            match v {
                "s" => Ok(Marker::Start),
                "c" => Ok(Marker::Component),
                _ => Err(de::Error::invalid_type(de::Unexpected::Str(v), &self)),
            }
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
    let hash_tree = vec![
        Marker::Start,
        Marker::End(1),
        Marker::Start,
        Marker::Start,
        Marker::Component,
        Marker::End(2),
        Marker::Start,
        Marker::End(3),
        Marker::End(4),
        Marker::End(5),
    ];
    let serialized = serde_json::to_string(&hash_tree).unwrap();
    assert_eq!(serialized, r#"["s",1,"s","s","c",2,"s",3,4,5]"#);
    let deserialized: Vec<Marker> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, hash_tree);
}
