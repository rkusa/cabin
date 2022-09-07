use std::hash::Hasher;

use serde::{Deserialize, Serialize};
use twox_hash::XxHash32;

pub struct HashTree {
    hasher: XxHash32,
    // Represent the tree as a flat structure to reduce allocations (as otherwise each level
    // would require a new allocation for its own vec).
    tree: Vec<Marker>,
    previous_tree: Option<Vec<Marker>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewHashTree(Vec<Marker>);

#[derive(Debug, PartialEq, Eq)]
enum Marker {
    Start,
    End(u32),
}

pub struct Node<'a> {
    parent_hasher: XxHash32,
    hash_tree: &'a mut HashTree,
}

impl Default for HashTree {
    fn default() -> Self {
        Self {
            hasher: XxHash32::default(),
            tree: Vec::with_capacity(32),
            previous_tree: None,
        }
    }
}

impl HashTree {
    pub fn from_previous_tree(previous_tree: ViewHashTree) -> Self {
        Self {
            previous_tree: Some(previous_tree.0),
            ..Default::default()
        }
    }

    pub fn hash(&self) -> u32 {
        self.hasher.finish() as u32
    }

    pub fn node(&mut self) -> Node<'_> {
        self.tree.push(Marker::Start);
        // When entering a node, set the hasher of the [HashTree] to a fresh hasher and store its
        // current hasher in the node. It is swapped back once the node gets dropped.
        let parent_hasher = std::mem::take(&mut self.hasher);
        Node {
            parent_hasher,
            hash_tree: self,
        }
    }

    pub fn finish(mut self) -> ViewHashTree {
        self.tree.push(Marker::End(self.hash()));
        ViewHashTree(self.tree)
    }

    pub fn changed_or_else<R>(&self, hash: u32, f: impl FnOnce() -> R) -> Option<R> {
        if let Some(Marker::End(previous)) = self
            .previous_tree
            .as_ref()
            .and_then(|t| t.get(self.tree.len() - 1))
        {
            if *previous == hash {
                return None;
            }
        }
        Some(f())
    }
}

impl<'a> Node<'a> {
    pub fn content(&mut self) -> &mut HashTree {
        self.hash_tree
    }

    pub fn end(self) -> u32 {
        self.hash_tree.hash()
    }
}

impl Hasher for HashTree {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes);
    }
}

impl<'a> Hasher for Node<'a> {
    fn finish(&self) -> u64 {
        self.hash_tree.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hash_tree.write(bytes);
    }
}

impl<'a> Drop for Node<'a> {
    fn drop(&mut self) {
        let hash = self.hash_tree.hash();
        self.hash_tree.tree.push(Marker::End(hash));
        self.parent_hasher.write_u32(hash);
        std::mem::swap(&mut self.hash_tree.hasher, &mut self.parent_hasher);
    }
}

impl Serialize for Marker {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Marker::Start => s.serialize_str("s"),
            Marker::End(hash) => s.serialize_u32(*hash),
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
            if v == "s" {
                Ok(Marker::Start)
            } else {
                Err(de::Error::invalid_type(de::Unexpected::Str(v), &self))
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
mod tests {
    use super::*;
    use crate::{html, Render, View};

    fn hash(tag: &str) -> u32 {
        let mut hasher = XxHash32::default();
        hasher.write(tag.as_bytes());
        hasher.finish() as u32
    }

    #[test]
    fn test_consistent_hash() {
        let mut hash_tree = HashTree::default();
        hash_tree.write(b"div");
        assert_eq!(hash_tree.hash(), 3201766860);
        assert_eq!(hash_tree.hash(), 3201766860);
    }

    #[test]
    fn test_hash_tree_tuple_flattened() {
        fn component() -> impl View {
            (html::div(()), (html::div(()), html::div(())))
        }
        let mut hash_tree = HashTree::default();
        let renderer = component().render(&mut hash_tree).unwrap();
        let mut out = String::new();
        renderer.render(&mut out, false).unwrap();
        assert_eq!(
            hash_tree.tree,
            vec![
                Marker::Start,
                Marker::End(hash("div")),
                Marker::Start,
                Marker::End(hash("div")),
                Marker::Start,
                Marker::End(hash("div"))
            ]
        );
    }

    #[test]
    fn test_hash_tree_nested() {
        fn component() -> impl View {
            (html::div(()), html::div(("foobar", html::div(()))))
        }
        let mut hash_tree = HashTree::default();
        let renderer = component().render(&mut hash_tree).unwrap();
        let mut out = String::new();
        renderer.render(&mut out, false).unwrap();
        let expected_parent_hash = {
            let mut hasher = XxHash32::default();
            hasher.write(b"div");
            hasher.write_u32(hash("foobar"));
            hasher.write_u32(hash("div"));
            hasher.finish() as u32
        };
        assert_eq!(
            hash_tree.tree,
            vec![
                Marker::Start,
                Marker::End(hash("div")),
                Marker::Start,
                Marker::Start,
                Marker::End(hash("foobar")),
                Marker::Start,
                Marker::End(hash("div")),
                Marker::End(expected_parent_hash)
            ]
        );
    }

    #[test]
    fn test_serde() {
        let hash_tree = vec![
            Marker::Start,
            Marker::End(1),
            Marker::Start,
            Marker::Start,
            Marker::End(2),
            Marker::Start,
            Marker::End(3),
            Marker::End(4),
            Marker::End(5),
        ];
        let serialized = serde_json::to_string(&hash_tree).unwrap();
        assert_eq!(serialized, r#"["s",1,"s","s",2,"s",3,4,5]"#);
        let deserialized: Vec<Marker> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, hash_tree);
    }
}
