use std::hash::Hasher;

use serde::Serialize;
use twox_hash::XxHash32;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum ViewHash {
    Leaf(u32),
    Node(u32, Box<[ViewHash]>),
}

impl ViewHash {
    pub fn hash(&self) -> u32 {
        match self {
            ViewHash::Leaf(hash) => *hash,
            ViewHash::Node(hash, child_hashes) => {
                if *hash > 0 {
                    *hash
                } else {
                    let mut hasher = XxHash32::default();
                    for h in child_hashes.iter() {
                        hasher.write_u32(h.hash());
                    }
                    hasher.finish() as u32
                }
            }
        }
    }

    pub fn into_parent(self, parent_hash: u32) -> Self {
        match self {
            ViewHash::Leaf(_) => ViewHash::Node(parent_hash, vec![self].into_boxed_slice()),
            ViewHash::Node(h, c) => {
                if h == 0 {
                    ViewHash::Node(parent_hash, c)
                } else {
                    ViewHash::Node(parent_hash, vec![ViewHash::Node(h, c)].into_boxed_slice())
                }
            }
        }
    }
}
