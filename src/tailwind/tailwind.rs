use smallvec::SmallVec;

use crate::prelude::Utility;
use crate::tailwind::registry::StyleRegistry;

#[derive(Default, Clone)]
pub struct Tailwind(SmallVec<(usize, &'static [Box<dyn Utility>]), 1>);

impl Tailwind {
    pub fn new(order: usize, utilities: &'static [Box<dyn Utility>]) -> Self {
        let mut tw = Self(SmallVec::new());
        tw.0.push((order, utilities));
        tw
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn append(mut self, mut other: Self) -> Self {
        self.0.append(&mut other.0);
        self
    }

    pub fn append_when(self, condition: bool, other: Self) -> Self {
        if !condition { self } else { self.append(other) }
    }

    pub fn append_to(self, r: &mut StyleRegistry) -> String {
        // FIXME: avoid allocations?
        self.0
            .into_iter()
            .map(|(order, styles)| r.add(order, styles))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
