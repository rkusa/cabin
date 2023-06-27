use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::hash::Hasher;

use once_cell::race::OnceBox;
use twox_hash::XxHash32;

use super::Style;

#[linkme::distributed_slice]
pub static STYLES: [fn(&mut StyleRegistry)] = [..];

static REGISTRY: OnceBox<StyleRegistry> = OnceBox::new();

pub struct StyleRegistry {
    out: String,
    hashes: HashSet<u32>,
}

impl StyleRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                out: Default::default(),
                hashes: Default::default(),
            };

            registry.out.push_str(include_str!("./base.css"));

            #[cfg(feature = "preflight")]
            registry
                .out
                .push_str(include_str!("./preflight/preflight-v3.2.4.css"));

            #[cfg(feature = "forms")]
            registry
                .out
                .push_str(include_str!("./forms/forms-v0.5.3.css"));

            for f in STYLES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn add(&mut self, styles: &[&dyn Style]) -> String {
        let mut sorted = styles
            .iter()
            .map(|s| (hash_style(*s), *s))
            .collect::<Vec<_>>();
        sorted.sort_by_key(|(hash, _)| *hash);

        let grouped = sorted.into_iter().map(|(_, s)| s).fold(
            HashMap::<_, Vec<_>>::new(),
            |mut grouped, style| {
                let mut hasher = DefaultHasher::new();
                style.hash_modifier(&mut hasher);
                let hash = hasher.finish();
                grouped.entry(hash).or_default().push(style);
                grouped
            },
        );

        // As everything is written to a string, all unwraps below are fine.
        let mut all_names = String::with_capacity(8);
        for styles in grouped.into_values() {
            let pos = self.out.len();
            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.get(0) {
                style.selector_prefix(&mut self.out).unwrap();
            }
            let class_name_offset = self.out.len();
            write!(&mut self.out, "          ").unwrap();
            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.get(0) {
                style.selector_suffix(&mut self.out).unwrap();
            }
            writeln!(&mut self.out, " {{").unwrap();
            for style in &styles {
                style.declarations(&mut self.out).unwrap();
            }
            write!(&mut self.out, "}}").unwrap();
            if let Some(style) = styles.get(0) {
                style.suffix(&mut self.out).unwrap();
            }
            writeln!(&mut self.out).unwrap();

            let mut hasher = XxHash32::default();
            hasher.write(self.out[pos..].as_bytes());
            let hash = hasher.finish() as u32;

            // write actual class name, prepend `_` as it class names must not start with a number
            let name = styles
                .get(0)
                .and_then(|s| s.override_class_name().map(Cow::Borrowed))
                .unwrap_or_else(|| Cow::Owned(format!("_{hash:x}")));

            if !self.hashes.insert(hash) {
                // already known, remove just written stuff from output
                self.out.truncate(pos);
            } else {
                let offset = class_name_offset + 9 - name.len();
                self.out.replace_range(offset..offset + 1, ".");
                self.out
                    .replace_range(offset + 1..offset + 1 + name.len(), &name);
            }

            if !all_names.is_empty() {
                all_names.push(' ');
            }
            all_names.push_str(&name);
        }

        all_names
    }

    pub fn style_sheet(&self) -> &str {
        &self.out
    }
}

fn hash_style(style: &dyn Style) -> u64 {
    struct HashWriter(DefaultHasher);

    impl fmt::Write for HashWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write(s.as_bytes());
            Ok(())
        }
    }

    let mut writer = HashWriter(DefaultHasher::default());
    style.declarations(&mut writer).ok();
    style.hash_modifier(&mut writer.0);
    writer.0.finish()
}

#[test]
fn test_deduplication() {
    // Generate same class name if styles are the same just in a different order.

    let mut r = StyleRegistry {
        out: Default::default(),
        hashes: Default::default(),
    };
    let a = r.add(&[&super::BLOCK, &super::p(4)]);
    let b = r.add(&[&super::p(4), &super::BLOCK]);
    assert_eq!(a, b);
    assert_eq!(r.out, "._fa8ecfbb {\ndisplay: block;\npadding: 1rem;\n}\n");
}
