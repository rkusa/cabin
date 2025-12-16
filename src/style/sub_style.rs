use super::collector::StyleDelegate;
use super::style_definition::StyleDefinition;
use super::units::length::Length;
use crate::style::Style;
use crate::style::animation::AnimationStyle;
use crate::style::modifier::StyleModifier;

#[cabin_macros::length_aliases]
pub trait SubStyle: Style {
    fn style_mut_for(&mut self, modifier: StyleModifier) -> &mut StyleDefinition;
    fn substyle<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        modifier: StyleModifier,
        f: F,
    ) -> Self;

    fn when_active<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                active: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_disabled<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                disabled: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_enabled<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                enabled: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_focus<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                focus: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_focus_visible<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                focus_visible: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_focus_within<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                focus_within: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_hover<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                hover: true,
                ..Default::default()
            },
            f,
        )
    }

    fn when_visited<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                visited: true,
                ..Default::default()
            },
            f,
        )
    }

    fn after<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        let mut style = self.substyle(
            StyleModifier {
                after: true,
                ..Default::default()
            },
            f,
        );
        {
            let style = style.style_mut();
            if style.content.is_none() {
                style.content = Some("");
            }
        }
        style
    }

    fn before<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        let mut style = self.substyle(
            StyleModifier {
                before: true,
                ..Default::default()
            },
            f,
        );
        {
            let style = style.style_mut();
            if style.content.is_none() {
                style.content = Some("");
            }
        }
        style
    }

    fn when_group_hover<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                group_hover: true,
                ..Default::default()
            },
            f,
        )
    }

    fn apply_to_children<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                all_children: true,
                ..Default::default()
            },
            f,
        )
    }

    fn max_page_width<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        max_width: u32,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                max_width: Some(max_width),
                ..Default::default()
            },
            f,
        )
    }

    fn min_page_width<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        min_width: u32,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                min_width: Some(min_width),
                ..Default::default()
            },
            f,
        )
    }

    fn max_container_width<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        max_width: u32,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                max_container_width: Some(max_width),
                ..Default::default()
            },
            f,
        )
    }

    fn min_container_width<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        min_width: u32,
        f: F,
    ) -> Self {
        self.substyle(
            StyleModifier {
                min_container_width: Some(min_width),
                ..Default::default()
            },
            f,
        )
    }

    fn print<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                print: true,
                ..Default::default()
            },
            f,
        )
    }

    fn dark<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                dark: true,
                ..Default::default()
            },
            f,
        )
    }

    fn animate_from<F: FnOnce(AnimationStyle) -> AnimationStyle>(mut self, f: F) -> Self {
        let style = self.style_mut();
        let animation = style.animation_from.take().unwrap_or_default();
        let animation = (f)(animation);
        style.animation_from = Some(animation);
        self
    }

    fn animate_to<F: FnOnce(AnimationStyle) -> AnimationStyle>(mut self, f: F) -> Self {
        let style = self.style_mut();
        let animation = style.animation_to.take().unwrap_or_default();
        let animation = (f)(animation);
        style.animation_to = Some(animation);
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-inline-start-width: 1px;
    /// }
    /// ```
    fn divide_x(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_width
        .get_or_insert_default()
        .set_inline_start(Length::Px(1.0));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-inline-start-width: {px}px;
    /// }
    /// ```
    fn divide_x_px(mut self, px: i16) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_width
        .get_or_insert_default()
        .set_inline_start(Length::Px(f32::from(px)));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-inline-start-width: {px}px;
    /// }
    /// ```
    fn divide_x_pxf(mut self, px: f32) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_width
        .get_or_insert_default()
        .set_inline_start(Length::Px(px));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-inline-end-width: border-inline-start-width;
    ///     border-inline-start-width: 0;
    /// }
    /// ```
    fn divide_x_reverse(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .divide_x_reversed = true;
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-block-start-width: 1px;
    /// }
    /// ```
    fn divide_y(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_width
        .get_or_insert_default()
        .set_block_start(Length::Px(1.0));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-block-start-width: {px}px;
    /// }
    /// ```
    fn divide_y_px(mut self, px: i16) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_width
        .get_or_insert_default()
        .set_block_start(Length::Px(f32::from(px)));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-block-start-width: {px}px;
    /// }
    /// ```
    fn divide_y_pxf(mut self, px: f32) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_width
        .get_or_insert_default()
        .set_block_start(Length::Px(px));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     border-block-end-width: border-block-start-width;
    ///     border-block-start-width: 0;
    /// }
    /// ```
    fn divide_y_reverse(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .divide_y_reversed = true;
        self
    }

    /// Set a custom divide border color.
    fn divide_color(mut self, color: &'static str) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_color
        .get_or_insert_default()
        .set(color);
        self
    }

    /// Remove an existing border style.
    /// ```css
    /// border-inline-style: none;
    /// ```
    fn divide_none(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_style = Some("none");
        self
    }

    /// ```css
    /// border-inline-style: solid;
    /// ```
    fn divide_solid(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_style = Some("solid");
        self
    }

    /// ```css
    /// border-inline-style: dashed;
    /// ```
    fn divide_dashed(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_style = Some("dashed");
        self
    }

    /// ```css
    /// border-inline-style: dotted;
    /// ```
    fn divide_dotted(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_style = Some("dotted");
        self
    }

    /// ```css
    /// border-inline-style: double;
    /// ```
    fn divide_double(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .border_inline_style = Some("double");
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-inline-start: 1px;
    /// }
    /// ```
    fn space_x(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .margin_inline
        .get_or_insert_default()
        .set_inline_start(Length::Px(1.0));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-inline-start: {px}px;
    /// }
    /// ```
    fn space_x_px(mut self, px: i16) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .margin_inline
        .get_or_insert_default()
        .set_inline_start(Length::Px(f32::from(px)));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-inline-start: {px}px;
    /// }
    /// ```
    fn space_x_pxf(mut self, px: f32) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .margin_inline
        .get_or_insert_default()
        .set_inline_start(Length::Px(px));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-inline-end: border-inline-start-width;
    ///     margin-inline-start: 0;
    /// }
    /// ```
    fn space_x_reverse(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .space_x_reversed = true;
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-block-start: 1px;
    /// }
    /// ```
    fn space_y(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .margin_inline
        .get_or_insert_default()
        .set_block_start(Length::Px(1.0));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-block-start: {px}px;
    /// }
    /// ```
    fn space_y_px(mut self, px: i16) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .margin_inline
        .get_or_insert_default()
        .set_block_start(Length::Px(f32::from(px)));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-block-start: {px}px;
    /// }
    /// ```
    fn space_y_pxf(mut self, px: f32) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .margin_inline
        .get_or_insert_default()
        .set_block_start(Length::Px(px));
        self
    }

    /// ```css
    /// & > :not(:last-child) {
    ///     margin-block-end: border-block-start-width;
    ///     margin-block-start: 0;
    /// }
    /// ```
    fn space_y_reverse(mut self) -> Self {
        self.style_mut_for(StyleModifier {
            all_but_last_children: true,
            ..Default::default()
        })
        .space_y_reversed = true;
        self
    }
}

impl<T> SubStyleExt for T where T: SubStyle {}
