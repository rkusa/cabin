pub mod de {
    use std::fmt;

    use serde::de::{self, Visitor};

    pub fn checkbox<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(CheckboxVisitor)
    }

    struct CheckboxVisitor;

    impl Visitor<'_> for CheckboxVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a bool or `on` string")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v == "on" {
                Ok(true)
            } else {
                Err(de::Error::invalid_type(de::Unexpected::Str(v), &self))
            }
        }
    }
}
