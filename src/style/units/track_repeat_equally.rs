use std::fmt;

pub struct TrackRepeatEqually(pub u16);

impl fmt::Display for TrackRepeatEqually {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "epeat({}, minmax(0, 1fr))", self.0)
    }
}
