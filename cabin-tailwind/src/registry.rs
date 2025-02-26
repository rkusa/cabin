use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::fmt::{self, Write};
use std::hash::Hasher;

use bytes::Bytes;
use cabin::View;
use twox_hash::XxHash32;

use super::Utility;

type Order = (usize, usize, u32);

pub struct StyleRegistry {
    classes: HashMap<String, Order>,
}

impl StyleRegistry {
    pub fn with(mut self, styles: &[fn(&mut StyleRegistry)]) -> Self {
        for f in styles {
            (f)(&mut self);
        }
        self
    }

    pub fn add(&mut self, major_order: usize, styles: &[&dyn Utility]) -> String {
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
        for (_, mut styles) in grouped {
            styles.sort_by_key(|s| s.order());

            let mut out = String::with_capacity(256);
            let pos = out.len();

            writeln!(&mut out, "@keyframes ").unwrap();
            let animation_name_offset1 = out.len();
            write!(&mut out, "          {{").unwrap();
            writeln!(&mut out, "  from {{").unwrap();
            let before_animate_from = out.len();
            for style in &styles {
                style.write_animate_from(&mut out).unwrap();
            }
            let has_animate_from = out.len() > before_animate_from;
            writeln!(&mut out, "  }}").unwrap();
            writeln!(&mut out, "  to {{").unwrap();
            let before_animate_to = out.len();
            for style in &styles {
                style.write_animate_to(&mut out).unwrap();
            }
            let has_animate_to = out.len() > before_animate_to;
            writeln!(&mut out, "  }}").unwrap();
            writeln!(&mut out, "}}").unwrap();

            let has_animation = has_animate_from || has_animate_to;
            if !has_animation {
                out.truncate(pos);
            }

            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.first() {
                style.selector_prefix(&mut out).unwrap();
            }
            let class_name_offset = out.len();
            write!(&mut out, "          ").unwrap();
            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.first() {
                style.selector_suffix(&mut out).unwrap();
            }
            writeln!(&mut out, " {{").unwrap();
            let mut animation_name_offset2 = 0;
            if has_animation {
                // TODO: make easing function, delay, duration, etc. customizable
                write!(&mut out, "animation: 250ms ease-in-out 1 forwards ").unwrap();
                animation_name_offset2 = out.len();
                writeln!(&mut out, "         ;").unwrap();
            }

            // already grouped by variants, so just writing it once (from the first), is enough
            if let Some(style) = styles.first() {
                style.selector_declarations(&mut out).unwrap();
            }
            for style in &styles {
                style.declarations(&mut out).unwrap();
            }
            write!(&mut out, "}}").unwrap();
            if let Some(style) = styles.first() {
                style.suffix(&mut out).unwrap();
            }
            writeln!(&mut out).unwrap();

            let hash = XxHash32::oneshot(0, out[pos..].as_bytes());

            // write actual class name, prepend `_` as it class names must not start with a number
            let name = styles
                .first()
                .and_then(|s| s.override_class_name().map(Cow::Borrowed))
                .unwrap_or_else(|| Cow::Owned(format!("_{hash:x}")));

            let offset = class_name_offset + 9 - name.len();
            out.replace_range(offset..offset + 1, ".");
            out.replace_range(offset + 1..offset + 1 + name.len(), &name);

            if has_animation {
                let offset = animation_name_offset1 + 9 - name.len();
                out.replace_range(offset..offset + name.len(), &name);
                let offset = animation_name_offset2 + 9 - name.len();
                out.replace_range(offset..offset + name.len(), &name);
            }

            let minor_order = styles.iter().map(|s| s.order()).max().unwrap_or_default();
            self.classes.insert(out, (major_order, minor_order, hash));

            if !all_names.is_empty() {
                all_names.push(' ');
            }
            all_names.push_str(&name);
        }

        all_names
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn build(self, include_base: bool) -> StyleSheet {
        let mut style_sheet = self.classes.into_iter().collect::<Vec<_>>();
        style_sheet.sort_by_key(|(_, (o1, o2, h))| (*o1, *o2, *h));

        let other = [
            #[cfg(not(test))]
            include_str!("./base.css"),
            #[cfg(all(feature = "preflight", not(test)))]
            include_str!("./preflight/preflight-v3.2.4.css"),
            #[cfg(all(feature = "forms", not(test)))]
            include_str!("./forms/forms-v0.5.3.css"),
        ];

        let css: String = if include_base {
            other
                .into_iter()
                .map(Cow::Borrowed)
                .chain(style_sheet.into_iter().map(|(s, _)| Cow::Owned(s)))
                .collect()
        } else {
            style_sheet.into_iter().map(|(s, _)| s).collect()
        };
        let hash = cabin::content_hash(css.as_bytes());
        StyleSheet {
            content: Bytes::from(css),
            path: format!("/styles.css?{hash}"),
        }
    }
}

impl Default for StyleRegistry {
    fn default() -> Self {
        Self {
            classes: HashMap::with_capacity(256),
        }
    }
}

pub struct StyleSheet {
    pub content: Bytes,
    pub path: String,
}

impl StyleSheet {
    pub fn link(&'static self) -> impl View {
        use cabin::html;
        use cabin::html::Common;
        use html::elements::link::Link;

        html::link()
            .id("cabin-styles")
            .rel(html::elements::link::Rel::StyleSheet)
            .href(&self.path)
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

    use crate::utilities::{BLOCK, p};

    let mut r = StyleRegistry {
        classes: Default::default(),
    };
    let a = r.add(0, &[&BLOCK, &p(4)]);
    let b = r.add(0, &[&p(4), &BLOCK]);
    assert_eq!(a, b);
    insta::assert_snapshot!(std::str::from_utf8(&r.build(true).content).unwrap());
}

#[test]
fn test_order() {
    // Test order of @media statements

    use super::Responsive;
    use crate::utilities::BLOCK;

    let mut r = StyleRegistry {
        classes: Default::default(),
    };
    r.add(
        0,
        &[
            &BLOCK.sm().max_md(),
            &BLOCK.md(),
            &BLOCK.max_sm(),
            &BLOCK.max_md(),
            &BLOCK.sm(),
        ],
    );
    insta::assert_snapshot!(std::str::from_utf8(&r.build(true).content).unwrap());
}
