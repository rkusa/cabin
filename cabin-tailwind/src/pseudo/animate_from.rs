use std::fmt;

use crate::Utility;

pub struct AnimateFrom<S>(pub S);

impl<S: Utility> Utility for AnimateFrom<S> {
    fn declarations(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn write_animate_from(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        self.0.declarations(f)
    }

    fn order(&self) -> usize {
        self.0.order()
    }
}
