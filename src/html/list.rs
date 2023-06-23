use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub enum SpaceSeparated<T> {
    Single(T),
    List(HashSet<T>),
}

impl<T> From<T> for SpaceSeparated<T> {
    fn from(value: T) -> Self {
        Self::Single(value)
    }
}

impl<T: fmt::Display> fmt::Display for SpaceSeparated<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpaceSeparated::Single(v) => v.fmt(f)?,
            SpaceSeparated::List(list) => {
                for (i, v) in list.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" ")?;
                    }
                    v.fmt(f)?;
                }
            }
        }

        Ok(())
    }
}

impl<T: Hash> Hash for SpaceSeparated<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SpaceSeparated::Single(v) => v.hash(state),
            SpaceSeparated::List(list) => {
                for v in list {
                    v.hash(state);
                }
            }
        }
    }
}
