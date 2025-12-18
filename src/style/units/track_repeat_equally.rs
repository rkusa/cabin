use std::fmt;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TrackRepeatEqually(pub u16);

impl fmt::Display for TrackRepeatEqually {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "repeat({}, minmax(0, 1fr))", self.0)
    }
}
