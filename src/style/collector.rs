use smallvec::SmallVec;

use crate::style::modifier::StyleModifier;
use crate::style::{Style, StyleDefinition, SubStyle};

#[derive(Clone)]
pub struct StyleCollector {
    styles: SmallVec<Entry, 1>,
}

#[derive(Clone)]
struct Entry {
    style: StyleDefinition,
    parent_modifier: Option<StyleModifier>,
}

impl Default for StyleCollector {
    fn default() -> Self {
        Self {
            styles: smallvec::smallvec![Entry {
                style: Default::default(),
                parent_modifier: None
            }],
        }
    }
}

impl Style for StyleCollector {
    fn style_mut(&mut self) -> &mut StyleDefinition {
        if self.styles.is_empty() {
            let style = StyleDefinition::default();
            self.styles.push(Entry {
                style,
                parent_modifier: None,
            });
        }
        &mut self.styles.first_mut().unwrap().style
    }
}

impl SubStyle for StyleCollector {
    fn style_mut_for(&mut self, modifier: StyleModifier) -> &mut StyleDefinition {
        let ix = self
            .styles
            .iter()
            .position(|style| style.style.modifier == modifier);
        if let Some(ix) = ix {
            &mut self.styles[ix].style
        } else {
            let style = StyleDefinition::new(modifier);
            self.styles.push(Entry {
                style,
                parent_modifier: self.styles.first().map(|s| s.style.modifier.clone()),
            });
            &mut self.styles.last_mut().unwrap().style
        }
    }

    fn substyle<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        mut self,
        modifier: StyleModifier,
        f: F,
    ) -> Self {
        let ix = self
            .styles
            .iter()
            .position(|style| style.style.modifier == modifier);
        if let Some(ix) = ix {
            let mut style = std::mem::take(&mut self.styles[ix].style);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: &mut self,
            };
            let _ = (f)(delegate);
            self.styles[ix].style = style;
            self
        } else {
            let mut style = StyleDefinition::new(modifier);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: &mut self,
            };
            let _ = (f)(delegate);
            self.styles.push(Entry {
                style,
                parent_modifier: self.styles.first().map(|s| s.style.modifier.clone()),
            });
            self
        }
    }
}

impl StyleCollector {
    pub fn build(mut self) -> String {
        self.styles
            .sort_by(|a, b| a.style.modifier.cmp(&b.style.modifier));
        let mut out = String::new();
        for style in self.into_styles() {
            style.write_to(&mut out);
        }
        out
    }

    pub fn into_styles(mut self) -> impl Iterator<Item = StyleDefinition> {
        for i in 0..self.styles.len() {
            let mut parent_modifier = self.styles[i].parent_modifier.clone();
            let mut infinite_loop_protection = self.styles.len();
            while let Some(m) = parent_modifier.take() {
                if infinite_loop_protection == 0 {
                    break;
                }
                infinite_loop_protection -= 1;

                let Some(ix) = self
                    .styles
                    .iter()
                    .position(|style| style.style.modifier == m)
                else {
                    continue;
                };
                if i == ix {
                    break;
                }

                if let Ok([a, b]) = self.styles.get_disjoint_mut([i, ix]) {
                    a.style.inherit(&b.style);
                    parent_modifier = b.parent_modifier.clone();
                }
            }
        }
        self.styles.into_iter().map(|e| e.style)
    }

    pub fn combine(mut self, other: Self) -> Self {
        for other in other.styles {
            let ix = self.styles.iter().position(|style| {
                style.style.modifier == other.style.modifier
                    && style.parent_modifier == other.parent_modifier
            });
            if let Some(ix) = ix {
                self.styles[ix].style.merge_from(other.style);
            } else {
                self.styles.push(other);
            }
        }
        self
    }

    pub fn combine_when(self, condition: bool, other: Self) -> Self {
        if condition { self.combine(other) } else { self }
    }
}

pub struct StyleDelegate<'a> {
    style: &'a mut StyleDefinition,
    collector: &'a mut StyleCollector,
}

impl<'a> Style for StyleDelegate<'a> {
    fn style_mut(&mut self) -> &mut StyleDefinition {
        &mut self.style
    }
}

impl<'a> SubStyle for StyleDelegate<'a> {
    fn style_mut_for(&mut self, mut modifier: StyleModifier) -> &mut StyleDefinition {
        self.style.modifier.merge_into(&mut modifier);
        self.collector.style_mut_for(modifier)
    }

    fn substyle<F: for<'b> FnOnce(StyleDelegate<'b>) -> StyleDelegate<'b>>(
        self,
        mut modifier: StyleModifier,
        f: F,
    ) -> Self {
        self.style.modifier.merge_into(&mut modifier);
        let ix = self
            .collector
            .styles
            .iter()
            .position(|style| style.style.modifier == modifier);

        if let Some(ix) = ix {
            let mut style = std::mem::take(&mut self.collector.styles[ix].style);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: self.collector,
            };
            let _ = (f)(delegate);
            self.collector.styles[ix].style = style;
            self
        } else {
            let mut style = StyleDefinition::new(modifier);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: self.collector,
            };
            let _ = (f)(delegate);
            self.collector.styles.push(Entry {
                style,
                parent_modifier: Some(self.style.modifier.clone()),
            });
            self
        }
    }
}
