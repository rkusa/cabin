use std::fmt;

pub trait PropertyDisplay {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> PropertyDisplay for Option<T>
where
    T: PropertyDisplay,
{
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = self {
            p.fmt_property(name, f)?;
        }
        Ok(())
    }
}

impl PropertyDisplay for &'static str {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{name}: {self};")
    }
}

impl PropertyDisplay for bool {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{name}: {};", if *self { 1 } else { 0 })
    }
}

impl PropertyDisplay for u16 {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{name}: {};", self)
    }
}

impl PropertyDisplay for i32 {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{name}: {};", self)
    }
}
