use core::fmt::{self, Write};

use once_cell::race::OnceBox;

use super::Style;

#[linkme::distributed_slice]
pub static STYLES: [fn(&mut StyleRegistry)] = [..];

static REGISTRY: OnceBox<StyleRegistry> = OnceBox::new();

pub struct StyleRegistry {
    out: String,
}

pub struct DeclarationBlock<'a> {
    out: &'a mut String,
}

impl StyleRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                out: Default::default(),
            };
            for f in STYLES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn add(&mut self, name: &str, f: impl FnOnce(DeclarationBlock)) {
        writeln!(&mut self.out, ".{name} {{").unwrap();
        (f)(DeclarationBlock { out: &mut self.out });
        writeln!(&mut self.out, "}}").unwrap();
    }

    pub fn style_sheet(&self) -> &str {
        &self.out
    }
}

impl<'a> DeclarationBlock<'a> {
    pub fn append(&mut self, style: impl Style) {
        write!(self.out, "{}", StyleWritter(style)).unwrap();
    }
}

struct StyleWritter<S>(S);

impl<S: Style> fmt::Display for StyleWritter<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.css(f)
    }
}
