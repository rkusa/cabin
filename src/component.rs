pub mod registry;

use std::fmt;
use std::hash::{Hash, Hasher};

pub use rustend_macros::PublicComponent;
use serde::de::DeserializeOwned;
use serde::Serialize;
use twox_hash::XxHash32;

use crate::restore::Restored;
use crate::view::View;

pub trait PublicComponent {
    fn id() -> ComponentId;
}

pub trait Component: PublicComponent {
    type Event: Serialize + DeserializeOwned;
    type Error: Into<crate::Error>;
    async fn update(&mut self, event: Self::Event);
    async fn view(self) -> Result<impl View<Self::Event>, Self::Error>;

    fn restore(id: impl Hash) -> Restored<Self>
    where
        Self: Sized + Default + DeserializeOwned,
    {
        let id = {
            let mut hasher = XxHash32::default();
            Self::id().hash(&mut hasher);
            id.hash(&mut hasher);
            hasher.finish() as u32
        };
        Restored::restore(id).unwrap_or_else(|| Restored::new(id, Default::default()))
    }

    fn restore_or(id: impl Hash, or: Self) -> Restored<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let id = {
            let mut hasher = XxHash32::default();
            Self::id().hash(&mut hasher);
            id.hash(&mut hasher);
            hasher.finish() as u32
        };
        Restored::restore(id).unwrap_or_else(|| Restored::new(id, or))
    }

    fn restore_or_else(id: impl Hash, or: impl FnOnce() -> Self) -> Restored<Self>
    where
        Self: Sized + DeserializeOwned,
    {
        let id = {
            let mut hasher = XxHash32::default();
            Self::id().hash(&mut hasher);
            id.hash(&mut hasher);
            hasher.finish() as u32
        };
        Restored::restore(id).unwrap_or_else(|| Restored::new(id, (or)()))
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ComponentId {
    module: &'static str,
    name: &'static str,
}

impl ComponentId {
    pub const fn new(module: &'static str, name: &'static str) -> Self {
        Self { module, name }
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: somehow do only once?
        f.write_str(if self.module.starts_with("r#") {
            &self.module[2..]
        } else {
            self.module
        })?;
        f.write_str("::")?;
        f.write_str(if self.name.starts_with("r#") {
            &self.name[2..]
        } else {
            self.name
        })?;
        Ok(())
    }
}
