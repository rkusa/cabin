use smallvec::SmallVec;

use crate::error::InternalError;
use crate::style::modifier::StyleModifier;
use crate::style::{Style, StyleDefinition, SubStyle};

#[derive(Default)]
pub struct StyleCollector {
    styles: SmallVec<StyleDefinition, 1>,
}

impl Style for StyleCollector {
    fn style_mut(&mut self) -> &mut StyleDefinition {
        if self.styles.is_empty() {
            let style = StyleDefinition::default();
            self.styles.push(style);
        }
        self.styles.last_mut().unwrap()
    }
}

impl SubStyle for StyleCollector {
    fn style_mut_for(&mut self, modifier: StyleModifier) -> &mut StyleDefinition {
        let ix = self
            .styles
            .iter_mut()
            .position(|style| style.modifier == modifier);
        if let Some(ix) = ix {
            &mut self.styles[ix]
        } else {
            let style = StyleDefinition::new(modifier);
            self.styles.push(style);
            self.styles.last_mut().unwrap()
        }
    }

    fn substyle<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        mut self,
        modifier: StyleModifier,
        f: F,
    ) -> Self {
        let ix = self
            .styles
            .iter_mut()
            .position(|style| style.modifier == modifier);

        if let Some(ix) = ix {
            let mut style = std::mem::take(&mut self.styles[ix]);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: &mut self,
            };
            let _ = (f)(delegate);
            self.styles[ix] = style;
            self
        } else {
            let mut style = StyleDefinition::new(modifier);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: &mut self,
            };
            let _ = (f)(delegate);
            self.styles.push(style);
            self
        }
    }
}

impl StyleCollector {
    pub fn build(mut self, _include_base: bool) -> Result<String, crate::Error> {
        self.styles.sort_by(|a, b| a.modifier.cmp(&b.modifier));

        let mut out = SmallVec::<u8, 32>::new();
        for style in self.styles {
            style.write_to(&mut out);
        }
        Ok(str::from_utf8(&out)
            .map_err(InternalError::from)?
            .to_string())
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
        modifier: StyleModifier,
        f: F,
    ) -> Self {
        let ix = self
            .collector
            .styles
            .iter_mut()
            .position(|style| style.modifier == modifier);

        if let Some(ix) = ix {
            let mut style = std::mem::take(&mut self.collector.styles[ix]);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: self.collector,
            };
            let _ = (f)(delegate);
            self.collector.styles[ix] = style;
            self
        } else {
            let mut style = StyleDefinition::new(modifier);
            let delegate = StyleDelegate {
                style: &mut style,
                collector: self.collector,
            };
            let _ = (f)(delegate);
            self.collector.styles.push(style);
            self
        }
    }
}
