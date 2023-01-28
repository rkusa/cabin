pub mod text;

use std::fmt;

pub use rustend_macros::css;

pub enum Length {
    Px(f32),
    Rem(f32),
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Px(v) => write!(f, "{v}px"),
            Length::Rem(v) => write!(f, "{v}rem"),
        }
    }
}

#[cfg(feature = "preflight")]
#[::linkme::distributed_slice(rustend::style::registry::BASE)]
fn __preflight(r: &mut rustend::style::registry::StyleRegistry) {
    use std::fmt::Write;
    r.write_str(include_str!("./preflight/preflight-v3.2.4.css"))
        .unwrap();
}

#[cfg(feature = "forms  ")]
#[::linkme::distributed_slice(rustend::style::registry::BASE)]
fn __forms(r: &mut rustend::style::registry::StyleRegistry) {
    use std::fmt::Write;
    r.write_str(include_str!("./forms/forms-v0.5.3.css"))
        .unwrap();
}
