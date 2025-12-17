use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher as _};

use smallvec::SmallVec;
use twox_hash::XxHash32;

use crate::html::attributes::Attributes;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClassName(u32);

impl ClassName {
    pub(crate) fn new(style: &impl Hash) -> Self {
        let mut s = XxHash32::default();
        style.hash(&mut s);
        let hash = s.finish();
        Self(hash as u32)
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_{:_<8x}", self.0)
    }
}

#[derive(Hash)]
pub struct ClassNames {
    pub class_names: SmallVec<ClassName, 4>,
    pub other: Cow<'static, str>,
}

impl Attributes for ClassNames {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        r.attribute("class", self);
        Ok(())
    }
}

impl fmt::Display for ClassNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut add_space = false;
        if !self.other.is_empty() {
            write!(f, "{}", self.other)?;
            add_space = true;
        }
        for class_name in &self.class_names {
            if add_space {
                f.write_str(" ")?;
            }
            write!(f, "{class_name}")?;
            add_space = true;
        }
        Ok(())
    }
}
