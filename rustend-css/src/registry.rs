use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::hash::Hasher;

use once_cell::race::OnceBox;

use super::Style;

#[linkme::distributed_slice]
pub static STYLES: [fn(&mut StyleRegistry)] = [..];

static REGISTRY: OnceBox<StyleRegistry> = OnceBox::new();

pub struct StyleRegistry {
    out: String,
}

impl StyleRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                out: Default::default(),
            };

            #[cfg(feature = "preflight")]
            registry
                .out
                .write_str(include_str!("./preflight/preflight-v3.2.4.css"))
                .unwrap();

            #[cfg(feature = "forms")]
            registry
                .out
                .write_str(include_str!("./forms/forms-v0.5.3.css"))
                .unwrap();

            for f in STYLES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn add(&mut self, name: &str, styles: &[&dyn Style]) {
        write!(self.out, "{}", StyleWritter { name, styles }).unwrap();
    }

    pub fn style_sheet(&self) -> &str {
        &self.out
    }
}

struct StyleWritter<'s> {
    name: &'s str,
    styles: &'s [&'s dyn Style],
}

impl<'s> fmt::Display for StyleWritter<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grouped = self
            .styles
            .iter()
            .fold(HashMap::<_, Vec<_>>::new(), |mut grouped, style| {
                let mut hasher = DefaultHasher::new();
                style.hash_modifier(&mut hasher);
                let hash = hasher.finish();
                grouped.entry(hash).or_default().push(style);
                grouped
            });

        for styles in grouped.into_values() {
            write!(f, ".{}", self.name).unwrap();
            for style in &styles {
                style.selector_prefix(f)?;
            }
            writeln!(f, " {{").unwrap();
            for style in &styles {
                style.declarations(f)?;
            }
            writeln!(f, "}}").unwrap();
        }

        Ok(())
    }
}
