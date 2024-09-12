use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::{self, Write};
use std::hash::Hasher;

use once_cell::race::OnceBox;
use twox_hash::XxHash32;

use super::Utility;

#[linkme::distributed_slice]
pub static STYLES: [(usize, fn(&mut StyleRegistry))] = [..];

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

            let mut styles = STYLES.to_vec();
            styles.sort_by_key(|(order, _)| *order);
            for (_, f) in styles {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn add(&mut self, styles: &[&dyn Utility]) -> String {
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
        let mut grouped = grouped
            .into_values()
            .map(|styles| (styles.iter().map(|s| s.order()).max().unwrap_or(0), styles))
            .collect::<Vec<_>>();
        grouped.sort_by_key(|(order, _)| *order);

        // As everything is written to a string, all unwraps below are fine.
        let mut all_names = String::with_capacity(8);
        for (_, mut styles) in grouped {
            styles.sort_by_key(|s| s.order());

            let pos = self.out.len();

            writeln!(&mut self.out, "@keyframes ").unwrap();
            let animation_name_offset1 = self.out.len();
            write!(&mut self.out, "          {{").unwrap();
            writeln!(&mut self.out, "  from {{").unwrap();
            let before_animate_from = self.out.len();
            for style in &styles {
                style.write_animate_from(&mut self.out).unwrap();
            }
            let has_animate_from = self.out.len() > before_animate_from;
            writeln!(&mut self.out, "  }}").unwrap();
            writeln!(&mut self.out, "  to {{").unwrap();
            let before_animate_to = self.out.len();
            for style in &styles {
                style.write_animate_to(&mut self.out).unwrap();
            }
            let has_animate_to = self.out.len() > before_animate_to;
            writeln!(&mut self.out, "  }}").unwrap();
            writeln!(&mut self.out, "}}").unwrap();

            let has_animation = has_animate_from || has_animate_to;
            if !has_animation {
                self.out.truncate(pos);
            }

            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.first() {
                style.selector_prefix(&mut self.out).unwrap();
            }
            let class_name_offset = self.out.len();
            write!(&mut self.out, "          ").unwrap();
            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.first() {
                style.selector_suffix(&mut self.out).unwrap();
            }
            writeln!(&mut self.out, " {{").unwrap();
            let mut animation_name_offset2 = 0;
            if has_animation {
                // TODO: make easing function, delay, duration, etc. customizable
                write!(&mut self.out, "animation: 250ms ease-in-out 1 forwards ").unwrap();
                animation_name_offset2 = self.out.len();
                writeln!(&mut self.out, "         ;").unwrap();
            }
            for style in &styles {
                style.declarations(&mut self.out).unwrap();
            }
            write!(&mut self.out, "}}").unwrap();
            if let Some(style) = styles.first() {
                style.suffix(&mut self.out).unwrap();
            }
            writeln!(&mut self.out).unwrap();

            let mut hasher = XxHash32::default();
            hasher.write(self.out[pos..].as_bytes());
            let hash = hasher.finish() as u32;

            // write actual class name, prepend `_` as it class names must not start with a number
            let name = styles
                .first()
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

                if has_animation {
                    let offset = animation_name_offset1 + 9 - name.len();
                    self.out.replace_range(offset..offset + name.len(), &name);
                    let offset = animation_name_offset2 + 9 - name.len();
                    self.out.replace_range(offset..offset + name.len(), &name);
                }
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

fn hash_style(style: &dyn Utility) -> u64 {
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

    use crate::utilities::{p, BLOCK};

    let mut r = StyleRegistry {
        out: Default::default(),
        hashes: Default::default(),
    };
    let a = r.add(&[&BLOCK, &p(4)]);
    let b = r.add(&[&p(4), &BLOCK]);
    assert_eq!(a, b);
    insta::assert_snapshot!(r.out);
}

#[test]
fn test_order() {
    // Test order of @media statements

    use super::Responsive;
    use crate::utilities::BLOCK;

    let mut r = StyleRegistry {
        out: Default::default(),
        hashes: Default::default(),
    };
    r.add(&[
        &BLOCK.sm().max_md(),
        &BLOCK.md(),
        &BLOCK.max_sm(),
        &BLOCK.max_md(),
        &BLOCK.sm(),
    ]);
    insta::assert_snapshot!(r.out);
}
