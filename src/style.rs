pub mod collector;
mod modifier;
mod property_display;
mod style_definition;
mod theme;
mod units;

pub use style_definition::StyleDefinition;
pub use theme::ThemeExt;

use collector::StyleDelegate;
use units::aspect::Aspect;
use units::box_shadow::ShadowKind;
use units::duration::Duration;
use units::either::Either;
use units::grid_lines::GridLine;
use units::iterations::Iterations;
use units::length::Length;
use units::line_clamp::LineClamp;
use units::track_repeat_equally::TrackRepeatEqually;
use units::transform::Transform;

use crate::style::modifier::StyleModifier;

#[cabin_macros::length_aliases]
pub trait Style: Sized {
    fn style_mut(&mut self) -> &mut StyleDefinition;
    fn style_mut_for(&mut self, modifier: StyleModifier) -> &mut StyleDefinition;
    fn substyle<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        self,
        modifier: StyleModifier,
        f: F,
    ) -> Self;

    fn active<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                active: true,
                ..Default::default()
            },
            f,
        )
    }

    fn disabled<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                disabled: true,
                ..Default::default()
            },
            f,
        )
    }

    fn enabled<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                enabled: true,
                ..Default::default()
            },
            f,
        )
    }

    fn focus<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                focus: true,
                ..Default::default()
            },
            f,
        )
    }

    fn focus_visible<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
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

    fn focus_within<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                focus_within: true,
                ..Default::default()
            },
            f,
        )
    }

    fn hover<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
        self.substyle(
            StyleModifier {
                hover: true,
                ..Default::default()
            },
            f,
        )
    }

    fn visited<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
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

    fn group_hover<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {
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

    /// Duration of CSS animations in milliseconds.
    /// ```css
    /// animation-duration: {ms}ms;
    /// ```
    fn animation_duration_ms(mut self, ms: u32) -> Self {
        self.style_mut().animation_duration = Some(Duration::Ms(ms));
        self
    }

    /// Duration of CSS animations in seconds.
    /// ```css
    /// animation-duration: {s}s;
    /// ```
    fn animation_duration_s(mut self, s: f32) -> Self {
        self.style_mut().animation_duration = Some(Duration::S(s));
        self
    }

    /// Delay of CSS animations in milliseconds.
    /// ```css
    /// animation-delay: {ms}ms;
    /// ```
    fn animation_delay_ms(mut self, ms: u32) -> Self {
        self.style_mut().animation_duration = Some(Duration::Ms(ms));
        self
    }

    /// Delay of CSS animations in seconds.
    /// ```css
    /// animation-delay: {s}s;
    /// ```
    fn animation_delay_s(mut self, s: f32) -> Self {
        self.style_mut().animation_duration = Some(Duration::S(s));
        self
    }

    /// Number of times the animation is played before stopping.
    /// ```css
    /// animation-iteration-count: {n};
    /// ```
    fn animation_iterations(mut self, n: u16) -> Self {
        self.style_mut().animation_iterations = Some(Iterations::Count(n));
        self
    }

    /// Run the animation indefinitely.
    /// ```css
    /// animation-iteration-count: infinite;
    /// ```
    fn animation_iterations_infinite(mut self) -> Self {
        self.style_mut().animation_iterations = Some(Iterations::Infinite);
        self
    }

    /// Disable platform-specific styling.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/appearance>
    /// ```css
    /// appearance: none;
    /// ```
    fn appearance_none(mut self) -> Self {
        self.style_mut().appearance = Some(false);
        self
    }

    /// Enable platform-specific styling.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/appearance>
    /// ```css
    /// appearance: auto;
    /// ```
    fn appearance_auto(mut self) -> Self {
        self.style_mut().appearance = Some(true);
        self
    }

    /// Set the preferred aspect ratio for the box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    /// ```css
    /// aspect-ratio: auto;
    /// ```
    fn aspect_auto(mut self) -> Self {
        self.style_mut().aspect = Some(Aspect::Auto);
        self
    }

    /// Set the preferred aspect ratio for the box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    /// ```css
    /// aspect-ratio: 1 / 1;
    /// ```
    fn aspect_square(mut self) -> Self {
        self.style_mut().aspect = Some(Aspect::Ratio(1, 1));
        self
    }

    /// Set the preferred aspect ratio for the box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    /// ```css
    /// aspect-ratio: 19 / 9;
    /// ```
    fn aspect_video(mut self) -> Self {
        self.style_mut().aspect = Some(Aspect::Ratio(16, 9));
        self
    }

    /// Set the preferred aspect ratio for the box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    /// ```css
    /// aspect-ratio: {w} / {h};
    /// ```
    fn aspect_ratio(mut self, w: u32, h: u32) -> Self {
        self.style_mut().aspect = Some(Aspect::Ratio(w, h));
        self
    }

    /// Set the preferred aspect ratio for the box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    /// ```css
    /// aspect-ratio: {ratio};
    /// ```
    fn aspect_ratiof(mut self, ratio: f32) -> Self {
        self.style_mut().aspect = Some(Aspect::Ratiof(ratio));
        self
    }

    /// Control the size of implicitly created grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    /// ```css
    /// grid-auto-columns: auto;
    /// ```
    fn auto_cols_auto(mut self) -> Self {
        self.style_mut().auto_cols = Some(Either::Right("auto"));
        self
    }

    /// Control the size of implicitly created grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    /// ```css
    /// grid-auto-columns: min-content;
    /// ```
    fn auto_cols_min(mut self) -> Self {
        self.style_mut().auto_cols = Some(Either::Left(Length::MinContent));
        self
    }

    /// Control the size of implicitly created grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    /// ```css
    /// grid-auto-columns: max-content;
    /// ```
    fn auto_cols_max(mut self) -> Self {
        self.style_mut().auto_cols = Some(Either::Left(Length::MaxContent));
        self
    }

    /// Control the size of implicitly created grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    /// ```css
    /// grid-auto-columns: minmax(0, 1fr);
    /// ```
    fn auto_cols_fr(mut self) -> Self {
        self.style_mut().auto_cols = Some(Either::Right("minmax(0, 1fr)"));
        self
    }

    /// Control the size of implicitly created grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    /// ```css
    /// grid-auto-columns: {auto_cols};
    /// ```
    fn auto_cols_with(mut self, auto_cols: &'static str) -> Self {
        self.style_mut().auto_cols = Some(Either::Right(auto_cols));
        self
    }

    /// Control the size of implicitly created grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>
    /// ```css
    /// grid-auto-columns: {x};
    /// ```
    #[with_auto]
    #[with_horizontal_viewport_units]
    fn auto_cols(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().auto_cols = Some(Either::Left(x.into()));
        self
    }

    /// Control the size of implicitly created grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    /// ```css
    /// grid-auto-rows: auto;
    /// ```
    fn auto_rows_auto(mut self) -> Self {
        self.style_mut().auto_rows = Some(Either::Right("auto"));
        self
    }

    /// Control the size of implicitly created grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    /// ```css
    /// grid-auto-rows: min-content;
    /// ```
    fn auto_rows_min(mut self) -> Self {
        self.style_mut().auto_rows = Some(Either::Left(Length::MinContent));
        self
    }

    /// Control the size of implicitly created grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    /// ```css
    /// grid-auto-rows: max-content;
    /// ```
    fn auto_rows_max(mut self) -> Self {
        self.style_mut().auto_rows = Some(Either::Left(Length::MaxContent));
        self
    }

    /// Control the size of implicitly created grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    /// ```css
    /// grid-auto-rows: minmax(0, 1fr);
    /// ```
    fn auto_rows_fr(mut self) -> Self {
        self.style_mut().auto_rows = Some(Either::Right("minmax(0, 1fr)"));
        self
    }

    /// Control the size of implicitly created grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    /// ```css
    /// grid-auto-rows: {auto_rows};
    /// ```
    fn auto_rows_with(mut self, auto_rows: &'static str) -> Self {
        self.style_mut().auto_rows = Some(Either::Right(auto_rows));
        self
    }

    /// Control the size of implicitly created grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>
    /// ```css
    /// grid-auto-rows: {x};
    /// ```
    #[with_auto]
    #[with_vertical_viewport_units]
    fn auto_rows(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().auto_rows = Some(Either::Left(x.into()));
        self
    }

    /// Set the initial main size of a flex item (`flex-basis`).
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>
    /// ```css
    /// padding-right: {x};
    /// padding-left: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_horizontal_viewport_units]
    fn basis(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().basis = Some(x.into());
        self
    }

    /// Set that the element's background extends underneath its border box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip>
    /// ```css
    /// background-clip: border-box;
    /// ```
    fn bg_border(mut self) -> Self {
        self.style_mut().background_clip = Some("border-box");
        self
    }

    /// Set that the element's background extends underneath its padding box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip>
    /// ```css
    /// background-clip: padding-box;
    /// ```
    fn bg_padding(mut self) -> Self {
        self.style_mut().background_clip = Some("padding-box");
        self
    }

    /// Set that the element's background extends underneath its content box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip>
    /// ```css
    /// background-clip: content-box;
    /// ```
    fn bg_content(mut self) -> Self {
        self.style_mut().background_clip = Some("content-box");
        self
    }

    /// Set that the element's background extends underneath its text.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip>
    /// ```css
    /// background-clip: text;
    /// ```
    fn bg_text(mut self) -> Self {
        self.style_mut().background_clip = Some("text");
        self
    }

    /// Set the background color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/background-color>
    /// ```css
    /// background-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn bg(mut self, color: &'static str) -> Self {
        self.style_mut().background_color = Some(color);
        self
    }

    /// ```css
    /// background-image: none;
    /// ```
    fn bg_none(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .set_none();
        self
    }

    /// ```css
    /// background-image: linear-gradient(to top, ...);
    /// ```
    fn bg_linear_to_t(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to top");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to top right, ...);
    /// ```
    fn bg_linear_to_tr(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to top right");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to right, ...);
    /// ```
    fn bg_linear_to_r(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to right");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to bottom right, ...);
    /// ```
    fn bg_linear_to_br(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to bottom right");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to bottom, ...);
    /// ```
    fn bg_linear_to_b(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to bottom");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to bottom left, ...);
    /// ```
    fn bg_linear_to_bl(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to bottom left");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to left, ...);
    /// ```
    fn bg_linear_to_l(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to left");
        self
    }

    /// ```css
    /// background-image: linear-gradient(to top left, ...);
    /// ```
    fn bg_linear_to_tl(mut self) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .gradient_line = Some("to top left");
        self
    }

    /// Set the border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-color>
    /// ```css
    /// border-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_color
            .get_or_insert_default()
            .set(color);
        self
    }

    /// Set the logical inline borders color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-color>
    /// ```css
    /// border-inline-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_x(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_inline_color
            .get_or_insert_default()
            .set_x(color);
        self
    }

    /// Set the logical block borders color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-color>
    /// ```css
    /// border-block-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_y(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_inline_color
            .get_or_insert_default()
            .set_y(color);
        self
    }

    /// Set the top border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-color>
    /// ```css
    /// border-top-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_t(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_color
            .get_or_insert_default()
            .set_top(color);
        self
    }

    /// Set the right border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-color>
    /// ```css
    /// border-right-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_r(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_color
            .get_or_insert_default()
            .set_right(color);
        self
    }

    /// Set the bottom border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-color>
    /// ```css
    /// border-bottom-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_b(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_color
            .get_or_insert_default()
            .set_bottom(color);
        self
    }

    /// Set the left border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-color>
    /// ```css
    /// border-left-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_l(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_color
            .get_or_insert_default()
            .set_left(color);
        self
    }

    /// Set the logical inline start border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-color>
    /// ```css
    /// border-inline-start-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_s(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_inline_color
            .get_or_insert_default()
            .set_inline_start(color);
        self
    }

    /// Set the logical inline end border color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-color>
    /// ```css
    /// border-inline-end-color: {color};` <b style="color:{color}">⏺</b>
    /// ```
    fn border_color_e(mut self, color: &'static str) -> Self {
        self.style_mut()
            .border_inline_color
            .get_or_insert_default()
            .set_inline_end(color);
        self
    }

    /// Set the border width on all sides.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
    /// ```css
    /// border-width: 1px;
    /// ```
    fn border(mut self) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set(Length::Px(1.0));
        self
    }

    /// Set the border width on all sides.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
    /// ```css
    /// border-width: {x}px;
    /// ```
    fn border_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set(Length::Px(f32::from(px)));
        self
    }

    /// Set the border width on all sides.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
    /// ```css
    /// border-width: {x}px;
    /// ```
    fn border_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set(Length::Px(px));
        self
    }

    /// Set the logical inline borders width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-width>
    /// ```css
    /// border-inline-width: 1px;
    /// ```
    fn border_x(mut self) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_x(Length::Px(1.0));
        self
    }

    /// Set the logical inline borders width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-width>
    /// ```css
    /// border-inline-width: {x}px;
    /// ```
    fn border_x_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_x(Length::Px(f32::from(px)));
        self
    }

    /// Set the logical inline borders width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-width>
    /// ```css
    /// border-inline-width: {x}px;
    /// ```
    fn border_x_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_x(Length::Px(px));
        self
    }

    /// Set the logical block border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-width>
    /// ```css
    /// border-block-width: 1px;
    /// ```
    fn border_y(mut self) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_y(Length::Px(1.0));
        self
    }

    /// Set the logical block border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-blcok-width>
    /// ```css
    /// border-block-width: {x}px;
    /// ```
    fn border_y_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_y(Length::Px(f32::from(px)));
        self
    }

    /// Set the logical block border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-block-width>
    /// ```css
    /// border-block-width: {x}px;
    /// ```
    fn border_y_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_y(Length::Px(px));
        self
    }

    /// Set the top border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-width>
    /// ```css
    /// border-top-width: 1px;
    /// ```
    fn border_t(mut self) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_top(Length::Px(1.0));
        self
    }

    /// Set the top border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-width>
    /// ```css
    /// border-top-width: {x}px;
    /// ```
    fn border_t_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_top(Length::Px(f32::from(px)));
        self
    }

    /// Set the top border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-width>
    /// ```css
    /// border-top-width: {x}px;
    /// ```
    fn border_t_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_top(Length::Px(px));
        self
    }

    /// Set the right border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-width>
    /// ```css
    /// border-right-width: 1px;
    /// ```
    fn border_r(mut self) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_right(Length::Px(1.0));
        self
    }

    /// Set the right border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-width>
    /// ```css
    /// border-right-width: {x}px;
    /// ```
    fn border_r_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_right(Length::Px(f32::from(px)));
        self
    }

    /// Set the right border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-right-width>
    /// ```css
    /// border-right-width: {x}px;
    /// ```
    fn border_r_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_right(Length::Px(px));
        self
    }

    /// Set the bottom border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-width>
    /// ```css
    /// border-bottom-width: 1px;
    /// ```
    fn border_b(mut self) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_bottom(Length::Px(1.0));
        self
    }

    /// Set the bottom border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-width>
    /// ```css
    /// border-bottom-width: {x}px;
    /// ```
    fn border_b_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_bottom(Length::Px(f32::from(px)));
        self
    }

    /// Set the bottom border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-width>
    /// ```css
    /// border-bottom-width: {x}px;
    /// ```
    fn border_b_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_bottom(Length::Px(px));
        self
    }

    /// Set the left border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-width>
    /// ```css
    /// border-left-width: 1px;
    /// ```
    fn border_l(mut self) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_left(Length::Px(1.0));
        self
    }

    /// Set the left border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-width>
    /// ```css
    /// border-left-width: {x}px;
    /// ```
    fn border_l_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_left(Length::Px(f32::from(px)));
        self
    }

    /// Set the left border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-left-width>
    /// ```css
    /// border-left-width: {x}px;
    /// ```
    fn border_l_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_width
            .get_or_insert_default()
            .set_left(Length::Px(px));
        self
    }

    /// Set the logical inline start border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-width>
    /// ```css
    /// border-inline-start-width: 1px;
    /// ```
    fn border_s(mut self) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_inline_start(Length::Px(1.0));
        self
    }

    /// Set the logical inline start border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-width>
    /// ```css
    /// border-inline-start-width: {x}px;
    /// ```
    fn border_s_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_inline_start(Length::Px(f32::from(px)));
        self
    }

    /// Set the logical inline start border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-start-width>
    /// ```css
    /// border-inline-start-width: {x}px;
    /// ```
    fn border_s_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_inline_start(Length::Px(px));
        self
    }

    /// Set the logical inline end border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-width>
    /// ```css
    /// border-inline-end-width: 1px;
    /// ```
    fn border_e(mut self) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_inline_end(Length::Px(1.0));
        self
    }

    /// Set the logical inline end border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-width>
    /// ```css
    /// border-inline-end-width: {x}px;
    /// ```
    fn border_e_px(mut self, px: i16) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_inline_end(Length::Px(f32::from(px)));
        self
    }

    /// Set the logical inline end border width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-inline-end-width>
    /// ```css
    /// border-inline-end-width: {x}px;
    /// ```
    fn border_e_pxf(mut self, px: f32) -> Self {
        self.style_mut()
            .border_inline_width
            .get_or_insert_default()
            .set_inline_end(Length::Px(px));
        self
    }

    /// Remove an existing border style.
    /// ```css
    /// border-style: none;
    /// ```
    fn border_none(mut self) -> Self {
        self.style_mut().border_style = Some("none");
        self
    }

    /// ```css
    /// border-style: solid;
    /// ```
    fn border_solid(mut self) -> Self {
        self.style_mut().border_style = Some("solid");
        self
    }

    /// ```css
    /// border-style: dashed;
    /// ```
    fn border_dashed(mut self) -> Self {
        self.style_mut().border_style = Some("dashed");
        self
    }

    /// ```css
    /// border-style: dotted;
    /// ```
    fn border_dotted(mut self) -> Self {
        self.style_mut().border_style = Some("dotted");
        self
    }

    /// ```css
    /// border-style: double;
    /// ```
    fn border_double(mut self) -> Self {
        self.style_mut().border_style = Some("double");
        self
    }

    /// ```css
    /// border-style: hidden;
    /// ```
    fn border_hidden(mut self) -> Self {
        self.style_mut().border_style = Some("hidden");
        self
    }

    /// Set the bottom position of the positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>
    /// ```css
    /// bottom: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_vertical]
    #[with_vertical_viewport_units]
    fn bottom(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .inset
            .get_or_insert_default()
            .set_bottom(x.into());
        self
    }

    /// Set how the browser should calculate an element's total size.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
    /// ```css
    /// box-sizing: border-box;
    /// ```
    fn box_border(mut self) -> Self {
        self.style_mut().box_sizing = Some("border-box");
        self
    }

    /// Set how the browser should calculate an element's total size.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
    /// ```css
    /// box-sizing: content-box;
    /// ```
    fn box_content(mut self) -> Self {
        self.style_mut().box_sizing = Some("content-box");
        self
    }

    /// Set how words break in the element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-wrap>
    /// ```css
    /// overflow-wrap: normal;
    /// word-break: normal;
    /// ```
    fn break_normal(mut self) -> Self {
        self.style_mut().word_break = Some("normal");
        self
    }

    /// Set how words break in the element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>
    /// ```css
    /// word-break: break-wird;
    /// ```
    fn break_word(mut self) -> Self {
        self.style_mut().word_break = Some("break-word");
        self
    }

    /// Set how words break in the element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>
    /// ```css
    /// word-break: break-all;
    /// ```
    fn break_all(mut self) -> Self {
        self.style_mut().word_break = Some("break-all");
        self
    }

    /// Set how words break in the element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>
    /// ```css
    /// word-break: keep-all;
    /// ```
    fn break_keep(mut self) -> Self {
        self.style_mut().word_break = Some("keep-all");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: auto;
    /// ```
    fn break_after_auto(mut self) -> Self {
        self.style_mut().break_after = Some("auto");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: avoid;
    /// ```
    fn break_after_avoid(mut self) -> Self {
        self.style_mut().break_after = Some("avoid");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: all;
    /// ```
    fn break_after_all(mut self) -> Self {
        self.style_mut().break_after = Some("all");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: avoid-page;
    /// ```
    fn break_after_avoid_page(mut self) -> Self {
        self.style_mut().break_after = Some("avoid-page");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: page;
    /// ```
    fn break_after_page(mut self) -> Self {
        self.style_mut().break_after = Some("page");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: left;
    /// ```
    fn break_after_left(mut self) -> Self {
        self.style_mut().break_after = Some("left");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: right;
    /// ```
    fn break_after_right(mut self) -> Self {
        self.style_mut().break_after = Some("right");
        self
    }

    /// Set how page, column, or region breaks should behave after a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>
    /// ```css
    /// break-after: column;
    /// ```
    fn break_after_column(mut self) -> Self {
        self.style_mut().break_after = Some("column");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: auto;
    /// ```
    fn break_before_auto(mut self) -> Self {
        self.style_mut().break_before = Some("auto");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: avoid;
    /// ```
    fn break_before_avoid(mut self) -> Self {
        self.style_mut().break_before = Some("avoid");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: all;
    /// ```
    fn break_before_all(mut self) -> Self {
        self.style_mut().break_before = Some("all");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: avoid-page;
    /// ```
    fn break_before_avoid_page(mut self) -> Self {
        self.style_mut().break_before = Some("avoid-page");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: page;
    /// ```
    fn break_before_page(mut self) -> Self {
        self.style_mut().break_before = Some("page");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: left;
    /// ```
    fn break_before_left(mut self) -> Self {
        self.style_mut().break_before = Some("left");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: right;
    /// ```
    fn break_before_right(mut self) -> Self {
        self.style_mut().break_before = Some("right");
        self
    }

    /// Set how page, column, or region breaks should behave before a generated box.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>
    /// ```css
    /// break-before: column;
    /// ```
    fn break_before_column(mut self) -> Self {
        self.style_mut().break_before = Some("column");
        self
    }

    /// Set how page, column, or region breaks should behave within an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside>
    /// ```css
    /// break-inside: auto;
    /// ```
    fn break_inside_auto(mut self) -> Self {
        self.style_mut().break_inside = Some("auto");
        self
    }

    /// Set how page, column, or region breaks should behave within an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside>
    /// ```css
    /// break-inside: avoid;
    /// ```
    fn break_inside_avoid(mut self) -> Self {
        self.style_mut().break_inside = Some("avoid");
        self
    }

    /// Set how page, column, or region breaks should behave within an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside>
    /// ```css
    /// break-inside: page;
    /// ```
    fn break_inside_page(mut self) -> Self {
        self.style_mut().break_inside = Some("page");
        self
    }

    /// Set how page, column, or region breaks should behave within an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside>
    /// ```css
    /// break-inside: column;
    /// ```
    fn break_inside_column(mut self) -> Self {
        self.style_mut().break_inside = Some("column");
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
    /// ```css
    /// grid-column: auto;
    /// ```
    fn col_auto(mut self) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set(GridLine::Auto);
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
    /// ```css
    /// grid-column: 1 / -1;
    /// ```
    fn col_full(mut self) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set_start(GridLine::Nth(1));
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set_end(GridLine::Nth(-1));
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
    /// ```css
    /// grid-column: {n};
    /// ```
    fn col(mut self, n: i16) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set(GridLine::Nth(n));
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>
    /// ```css
    /// grid-column: span {n} / span {n};
    /// ```
    fn col_span(mut self, n: u16) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set_start(GridLine::Span(n));
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set_end(GridLine::Span(n));
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column-start>
    /// ```css
    /// grid-column-start: auto;
    /// ```
    fn col_start_auto(mut self) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set(GridLine::Auto);
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column-start>
    /// ```css
    /// grid-column-start: {n};
    /// ```
    fn col_start(mut self, n: i16) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set_start(GridLine::Nth(n));
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column-end>
    /// ```css
    /// grid-column-end: auto;
    /// ```
    fn col_end_auto(mut self) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set(GridLine::Auto);
        self
    }

    /// Set how elements are sized and placed across grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column-end>
    /// ```css
    /// grid-column-end: {n};
    /// ```
    fn col_end(mut self, n: i16) -> Self {
        self.style_mut()
            .grid_column
            .get_or_insert_default()
            .set_end(GridLine::Nth(n));
        self
    }

    /// Establish the element as a query container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/container-type>
    /// ```css
    /// container-type: normal;
    /// ```
    fn container_normal(mut self) -> Self {
        self.style_mut().container = Some("normal");
        self
    }

    /// Establish the element as a query container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/container-type>
    /// ```css
    /// container-type: size;
    /// ```
    fn container_size(mut self) -> Self {
        self.style_mut().container = Some("size");
        self
    }

    /// Establish the element as a query container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/container-type>
    /// ```css
    /// container-type: inline-size;
    /// ```
    fn container_inline_size(mut self) -> Self {
        self.style_mut().container = Some("inline-size");
        self
    }

    /// Replace the element's content.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/content>
    /// ```css
    /// content: {content};
    /// ```
    fn content(mut self, content: &'static str) -> Self {
        self.style_mut().content = Some(content);
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: auto;
    /// ```
    fn cursor_auto(mut self) -> Self {
        self.style_mut().cursor = Some("auto");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: default;
    /// ```
    fn cursor_default(mut self) -> Self {
        self.style_mut().cursor = Some("default");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: pointer;
    /// ```
    fn cursor_pointer(mut self) -> Self {
        self.style_mut().cursor = Some("pointer");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: wait;
    /// ```
    fn cursor_wait(mut self) -> Self {
        self.style_mut().cursor = Some("wait");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: text;
    /// ```
    fn cursor_text(mut self) -> Self {
        self.style_mut().cursor = Some("text");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: move;
    /// ```
    fn cursor_move(mut self) -> Self {
        self.style_mut().cursor = Some("move");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: help;
    /// ```
    fn cursor_help(mut self) -> Self {
        self.style_mut().cursor = Some("help");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: not-allowed;
    /// ```
    fn cursor_not_allowed(mut self) -> Self {
        self.style_mut().cursor = Some("not-allowed");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: none;
    /// ```
    fn cursor_none(mut self) -> Self {
        self.style_mut().cursor = Some("none");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: context-menu;
    /// ```
    fn cursor_context_menu(mut self) -> Self {
        self.style_mut().cursor = Some("context-menu");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: progress;
    /// ```
    fn cursor_progress(mut self) -> Self {
        self.style_mut().cursor = Some("progress");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: cell;
    /// ```
    fn cursor_cell(mut self) -> Self {
        self.style_mut().cursor = Some("cell");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: crosshair;
    /// ```
    fn cursor_crosshair(mut self) -> Self {
        self.style_mut().cursor = Some("crosshair");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: vertical-text;
    /// ```
    fn cursor_vertical_text(mut self) -> Self {
        self.style_mut().cursor = Some("vertical-text");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: alias;
    /// ```
    fn cursor_alias(mut self) -> Self {
        self.style_mut().cursor = Some("alias");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: copy;
    /// ```
    fn cursor_copy(mut self) -> Self {
        self.style_mut().cursor = Some("copy");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: no-drop;
    /// ```
    fn cursor_no_drop(mut self) -> Self {
        self.style_mut().cursor = Some("no-drop");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: grab;
    /// ```
    fn cursor_grab(mut self) -> Self {
        self.style_mut().cursor = Some("grab");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: grabbing;
    /// ```
    fn cursor_grabbing(mut self) -> Self {
        self.style_mut().cursor = Some("grabbing");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: all-scroll;
    /// ```
    fn cursor_all_scroll(mut self) -> Self {
        self.style_mut().cursor = Some("all-scroll");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: col-resize;
    /// ```
    fn cursor_col_resize(mut self) -> Self {
        self.style_mut().cursor = Some("col-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: row-resize;
    /// ```
    fn cursor_row_resize(mut self) -> Self {
        self.style_mut().cursor = Some("row-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: n-resize;
    /// ```
    fn cursor_n_resize(mut self) -> Self {
        self.style_mut().cursor = Some("n-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: e-resize;
    /// ```
    fn cursor_e_resize(mut self) -> Self {
        self.style_mut().cursor = Some("e-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: s-resize;
    /// ```
    fn cursor_s_resize(mut self) -> Self {
        self.style_mut().cursor = Some("s-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: w-resize;
    /// ```
    fn cursor_w_resize(mut self) -> Self {
        self.style_mut().cursor = Some("w-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: ne-resize;
    /// ```
    fn cursor_ne_resize(mut self) -> Self {
        self.style_mut().cursor = Some("ne-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: nw-resize;
    /// ```
    fn cursor_nw_resize(mut self) -> Self {
        self.style_mut().cursor = Some("nw-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: se-resize;
    /// ```
    fn cursor_se_resize(mut self) -> Self {
        self.style_mut().cursor = Some("se-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: sw-resize;
    /// ```
    fn cursor_sw_resize(mut self) -> Self {
        self.style_mut().cursor = Some("sw-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: ew-resize;
    /// ```
    fn cursor_ew_resize(mut self) -> Self {
        self.style_mut().cursor = Some("ew-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: ns-resize;
    /// ```
    fn cursor_ns_resize(mut self) -> Self {
        self.style_mut().cursor = Some("ns-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: nesw-resize;
    /// ```
    fn cursor_nesw_resize(mut self) -> Self {
        self.style_mut().cursor = Some("nesw-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: nwse-resize;
    /// ```
    fn cursor_nwse_resize(mut self) -> Self {
        self.style_mut().cursor = Some("nwse-resize");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: zoom-in;
    /// ```
    fn cursor_zoom_in(mut self) -> Self {
        self.style_mut().cursor = Some("zoom-in");
        self
    }

    /// Set the cursor style.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/cursor>
    /// ```css
    /// cursor: zoom-out;
    /// ```
    fn cursor_zoom_out(mut self) -> Self {
        self.style_mut().cursor = Some("zoom-out");
        self
    }

    /// Set the decoration of text.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-line>
    /// ```css
    /// text-decoration-line: underline;
    /// ```
    fn underline(mut self) -> Self {
        self.style_mut().decoration = Some("underline");
        self
    }

    /// Set the decoration of text.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-line>
    /// ```css
    /// text-decoration-line: overline;
    /// ```
    fn overline(mut self) -> Self {
        self.style_mut().decoration = Some("overline");
        self
    }

    /// Set the decoration of text.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-line>
    /// ```css
    /// text-decoration-line: line-through;
    /// ```
    fn line_through(mut self) -> Self {
        self.style_mut().decoration = Some("line-through");
        self
    }

    /// Set the decoration of text.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-line>
    /// ```css
    /// text-decoration-line: none;
    /// ```
    fn no_underline(mut self) -> Self {
        self.style_mut().decoration = Some("none");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: block;
    /// ```
    fn block(mut self) -> Self {
        self.style_mut().display = Some("block");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: inline-block;
    /// ```
    fn inline_block(mut self) -> Self {
        self.style_mut().display = Some("inline-block");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: inline;
    /// ```
    fn inline(mut self) -> Self {
        self.style_mut().display = Some("inline");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: flex;
    /// ```
    fn flex(mut self) -> Self {
        self.style_mut().display = Some("flex");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: inline-flex;
    /// ```
    fn inline_flex(mut self) -> Self {
        self.style_mut().display = Some("inline-flex");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table;
    /// ```
    fn table(mut self) -> Self {
        self.style_mut().display = Some("table");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: inline-table;
    /// ```
    fn inline_table(mut self) -> Self {
        self.style_mut().display = Some("inline-table");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-caption;
    /// ```
    fn table_caption(mut self) -> Self {
        self.style_mut().display = Some("table-caption");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-cell;
    /// ```
    fn table_cell(mut self) -> Self {
        self.style_mut().display = Some("table-cell");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-column;
    /// ```
    fn table_column(mut self) -> Self {
        self.style_mut().display = Some("table-column");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-column-group;
    /// ```
    fn table_column_group(mut self) -> Self {
        self.style_mut().display = Some("table-column-group");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-footer-group;
    /// ```
    fn table_footer_group(mut self) -> Self {
        self.style_mut().display = Some("table-footer-group");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-header-group;
    /// ```
    fn table_header_group(mut self) -> Self {
        self.style_mut().display = Some("table-header-group");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-row-group;
    /// ```
    fn table_row_group(mut self) -> Self {
        self.style_mut().display = Some("table-row-group");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: table-row;
    /// ```
    fn table_row(mut self) -> Self {
        self.style_mut().display = Some("table-row");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: flow-root;
    /// ```
    fn flow_root(mut self) -> Self {
        self.style_mut().display = Some("flow-root");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: grid;
    /// ```
    fn grid(mut self) -> Self {
        self.style_mut().display = Some("grid");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: inline-grid;
    /// ```
    fn inline_grid(mut self) -> Self {
        self.style_mut().display = Some("inline-grid");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: contents;
    /// ```
    fn contents(mut self) -> Self {
        self.style_mut().display = Some("contents");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: list-item;
    /// ```
    fn list_item(mut self) -> Self {
        self.style_mut().display = Some("list-item");
        self
    }

    /// Set whether an element is treated as a block or inline element and the layout used for its
    /// children, such as flow layout, grid or flex.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    /// ```css
    /// display: none;
    /// ```
    fn hidden(mut self) -> Self {
        self.style_mut().display = Some("none");
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

    /// Set the easing function of CSS transitions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function>
    /// ```css
    /// transition-timing-function: {func};
    /// ```
    fn ease_custom(mut self, func: &'static str) -> Self {
        self.style_mut().transition_timing_function = Some(func);
        self
    }

    /// Set the easing function of CSS transitions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function>
    /// ```css
    /// transition-timing-function: linear;
    /// ```
    fn ease_linear(mut self) -> Self {
        self.style_mut().transition_timing_function = Some("linear");
        self
    }

    /// Set the easing function of CSS transitions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function>
    /// ```css
    /// transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
    /// ```
    fn ease_in(mut self) -> Self {
        self.style_mut().transition_timing_function = Some("cubic-bezier(0.4, 0, 1, 1)");
        self
    }

    /// Set the easing function of CSS transitions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function>
    /// ```css
    /// transition-timing-function: cubic-bezier(0, 0, 0.2, 1);
    /// ```
    fn ease_out(mut self) -> Self {
        self.style_mut().transition_timing_function = Some("cubic-bezier(0, 0, 0.2, 1)");
        self
    }

    /// Set the easing function of CSS transitions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function>
    /// ```css
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// ```
    fn ease_in_out(mut self) -> Self {
        self.style_mut().transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        self
    }

    /// Set the logical inline end position of a positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset-inline-end>
    /// ```css
    /// inset-inline-end: {x};
    /// ```
    #[with_auto]
    #[with_full]
    fn end(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().inset_inline_end = Some(x.into());
        self
    }

    /// ```css
    /// flex-grow: 1;
    /// ```
    fn flex_grow(mut self) -> Self {
        self.style_mut().flex_grow = Some(true);
        self
    }

    /// ```css
    /// flex-grow: 0;
    /// ```
    fn flex_no_grow(mut self) -> Self {
        self.style_mut().flex_grow = Some(false);
        self
    }

    /// ```css
    /// flex-shrink: 1;
    /// ```
    fn flex_shrink(mut self) -> Self {
        self.style_mut().flex_shrink = Some(true);
        self
    }

    /// ```css
    /// flex-shrink: 0;
    /// ```
    fn flex_no_shrink(mut self) -> Self {
        self.style_mut().flex_shrink = Some(false);
        self
    }

    /// ```css
    /// flex-wrap: wrap;
    /// ```
    fn flex_wrap(mut self) -> Self {
        self.style_mut().flex_wrap = Some("wrap");
        self
    }

    /// ```css
    /// flex-wrap: wrap-reverse;
    /// ```
    fn flex_wrap_reverse(mut self) -> Self {
        self.style_mut().flex_wrap = Some("wrap-reverse");
        self
    }

    /// ```css
    /// flex-wrap: nowrap;
    /// ```
    fn flex_nowrap(mut self) -> Self {
        self.style_mut().flex_wrap = Some("nowrap");
        self
    }

    /// Control direction of flex items (`flex-direction`).
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    /// ```css
    /// flex-direction: row;
    /// ```
    fn flex_row(mut self) -> Self {
        self.style_mut().flex_direction = Some("row");
        self
    }

    /// Control direction of flex items (`flex-direction`).
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    /// ```css
    /// flex-direction: row-reverse;
    /// ```
    fn flex_row_reverse(mut self) -> Self {
        self.style_mut().flex_direction = Some("row-reverse");
        self
    }

    /// Control direction of flex items (`flex-direction`).
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    /// ```css
    /// flex-direction: column;
    /// ```
    fn flex_col(mut self) -> Self {
        self.style_mut().flex_direction = Some("column");
        self
    }

    /// Control direction of flex items (`flex-direction`).
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    /// ```css
    /// flex-direction: column-reverse;
    /// ```
    fn flex_col_reverse(mut self) -> Self {
        self.style_mut().flex_direction = Some("column-reverse");
        self
    }

    /// Set a custom font family.
    fn font_family(mut self, family: &'static str) -> Self {
        self.style_mut().font_family = Some(family);
        self
    }

    /// ```css
    /// font-style: italic;
    /// ```
    fn italic(mut self) -> Self {
        self.style_mut().font_style = Some("italic");
        self
    }

    /// ```css
    /// font-style: normal;
    /// ```
    fn not_italic(mut self) -> Self {
        self.style_mut().font_style = Some("normal");
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 100;
    /// ```
    fn font_thin(mut self) -> Self {
        self.style_mut().font_weight = Some(100);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 200;
    /// ```
    fn font_extralight(mut self) -> Self {
        self.style_mut().font_weight = Some(200);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 300;
    /// ```
    fn font_light(mut self) -> Self {
        self.style_mut().font_weight = Some(300);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 400;
    /// ```
    fn font_normal(mut self) -> Self {
        self.style_mut().font_weight = Some(400);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 500;
    /// ```
    fn font_medium(mut self) -> Self {
        self.style_mut().font_weight = Some(500);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 600;
    /// ```
    fn font_semibold(mut self) -> Self {
        self.style_mut().font_weight = Some(600);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 700;
    /// ```
    fn font_bold(mut self) -> Self {
        self.style_mut().font_weight = Some(700);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 800;
    /// ```
    fn font_extrabold(mut self) -> Self {
        self.style_mut().font_weight = Some(800);
        self
    }

    /// Set the weight (or boldness) of the font.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-weight>
    /// ```css
    /// font-weight: 900;
    /// ```
    fn font_black(mut self) -> Self {
        self.style_mut().font_weight = Some(900);
        self
    }

    /// Set gradient from color.
    /// ```css
    /// background-image: linear-gradient(..., {color}, ...);
    /// ```
    fn from(mut self, color: &'static str) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .from_color = Some(color);
        self
    }

    /// Set gradient from position in percent.
    /// ```css
    /// background-image: linear-gradient(..., from_color {x}, ...);
    /// ```
    fn from_position(mut self, x: i16) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .from_position = Some(Length::Percent(f32::from(x)));
        self
    }

    /// Set gradient from position in percent.
    /// ```css
    /// background-image: linear-gradient(..., from_color {x}, ...);
    /// ```
    fn from_positionf(mut self, x: f32) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .from_position = Some(Length::Percent(x));
        self
    }

    /// Set the gaps (gutters) between rows and columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>
    /// ```css
    /// gap: {x};
    /// ```
    fn gap(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().gap.get_or_insert_default().set(x.into());
        self
    }

    /// Set the gaps (gutters) between columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>
    /// ```css
    /// column-gap: {x};
    /// ```
    fn gap_x(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().gap.get_or_insert_default().set(x.into());
        self
    }

    /// Set the gaps (gutters) between rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/row-gap>
    /// ```css
    /// row-gap: {x};
    /// ```
    fn gap_y(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().gap.get_or_insert_default().set(x.into());
        self
    }

    /// Set equally sized grid columns.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
    /// ```css
    /// grid-template-columns: repeat({n}, minmax(0, 1fr));
    /// ```
    fn cols(mut self, n: u16) -> Self {
        self.style_mut().grid_template_columns = Some(Either::Left(TrackRepeatEqually(n)));
        self
    }

    /// Indicate that there is no explicit grid.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
    /// ```css
    /// grid-template-columns: none;
    /// ```
    fn cols_none(mut self) -> Self {
        self.style_mut().grid_template_columns = Some(Either::Right("none"));
        self
    }

    /// Set grid columns template.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-columns>
    /// ```css
    /// grid-template-columns: {template};
    /// ```
    fn cols_custom(mut self, template: &'static str) -> Self {
        self.style_mut().grid_template_columns = Some(Either::Right(template));
        self
    }

    /// Set equally sized grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>
    /// ```css
    /// grid-template-rows: repeat({n}, minmax(0, 1fr));
    /// ```
    fn rows(mut self, n: u16) -> Self {
        self.style_mut().grid_template_rows = Some(Either::Left(TrackRepeatEqually(n)));
        self
    }

    /// Indicate that there is no explicit grid.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>
    /// ```css
    /// grid-template-rows: none;
    /// ```
    fn rows_none(mut self) -> Self {
        self.style_mut().grid_template_rows = Some(Either::Right("none"));
        self
    }

    /// Set grid rows template.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-rows>
    /// ```css
    /// grid-template-rows: {template};
    /// ```
    fn rows_custom(mut self, template: &'static str) -> Self {
        self.style_mut().grid_template_rows = Some(Either::Right(template));
        self
    }

    /// Set how elements in a grid are auto-placed.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    /// ```css
    /// grid-auto-flow: row;
    /// ```
    fn grid_flow_row(mut self) -> Self {
        self.style_mut().grid_auto_flow = Some("row");
        self
    }

    /// Set how elements in a grid are auto-placed.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    /// ```css
    /// grid-auto-flow: column;
    /// ```
    fn grid_flow_col(mut self) -> Self {
        self.style_mut().grid_auto_flow = Some("column");
        self
    }

    /// Set how elements in a grid are auto-placed.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    /// ```css
    /// grid-auto-flow: dense;
    /// ```
    fn grid_flow_dense(mut self) -> Self {
        self.style_mut().grid_auto_flow = Some("dense");
        self
    }

    /// Set how elements in a grid are auto-placed.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    /// ```css
    /// grid-auto-flow: row dense;
    /// ```
    fn grid_flow_row_dense(mut self) -> Self {
        self.style_mut().grid_auto_flow = Some("row dense");
        self
    }

    /// Set how elements in a grid are auto-placed.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>
    /// ```css
    /// grid-auto-flow: column dense;
    /// ```
    fn grid_flow_col_dense(mut self) -> Self {
        self.style_mut().grid_auto_flow = Some("column dense");
        self
    }

    /// Set the element's height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/height>
    /// ```css
    /// height: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_vertical]
    #[with_vertical_viewport_units]
    #[with_content]
    fn h(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().height = Some(x.into());
        self
    }

    /// Set horizontal and vertical position of a positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset>
    /// ```css
    /// inset: {x};
    /// ```
    #[with_auto]
    fn inset(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().inset.get_or_insert_default().set(x.into());
        self
    }

    /// Set horizontal position of a positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right>
    /// ```css
    /// left: {x};
    /// right: {x};
    /// ```
    #[with_auto]
    #[with_horizontal_viewport_units]
    fn inset_x(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .inset
            .get_or_insert_default()
            .set_x(x.into());
        self
    }

    /// Set vertical position of a positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>
    /// ```css
    /// left: {x};
    /// right: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_vertical_viewport_units]
    fn inset_y(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .inset
            .get_or_insert_default()
            .set_y(x.into());
        self
    }

    /// In Flexbox, control the alignment of items on the Cross Axis. In Grid Layout, controlc the
    /// alignment of items on the Block Axis within their grid area.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    /// ```css
    /// align-items: flex-start;
    /// ```
    fn items_start(mut self) -> Self {
        self.style_mut().align_items = Some("flex-start");
        self
    }

    /// In Flexbox, control the alignment of items on the Cross Axis. In Grid Layout, controlc the
    /// alignment of items on the Block Axis within their grid area.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    /// ```css
    /// align-items: flex-end;
    /// ```
    fn items_end(mut self) -> Self {
        self.style_mut().align_items = Some("flex-end");
        self
    }

    /// In Flexbox, control the alignment of items on the Cross Axis. In Grid Layout, controlc the
    /// alignment of items on the Block Axis within their grid area.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    /// ```css
    /// align-items: center
    /// ```
    fn items_center(mut self) -> Self {
        self.style_mut().align_items = Some("center");
        self
    }

    /// In Flexbox, control the alignment of items on the Cross Axis. In Grid Layout, controlc the
    /// alignment of items on the Block Axis within their grid area.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    /// ```css
    /// align-items: baseline;
    /// ```
    fn items_baseline(mut self) -> Self {
        self.style_mut().align_items = Some("baseline");
        self
    }

    /// In Flexbox, control the alignment of items on the Cross Axis. In Grid Layout, controlc the
    /// alignment of items on the Block Axis within their grid area.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    /// ```css
    /// align-items: stretch;
    /// ```
    fn items_stretch(mut self) -> Self {
        self.style_mut().align_items = Some("stretch");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: normal;
    /// ```
    fn justify_normal(mut self) -> Self {
        self.style_mut().justify_content = Some("normal");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: flex-start;
    /// ```
    fn justify_start(mut self) -> Self {
        self.style_mut().justify_content = Some("flex-start");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: flex-end;
    /// ```
    fn justify_end(mut self) -> Self {
        self.style_mut().justify_content = Some("flex-end");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: center;
    /// ```
    fn justify_center(mut self) -> Self {
        self.style_mut().justify_content = Some("center");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: space-between;
    /// ```
    fn justify_between(mut self) -> Self {
        self.style_mut().justify_content = Some("space-between");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: space-around;
    /// ```
    fn justify_around(mut self) -> Self {
        self.style_mut().justify_content = Some("space-around");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: space-evenly;
    /// ```
    fn justify_evenly(mut self) -> Self {
        self.style_mut().justify_content = Some("space-evenly");
        self
    }

    /// Define how to distribute space between and around content items along the main-axis of a flex
    /// container, and the inline axis of a grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    /// ```css
    /// justify-content: stretch;
    /// ```
    fn justify_stretch(mut self) -> Self {
        self.style_mut().justify_content = Some("stretch");
        self
    }

    /// Define how to justify a box inside its alignment container along the appropriate axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    /// ```css
    /// justify-self: auto;
    /// ```
    fn justify_self_auto(mut self) -> Self {
        self.style_mut().justify_self = Some("auto");
        self
    }

    /// Define how to justify a box inside its alignment container along the appropriate axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    /// ```css
    /// justify-self: start;
    /// ```
    fn justify_self_start(mut self) -> Self {
        self.style_mut().justify_self = Some("start");
        self
    }

    /// Define how to justify a box inside its alignment container along the appropriate axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    /// ```css
    /// justify-self: end;
    /// ```
    fn justify_self_end(mut self) -> Self {
        self.style_mut().justify_self = Some("end");
        self
    }

    /// Define how to justify a box inside its alignment container along the appropriate axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    /// ```css
    /// justify-self: center;
    /// ```
    fn justify_self_center(mut self) -> Self {
        self.style_mut().justify_self = Some("center");
        self
    }

    /// Define how to justify a box inside its alignment container along the appropriate axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    /// ```css
    /// justify-self: stretch;
    /// ```
    fn justify_self_stretch(mut self) -> Self {
        self.style_mut().justify_self = Some("stretch");
        self
    }
    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: 1;
    /// ```
    fn leading_none(mut self) -> Self {
        self.style_mut().line_height = Some(Either::Right(1.0));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: 1.25;
    /// ```
    fn leading_tight(mut self) -> Self {
        self.style_mut().line_height = Some(Either::Right(1.25));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: 1.375;
    /// ```
    fn leading_snug(mut self) -> Self {
        self.style_mut().line_height = Some(Either::Right(1.375));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: 1.5;
    /// ```
    fn leading_normal(mut self) -> Self {
        self.style_mut().line_height = Some(Either::Right(1.5));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: 1.625;
    /// ```
    fn leading_relaxed(mut self) -> Self {
        self.style_mut().line_height = Some(Either::Right(1.625));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: 2;
    /// ```
    fn leading_loose(mut self) -> Self {
        self.style_mut().line_height = Some(Either::Right(2.0));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: {x};
    /// ```
    fn leading_multiple(mut self, x: f32) -> Self {
        self.style_mut().line_height = Some(Either::Right(x));
        self
    }

    /// Set the element's line height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
    /// ```css
    /// line-height: {x};
    /// ```
    #[without_zero]
    fn leading(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().line_height = Some(Either::Left(x.into()));
        self
    }

    /// Set the left position of the positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
    /// ```css
    /// left: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_horizontal]
    #[with_horizontal_viewport_units]
    fn left(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .inset
            .get_or_insert_default()
            .set_left(x.into());
        self
    }

    /// Clamping text to a specific number of lines.
    /// ```css
    /// overflow: hidden;
    /// display: -webkit-box;
    /// -webkit-box-orient: vertical;
    /// -webkit-line-clamp: {n};
    /// ```
    fn line_clamp(mut self, n: u16) -> Self {
        self.style_mut().line_clamp = Some(LineClamp::Lines(n));
        self
    }

    /// Undo a previously applied line clamp utility.
    /// ```css
    /// overflow: visible;
    /// display: block;
    /// -webkit-box-orient: horizontal;
    /// -webkit-line-clamp: none;
    /// ```
    fn line_clamp_none(mut self) -> Self {
        self.style_mut().line_clamp = None;
        self
    }

    /// Set the bullet/number style of a list.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type>
    /// ```css
    /// list-style-type: none;
    /// ```
    fn list_none(mut self) -> Self {
        self.style_mut().list_style_type = Some("none");
        self
    }

    /// Set the bullet/number style of a list.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type>
    /// ```css
    /// list-style-type: disc;
    /// ```
    fn list_disc(mut self) -> Self {
        self.style_mut().list_style_type = Some("disc");
        self
    }

    /// Set the bullet/number style of a list.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type>
    /// ```css
    /// list-style-type: decimnal;
    /// ```
    fn list_decimal(mut self) -> Self {
        self.style_mut().list_style_type = Some("decimnal");
        self
    }

    /// Set the margin area.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>
    /// ```css
    /// margin: {x};
    /// ```
    fn m(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin
            .get_or_insert_default()
            .set(x.into());
        self
    }

    /// Set the margin area on logical inline start end end.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-start>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-end>
    /// ```css
    /// margin-inline-start: {x};
    /// margin-inline-end: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn mx(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin_inline
            .get_or_insert_default()
            .set_x(x.into());
        self
    }

    /// Set the margin area on logical block start end end.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-block-start>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-block-end>
    /// ```css
    /// margin-block-start: {x};
    /// margin-block-end: {x};
    /// ```
    #[with_vertical_viewport_units]
    fn my(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin_inline
            .get_or_insert_default()
            .set_y(x.into());
        self
    }

    /// Set the margin area on the top.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-top>
    /// ```css
    /// margin-top: {x};
    /// ```
    #[with_vertical_viewport_units]
    fn mt(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin
            .get_or_insert_default()
            .set_top(x.into());
        self
    }

    /// Set the margin area on the right.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-right>
    /// ```css
    /// margin-right: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn mr(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin
            .get_or_insert_default()
            .set_right(x.into());
        self
    }

    /// Set the margin area on the bottom.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-bottom>
    /// ```css
    /// margin-bottom: {x};
    /// ```
    #[with_vertical_viewport_units]
    fn mb(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin
            .get_or_insert_default()
            .set_bottom(x.into());
        self
    }

    /// Set the margin area on the left.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-left>
    /// ```css
    /// margin-left: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn ml(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin
            .get_or_insert_default()
            .set_left(x.into());
        self
    }

    /// Set the margin area on the logical inline start.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-start>
    /// ```css
    /// margin-inline-start: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn ms(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin_inline
            .get_or_insert_default()
            .set_inline_start(x.into());
        self
    }

    /// Set the margin area on the logical inline end.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-end>
    /// ```css
    /// margin-inline-end: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn me(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .margin_inline
            .get_or_insert_default()
            .set_inline_end(x.into());
        self
    }

    /// Set the element's maximum height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>
    /// ```css
    /// max-height: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_vertical]
    #[with_vertical_viewport_units]
    #[with_content]
    fn max_h(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().max_height = Some(x.into());
        self
    }

    /// Set the element's maximum width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>
    /// ```css
    /// max-width: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_horizontal]
    #[with_horizontal_viewport_units]
    #[with_content]
    fn max_w(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().max_width = Some(x.into());
        self
    }

    /// Set the element's minimum height.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>
    /// ```css
    /// min-height: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_vertical]
    #[with_vertical_viewport_units]
    #[with_content]
    fn min_h(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().min_height = Some(x.into());
        self
    }

    /// Set the element's minimum width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>
    /// ```css
    /// min-width: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_horizontal]
    #[with_horizontal_viewport_units]
    #[with_content]
    fn min_w(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().min_width = Some(x.into());
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: bottom;
    /// ```
    fn object_block(mut self) -> Self {
        self.style_mut().object_position = Some("bottom");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: center;
    /// ```
    fn object_center(mut self) -> Self {
        self.style_mut().object_position = Some("center");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: left;
    /// ```
    fn object_left(mut self) -> Self {
        self.style_mut().object_position = Some("left");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: left bottom;
    /// ```
    fn object_left_bottom(mut self) -> Self {
        self.style_mut().object_position = Some("left bottom");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: left top;
    /// ```
    fn object_left_top(mut self) -> Self {
        self.style_mut().object_position = Some("left top");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: right;
    /// ```
    fn object_right(mut self) -> Self {
        self.style_mut().object_position = Some("right");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: right bottom;
    /// ```
    fn object_right_bottom(mut self) -> Self {
        self.style_mut().object_position = Some("right bottom");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: right top;
    /// ```
    fn object_right_top(mut self) -> Self {
        self.style_mut().object_position = Some("right top");
        self
    }

    /// Set how a replaced element's content should be positioned within its container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
    /// ```css
    /// object-position: top;
    /// ```
    fn object_top(mut self) -> Self {
        self.style_mut().object_position = Some("top");
        self
    }

    /// Set how a replaced element's content should be resized.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit>
    /// ```css
    /// object-fit: contain;
    /// ```
    fn object_contain(mut self) -> Self {
        self.style_mut().object_fit = Some("contain");
        self
    }

    /// Set how a replaced element's content should be resized.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit>
    /// ```css
    /// object-fit: cover;
    /// ```
    fn object_cover(mut self) -> Self {
        self.style_mut().object_fit = Some("cover");
        self
    }

    /// Set how a replaced element's content should be resized.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit>
    /// ```css
    /// object-fit: fill;
    /// ```
    fn object_fill(mut self) -> Self {
        self.style_mut().object_fit = Some("fill");
        self
    }

    /// Set how a replaced element's content should be resized.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit>
    /// ```css
    /// object-fit: none;
    /// ```
    fn object_none(mut self) -> Self {
        self.style_mut().object_fit = Some("none");
        self
    }

    /// Set how a replaced element's content should be resized.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit>
    /// ```css
    /// object-fit: scale-down;
    /// ```
    fn object_scale_down(mut self) -> Self {
        self.style_mut().object_fit = Some("scale-down");
        self
    }

    /// The order to lay out an item in a flex or grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/order>
    /// ```css
    /// order: {n};
    /// ```
    fn order(mut self, n: u16) -> Self {
        self.style_mut().order = Some(i32::from(n));
        self
    }

    /// The order to lay out an item in a flex or grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/order>
    /// ```css
    /// order: -9999;
    /// ```
    fn order_first(mut self) -> Self {
        self.style_mut().order = Some(-9999);
        self
    }

    /// The order to lay out an item in a flex or grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/order>
    /// ```css
    /// order: 9999;
    /// ```
    fn order_last(mut self) -> Self {
        self.style_mut().order = Some(9999);
        self
    }

    /// The order to lay out an item in a flex or grid container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/order>
    /// ```css
    /// order: 0;
    /// ```
    fn order_none(mut self) -> Self {
        self.style_mut().order = Some(0);
        self
    }

    /// ```css
    /// outline-width: {x}px;
    /// ```
    fn outline(mut self, x: i16) -> Self {
        let style = self.style_mut();
        style.outline_width = Some(Length::Px(f32::from(x)));
        if style.outline_style.is_none() {
            self.outline_solid()
        } else {
            self
        }
    }

    /// ```css
    /// outline-width: {x}px;
    /// ```
    fn outlinef(mut self, x: f32) -> Self {
        let style = self.style_mut();
        style.outline_width = Some(Length::Px(x));
        if style.outline_style.is_none() {
            self.outline_solid()
        } else {
            self
        }
    }

    /// Hide the default browser outline on focused elements.
    /// ```css
    /// outline: 2px solid transparent;
    /// outline-offset: 2px;
    /// ```
    fn outline_hidden(mut self) -> Self {
        let style = self.style_mut();
        style.outline_width = Some(Length::Px(2.0));
        style.outline_style = Some("solid");
        style.outline_color = Some("transparent");
        style.outline_offset = Some(Length::Px(2.0));
        self
    }

    /// Control the style of an element's outline.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>
    /// ```css
    /// outline-style: none;
    /// ```
    fn outline_none(mut self) -> Self {
        self.style_mut().outline_style = Some("none");
        self
    }

    /// Control the style of an element's outline.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>
    /// ```css
    /// outline-style: solid;
    /// ```
    fn outline_solid(mut self) -> Self {
        self.style_mut().outline_style = Some("solid");
        self
    }

    /// Control the style of an element's outline.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>
    /// ```css
    /// outline-style: dashed;
    /// ```
    fn outline_dashed(mut self) -> Self {
        self.style_mut().outline_style = Some("dashed");
        self
    }

    /// Control the style of an element's outline.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>
    /// ```css
    /// outline-style: dotted;
    /// ```
    fn outline_dotted(mut self) -> Self {
        self.style_mut().outline_style = Some("dotted");
        self
    }

    /// Control the style of an element's outline.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>
    /// ```css
    /// outline-style: double;
    /// ```
    fn outline_double(mut self) -> Self {
        self.style_mut().outline_style = Some("double");
        self
    }

    /// ```css
    /// outline-color: {color};
    /// ```
    fn outline_color(mut self, color: &'static str) -> Self {
        self.style_mut().outline_color = Some(color);
        self
    }

    /// ```css
    /// outline-offset: {x}px;
    /// ```
    fn outline_offset(mut self, x: i16) -> Self {
        self.style_mut().outline_offset = Some(Length::Px(f32::from(x)));
        self
    }

    /// ```css
    /// outline-offset: {x}px;
    /// ```
    fn outline_offsetf(mut self, x: f32) -> Self {
        self.style_mut().outline_offset = Some(Length::Px(x));
        self
    }

    /// Set how an element handles content that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    /// ```css
    /// overflow: auto;
    /// ```
    fn overflow_auto(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set("auto");
        self
    }

    /// Set how an element handles content that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    /// ```css
    /// overflow: hidden;
    /// ```
    fn overflow_hidden(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set("hidden");
        self
    }

    /// Set how an element handles content that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    /// ```css
    /// overflow: clip;
    /// ```
    fn overflow_clip(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set("clip");
        self
    }

    /// Set how an element handles content that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    /// ```css
    /// overflow: visible;
    /// ```
    fn overflow_visible(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set("visible");
        self
    }

    /// Set how an element handles content that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    /// ```css
    /// overflow: scroll;
    /// ```
    fn overflow_scroll(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set("scroll");
        self
    }

    /// Set how an element handles content horizontally that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-x>
    /// ```css
    /// overflow-x: auto;
    /// ```
    fn overflow_x_auto(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_x("auto");
        self
    }

    /// Set how an element handles content horizontally that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-x>
    /// ```css
    /// overflow-x: hidden;
    /// ```
    fn overflow_x_hidden(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_x("hidden");
        self
    }

    /// Set how an element handles content horizontally that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-x>
    /// ```css
    /// overflow-x: clip;
    /// ```
    fn overflow_x_clip(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_x("clip");
        self
    }

    /// Set how an element handles content horizontally that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-x>
    /// ```css
    /// overflow-x: visible;
    /// ```
    fn overflow_x_visible(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_x("visible");
        self
    }

    /// Set how an element handles content horizontally that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-x>
    /// ```css
    /// overflow-x: scroll;
    /// ```
    fn overflow_x_scroll(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_x("scroll");
        self
    }

    /// Set how an element handles content vertically that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-y>
    /// ```css
    /// overflow-y: auto;
    /// ```
    fn overflow_y_auto(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_y("auto");
        self
    }

    /// Set how an element handles content vertically that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-y>
    /// ```css
    /// overflow-y: hidden;
    /// ```
    fn overflow_y_hidden(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_y("hidden");
        self
    }

    /// Set how an element handles content vertically that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-y>
    /// ```css
    /// overflow-y: clip;
    /// ```
    fn overflow_y_clip(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_y("clip");
        self
    }

    /// Set how an element handles content vertically that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-y>
    /// ```css
    /// overflow-y: visible;
    /// ```
    fn overflow_y_visible(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_y("visible");
        self
    }

    /// Set how an element handles content vertically that is too large for the container.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-y>
    /// ```css
    /// overflow-y: scroll;
    /// ```
    fn overflow_y_scroll(mut self) -> Self {
        self.style_mut()
            .overflow
            .get_or_insert_default()
            .set_y("scroll");
        self
    }

    /// Set the padding area on all four sides.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>
    /// ```css
    /// padding: {x};
    /// ```
    fn p(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set(x.into());
        self
    }

    /// Set the padding area on the left and right.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-left>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-right>
    /// ```css
    /// padding-right: {x};
    /// padding-left: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn px(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set_x(x.into());
        self
    }

    /// Set the padding area on the top and bottom.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-top>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-bottom>
    /// ```css
    /// padding-top: {x};
    /// padding-bottom: {x};
    /// ```
    #[with_vertical_viewport_units]
    fn py(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set_y(x.into());
        self
    }

    /// Set the padding area on the top.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-top>
    /// ```css
    /// padding-top: {x};
    /// ```
    #[with_vertical_viewport_units]
    fn pt(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set_top(x.into());
        self
    }

    /// Set the padding area on the right.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-right>
    /// ```css
    /// padding-right: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn pr(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set_right(x.into());
        self
    }

    /// Set the padding area on the bottom.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-bottom>
    /// ```css
    /// padding-bottom: {x};
    /// ```
    #[with_vertical_viewport_units]
    fn pb(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set_bottom(x.into());
        self
    }

    /// Set the padding area on the left.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-left>
    /// ```css
    /// padding-left: {x};
    /// ```
    #[with_horizontal_viewport_units]
    fn pl(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .padding
            .get_or_insert_default()
            .set_left(x.into());
        self
    }

    /// Set how an individual item is justified and aligned at the same time.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self>
    /// ```css
    /// place-self: auto;
    /// ```
    fn place_auto(mut self) -> Self {
        self.style_mut().place_self = Some("auto");
        self
    }

    /// Set how an individual item is justified and aligned at the same time.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self>
    /// ```css
    /// place-self: start;
    /// ```
    fn place_start(mut self) -> Self {
        self.style_mut().place_self = Some("start");
        self
    }

    /// Set how an individual item is justified and aligned at the same time.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self>
    /// ```css
    /// place-self: end;
    /// ```
    fn place_end(mut self) -> Self {
        self.style_mut().place_self = Some("end");
        self
    }

    /// Set how an individual item is justified and aligned at the same time.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self>
    /// ```css
    /// place-self: center;
    /// ```
    fn place_center(mut self) -> Self {
        self.style_mut().place_self = Some("center");
        self
    }

    /// Set how an individual item is justified and aligned at the same time.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self>
    /// ```css
    /// place-self: stretch;
    /// ```
    fn place_stretch(mut self) -> Self {
        self.style_mut().place_self = Some("stretch");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: center;
    /// ```
    fn place_content_center(mut self) -> Self {
        self.style_mut().place_content = Some("center");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: start;
    /// ```
    fn place_content_start(mut self) -> Self {
        self.style_mut().place_content = Some("start");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: end;
    /// ```
    fn place_content_end(mut self) -> Self {
        self.style_mut().place_content = Some("end");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: space-between;
    /// ```
    fn place_content_between(mut self) -> Self {
        self.style_mut().place_content = Some("space-between");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: space-around;
    /// ```
    fn place_content_around(mut self) -> Self {
        self.style_mut().place_content = Some("space-around");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: space-evenly;
    /// ```
    fn place_content_evenly(mut self) -> Self {
        self.style_mut().place_content = Some("space-evenly");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: baseline;
    /// ```
    fn place_content_baseline(mut self) -> Self {
        self.style_mut().place_content = Some("baseline");
        self
    }

    /// Set how content is aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>
    /// ```css
    /// place-content: stretch;
    /// ```
    fn place_content_stretch(mut self) -> Self {
        self.style_mut().place_content = Some("stretch");
        self
    }

    /// Set how items are aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items>
    /// ```css
    /// place-items: start;
    /// ```
    fn place_items_start(mut self) -> Self {
        self.style_mut().place_items = Some("start");
        self
    }

    /// Set how items are aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items>
    /// ```css
    /// place-items: end;
    /// ```
    fn place_items_end(mut self) -> Self {
        self.style_mut().place_items = Some("end");
        self
    }

    /// Set how items are aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items>
    /// ```css
    /// place-items: center;
    /// ```
    fn place_items_center(mut self) -> Self {
        self.style_mut().place_items = Some("center");
        self
    }

    /// Set how items are aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items>
    /// ```css
    /// place-items: baseline;
    /// ```
    fn place_items_baseline(mut self) -> Self {
        self.style_mut().place_items = Some("baseline");
        self
    }

    /// Set how items are aligned along both block and inline directions.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items>
    /// ```css
    /// place-items: stretch;
    /// ```
    fn place_items_stretch(mut self) -> Self {
        self.style_mut().place_items = Some("stretch");
        self
    }

    /// Set whether whether an element responds to pointer events.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events>
    /// ```css
    /// pointer-events: none;
    /// ```
    fn pointer_events_none(mut self) -> Self {
        self.style_mut().pointer_events = Some("none");
        self
    }

    /// Set whether whether an element responds to pointer events.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events>
    /// ```css
    /// pointer-events: auto;
    /// ```
    fn pointer_events_auto(mut self) -> Self {
        self.style_mut().pointer_events = Some("auto");
        self
    }

    /// Set how an element is positioned in a document.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    /// ```css
    /// position: static;
    /// ```
    fn static_(mut self) -> Self {
        self.style_mut().position = Some("static");
        self
    }

    /// Set how an element is positioned in a document.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    /// ```css
    /// position: relative;
    /// ```
    fn relative(mut self) -> Self {
        self.style_mut().position = Some("relative");
        self
    }

    /// Set how an element is positioned in a document.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    /// ```css
    /// position: absolute;
    /// ```
    fn absolute(mut self) -> Self {
        self.style_mut().position = Some("absolute");
        self
    }

    /// Set how an element is positioned in a document.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    /// ```css
    /// position: sticky;
    /// ```
    fn sticky(mut self) -> Self {
        self.style_mut().position = Some("sticky");
        self
    }

    /// Set how an element is positioned in a document.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    /// ```css
    /// position: fixed;
    /// ```
    fn fixed(mut self) -> Self {
        self.style_mut().position = Some("fixed");
        self
    }

    /// Set the right position of the positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right>
    /// ```css
    /// right: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_horizontal]
    #[with_horizontal_viewport_units]
    fn right(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .inset
            .get_or_insert_default()
            .set_right(x.into());
        self
    }

    /// Add a solid box-shadow.
    /// ```css
    /// box-shadow: 0 0 0 {x} ...;
    /// ```
    fn ring(mut self, x: i16) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .ring
            .get_or_insert_default()
            .width = Length::Px(f32::from(x));
        self
    }

    /// Add a solid box-shadow.
    /// ```css
    /// box-shadow: 0 0 0 {x} ...;
    /// ```
    fn ringf(mut self, x: f32) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .ring
            .get_or_insert_default()
            .width = Length::Px(x);
        self
    }

    /// Force a ring to render on the inside of an element instead of the outside.
    /// ```css
    /// box-shadow: inset ...;
    /// ```
    fn ring_inset(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .ring
            .get_or_insert_default()
            .inset = true;
        self
    }

    /// Set ring color.
    /// ```css
    /// box-shadow: ... {color};
    /// ```
    fn ring_color(mut self, color: &'static str) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .ring
            .get_or_insert_default()
            .color = color;
        self
    }

    /// Set rotation transforms.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>
    /// ```css
    /// transform: rotate({deg}deg);
    /// ```
    fn rotate(mut self, deg: i16) -> Self {
        self.style_mut().transform = Some(Transform::Rotate(deg));
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_xs2(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Xs2);
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_xs(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Xs);
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_sm(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Sm);
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_md(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Md);
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_lg(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Lg);
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_xl(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Xl);
        self
    }

    /// Add a shadow effects around an element's frame.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>
    fn shadow_xl2(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = Some(ShadowKind::Xl2);
        self
    }

    /// Remove any existing shadow effects.
    fn shadow_none(mut self) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_kind = None;
        self
    }

    /// Set the shadow's color.
    fn shadow_color(mut self, color: &'static str) -> Self {
        self.style_mut()
            .box_shadow
            .get_or_insert_default()
            .shadow_color = Some(color);
        self
    }

    /// Rounds the corners of an element's outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
    /// ```css
    /// border-radius: {x};
    /// ```
    fn rounded(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
    /// ```css
    /// border-radius: 0.25rem;
    /// ```
    fn rounded_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
    /// ```css
    /// border-radius: calc(infinity * 1px);
    /// ```
    fn rounded_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's top outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// ```css
    /// border-top-right-radius: {x};
    /// border-top-left-radius: {x};
    /// ```
    fn rounded_t(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_t(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's top outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// ```css
    /// border-top-right-radius: 0.25rem;
    /// border-top-left-radius: 0.25rem;
    /// ```
    fn rounded_t_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_t(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's top outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// ```css
    /// border-top-right-radius: calc(infinity * 1px);
    /// border-top-left-radius: calc(infinity * 1px);
    /// ```
    fn rounded_t_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_t(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// ```css
    /// border-top-right-radius: {x};
    /// border-bottom-right-radius: {x};
    /// ```
    fn rounded_r(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_r(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// ```css
    /// border-top-right-radius: 0.25rem;
    /// border-bottom-right-radius: 0.25rem;
    /// ```
    fn rounded_r_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_r(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// ```css
    /// border-top-right-radius: calc(infinity * 1px);
    /// border-bottom-right-radius: calc(infinity * 1px);
    /// ```
    fn rounded_r_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_r(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's bottom outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-bottom-right-radius: {x};
    /// border-bottom-left-radius: {x};
    /// ```
    fn rounded_b(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_b(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's bottom outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-bottom-right-radius: 0.25rem;
    /// border-bottom-left-radius: 0.25rem;
    /// ```
    fn rounded_b_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_b(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's bottom outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-bottom-right-radius: calc(infinity * 1px);
    /// border-bottom-left-radius: calc(infinity * 1px);
    /// ```
    fn rounded_b_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_b(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-top-left-radius: {x};
    /// border-bottom-left-radius: {x};
    /// ```
    fn rounded_l(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_l(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-top-left-radius: 0.25rem;
    /// border-bottom-left-radius: 0.25rem;
    /// ```
    fn rounded_l_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_l(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-top-left-radius: calc(infinity * 1px);
    /// border-bottom-left-radius: calc(infinity * 1px);
    /// ```
    fn rounded_l_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_l(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's top left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// ```css
    /// border-top-left-radius: {x};
    /// ```
    fn rounded_tl(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_tl(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's top left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// ```css
    /// border-top-left-radius: 0.25rem;
    /// ```
    fn rounded_tl_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_tl(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's top left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
    /// ```css
    /// border-top-left-radius: calc(infinity * 1px);
    /// ```
    fn rounded_tl_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_tl(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's top right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// ```css
    /// border-top-right-radius: {x};
    /// ```
    fn rounded_tr(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_tr(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's top right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// ```css
    /// border-top-right-radius: 0.25rem;
    /// ```
    fn rounded_tr_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_tr(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's top right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
    /// ```css
    /// border-top-right-radius: calc(infinity * 1px);
    /// ```
    fn rounded_tr_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_tr(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's bottom right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// ```css
    /// border-bottom-right-radius: {x};
    /// ```
    fn rounded_br(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_br(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's bottom right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// ```css
    /// border-bottom-right-radius: 0.25rem;
    /// ```
    fn rounded_br_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_br(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's bottom right outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>
    /// ```css
    /// border-bottom-right-radius: calc(infinity * 1px);
    /// ```
    fn rounded_br_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_br(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Rounds the corners of an element's bottom left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-bottom-left-radius: {x};
    /// ```
    fn rounded_bl(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_bl(Either::Left(x.into()));
        self
    }

    /// Rounds the corners of an element's bottom left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-bottom-left-radius: 0.25rem;
    /// ```
    fn rounded_bl_default(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_bl(Either::Left(Length::Rem(0.25)));
        self
    }

    /// Rounds the corners of an element's bottom left outer border edge.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>
    /// ```css
    /// border-bottom-left-radius: calc(infinity * 1px);
    /// ```
    fn rounded_bl_full(mut self) -> Self {
        self.style_mut()
            .border_radius
            .get_or_insert_default()
            .set_bl(Either::Right("calc(infinity * 1px)"));
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
    /// ```css
    /// grid-row: auto;
    /// ```
    fn row_auto(mut self) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set(GridLine::Auto);
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
    /// ```css
    /// grid-row: 1 / -1;
    /// ```
    fn row_full(mut self) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set_start(GridLine::Nth(1));
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set_end(GridLine::Nth(-1));
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
    /// ```css
    /// grid-row: {n};
    /// ```
    fn row(mut self, n: i16) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set(GridLine::Nth(n));
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>
    /// ```css
    /// grid-row: span {n} / span {n};
    /// ```
    fn row_span(mut self, n: u16) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set_start(GridLine::Span(n));
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set_end(GridLine::Span(n));
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-start>
    /// ```css
    /// grid-row-start: auto;
    /// ```
    fn row_start_auto(mut self) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set(GridLine::Auto);
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-start>
    /// ```css
    /// grid-row-start: {n};
    /// ```
    fn row_start(mut self, n: i16) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set_start(GridLine::Nth(n));
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-end>
    /// ```css
    /// grid-row-end: auto;
    /// ```
    fn row_end_auto(mut self) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set(GridLine::Auto);
        self
    }

    /// Set how elements are sized and placed across grid rows.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row-end>
    /// ```css
    /// grid-row-end: {n};
    /// ```
    fn row_end(mut self, n: i16) -> Self {
        self.style_mut()
            .grid_row
            .get_or_insert_default()
            .set_end(GridLine::Nth(n));
        self
    }

    /// Set whether an element's text can be selected.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/user-select>
    /// ```css
    /// user-select: none;
    /// ```
    fn select_none(mut self) -> Self {
        self.style_mut().user_select = Some("none");
        self
    }

    /// Set whether an element's text can be selected.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/user-select>
    /// ```css
    /// user-select: text;
    /// ```
    fn select_text(mut self) -> Self {
        self.style_mut().user_select = Some("text");
        self
    }

    /// Set whether an element's text can be selected.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/user-select>
    /// ```css
    /// user-select: all;
    /// ```
    fn select_all(mut self) -> Self {
        self.style_mut().user_select = Some("all");
        self
    }

    /// Set whether an element's text can be selected.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/user-select>
    /// ```css
    /// user-select: auto;
    /// ```
    fn select_auto(mut self) -> Self {
        self.style_mut().user_select = Some("auto");
        self
    }

    /// Define how to justify an invidvidual flex or grid item along its container's cross axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    /// ```css
    /// align-self: auto;
    /// ```
    fn self_auto(mut self) -> Self {
        self.style_mut().align_self = Some("auto");
        self
    }

    /// Define how to justify an invidvidual flex or grid item along its container's cross axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    /// ```css
    /// align-self: flex-start;
    /// ```
    fn self_start(mut self) -> Self {
        self.style_mut().align_self = Some("flex-start");
        self
    }

    /// Define how to justify an invidvidual flex or grid item along its container's cross axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    /// ```css
    /// align-self: flex-end;
    /// ```
    fn self_end(mut self) -> Self {
        self.style_mut().align_self = Some("flex-end");
        self
    }

    /// Define how to justify an invidvidual flex or grid item along its container's cross axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    /// ```css
    /// align-self: center;
    /// ```
    fn self_center(mut self) -> Self {
        self.style_mut().align_self = Some("center");
        self
    }

    /// Define how to justify an invidvidual flex or grid item along its container's cross axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    /// ```css
    /// align-self: stretch;
    /// ```
    fn self_stretch(mut self) -> Self {
        self.style_mut().align_self = Some("stretch");
        self
    }

    /// Define how to justify an invidvidual flex or grid item along its container's cross axis.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    /// ```css
    /// align-self: baseline;
    /// ```
    fn self_baseline(mut self) -> Self {
        self.style_mut().align_self = Some("baseline");
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

    /// Set the logical inline start position of a positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/inset-inline-start>
    /// ```css
    /// inset-inline-start: {x};
    /// ```
    #[with_auto]
    #[with_full]
    fn start(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().inset_inline_start = Some(x.into());
        self
    }

    /// Set the text color.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/color>
    /// ```css
    /// color: {color};
    /// ```
    fn text_color(mut self, color: &'static str) -> Self {
        self.style_mut().color = Some(color);
        self
    }

    /// Set the font size.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/font-size>
    /// ```css
    /// font-size: {x};
    /// ```
    #[without_zero]
    fn text_size(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().font_size = Some(x.into());
        self
    }

    /// Set the text layout.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    /// ```css
    /// text-align: left;
    /// ```
    fn text_left(mut self) -> Self {
        self.style_mut().text_align = Some("left");
        self
    }

    /// Set the text layout.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    /// ```css
    /// text-align: center;
    /// ```
    fn text_center(mut self) -> Self {
        self.style_mut().text_align = Some("center");
        self
    }

    /// Set the text layout.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    /// ```css
    /// text-align: right;
    /// ```
    fn text_right(mut self) -> Self {
        self.style_mut().text_align = Some("right");
        self
    }

    /// Set the text layout.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    /// ```css
    /// text-align: justify;
    /// ```
    fn text_justify(mut self) -> Self {
        self.style_mut().text_align = Some("justify");
        self
    }

    /// Set the text layout.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    /// ```css
    /// text-align: start;
    /// ```
    fn text_start(mut self) -> Self {
        self.style_mut().text_align = Some("start");
        self
    }

    /// Set the text layout.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>
    /// ```css
    /// text-align: end;
    /// ```
    fn text_end(mut self) -> Self {
        self.style_mut().text_align = Some("end");
        self
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform>
    /// ```css
    /// text-transform: uppercase;
    /// ```
    fn uppercase(mut self) -> Self {
        self.style_mut().text_transform = Some("uppercase");
        self
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform>
    /// ```css
    /// text-transform: lowercase;
    /// ```
    fn lowercase(mut self) -> Self {
        self.style_mut().text_transform = Some("lowercase");
        self
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform>
    /// ```css
    /// text-transform: capitalize;
    /// ```
    fn capitalize(mut self) -> Self {
        self.style_mut().text_transform = Some("capitalize");
        self
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-transform>
    /// ```css
    /// text-transform: none;
    /// ```
    fn normal_case(mut self) -> Self {
        self.style_mut().text_transform = Some("none");
        self
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-overflow>
    /// ```css
    /// text-overflow: ellipsis;
    /// ```
    fn text_ellipsis(mut self) -> Self {
        self.style_mut().text_overflow = Some("ellipsis");
        self
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/text-overflow>
    /// ```css
    /// text-overflow: clip;
    /// ```
    fn text_clip(mut self) -> Self {
        self.style_mut().text_overflow = Some("clip");
        self
    }

    /// ```css
    /// overflow: hidden;
    /// text-overflow: ellipsis;
    /// white-space: nowrap;
    /// ```
    fn truncate(self) -> Self {
        self.overflow_hidden().text_ellipsis().whitespace_nowrap()
    }

    /// Set gradient to color.
    /// ```css
    /// background-image: linear-gradient(..., ..., {color});
    /// ```
    fn to(mut self, color: &'static str) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .to_color = Some(color);
        self
    }

    /// Set gradient to position in percent.
    /// ```css
    /// background-image: linear-gradient(..., ..., to_color {x});
    /// ```
    fn to_position(mut self, x: i16) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .to_position = Some(Length::Percent(f32::from(x)));
        self
    }

    /// Set gradient to position in percent.
    /// ```css
    /// background-image: linear-gradient(..., ..., to_color {x});
    /// ```
    fn to_positionf(mut self, x: f32) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .to_position = Some(Length::Percent(x));
        self
    }

    /// Set the top position of the positioned element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
    /// ```css
    /// top: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_vertical]
    #[with_vertical_viewport_units]
    fn top(mut self, x: impl Into<Length>) -> Self {
        self.style_mut()
            .inset
            .get_or_insert_default()
            .set_top(x.into());
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: {x};
    /// ```
    fn tracking(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().letter_spacing = Some(x.into());
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: -0.05em;
    /// ```
    fn tracking_tighter(mut self) -> Self {
        self.style_mut().letter_spacing = Some(Length::Em(-0.05));
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: -0.025em;
    /// ```
    fn tracking_tight(mut self) -> Self {
        self.style_mut().letter_spacing = Some(Length::Em(-0.025));
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: 0.0em;
    /// ```
    fn tracking_normal(mut self) -> Self {
        self.style_mut().letter_spacing = Some(Length::Em(0.0));
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: 0.025em;
    /// ```
    fn tracking_wide(mut self) -> Self {
        self.style_mut().letter_spacing = Some(Length::Em(0.025));
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: 0.05em;
    /// ```
    fn tracking_wider(mut self) -> Self {
        self.style_mut().letter_spacing = Some(Length::Em(0.05));
        self
    }

    /// Set the tracking (letter spacing) of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/letter-spacing>
    /// ```css
    /// letter-spacing: 0.1em;
    /// ```
    fn tracking_widest(mut self) -> Self {
        self.style_mut().letter_spacing = Some(Length::Em(0.1));
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: {property};
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition_property(mut self, property: &'static str) -> Self {
        let style = self.style_mut();
        style.transition_property = Some(property);
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Set which properties to transition to none.
    /// ```css
    /// transition-property: none;
    /// ```
    fn transition_none(mut self) -> Self {
        self.style_mut().transition_property = Some("none");
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: all;
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition_all(mut self) -> Self {
        let style = self.style_mut();
        style.transition_property = Some("all");
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: color, background-color, border-color, text-decoration-color, fill,
    ///     stroke, opacity, box-shadow, transform, filter, backdrop-filter;
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition(mut self) -> Self {
        let style = self.style_mut();
        style.transition_property = Some(
            "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, \
         box-shadow, transform, filter, backdrop-filter",
        );
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: color, background-color, border-color, text-decoration-color, fill,
    ///     stroke;
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition_colors(mut self) -> Self {
        let style = self.style_mut();
        style.transition_property =
            Some("color, background-color, border-color, text-decoration-color, fill, stroke");
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: opacity;
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition_opacity(mut self) -> Self {
        let style = self.style_mut();
        style.transition_property = Some("opacity");
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: box-shadow;
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition_shadow(mut self) -> Self {
        let style = self.style_mut();
        style.transition_property = Some("box-shadow");
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Set which properties to transition (and enable default values for transition timing and duration if not already set).
    /// ```css
    /// transition-property: transform;
    /// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// transition-duration: 150ms;
    /// ```
    fn transition_transform(mut self) -> Self {
        let style = self.style_mut();
        style.transition_property = Some("transform");
        style.transition_timing_function = Some("cubic-bezier(0.4, 0, 0.2, 1)");
        style.transition_duration = Some(Duration::Ms(150));
        self
    }

    /// Duration of CSS transitions in milliseconds.
    /// ```css
    /// transition-duration: {ms}ms;
    /// ```
    fn duration_ms(mut self, ms: u32) -> Self {
        self.style_mut().transition_duration = Some(Duration::Ms(ms));
        self
    }

    /// Duration of CSS transitions in seconds.
    /// ```css
    /// transition-duration: {s}s;
    /// ```
    fn duration_s(mut self, s: f32) -> Self {
        self.style_mut().transition_duration = Some(Duration::S(s));
        self
    }

    /// Delay of CSS transitions in milliseconds.
    /// ```css
    /// transition-delay: {ms}ms;
    /// ```
    fn delay_ms(mut self, ms: u32) -> Self {
        self.style_mut().transition_duration = Some(Duration::Ms(ms));
        self
    }

    /// Delay of CSS transitions in seconds.
    /// ```css
    /// transition-delay: {s}s;
    /// ```
    fn delay_s(mut self, s: f32) -> Self {
        self.style_mut().transition_duration = Some(Duration::S(s));
        self
    }

    /// Set translation transforms.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>
    /// ```css
    /// transform: translateX({x});
    /// ```
    #[with_full]
    #[with_horizontal_viewport_units]
    #[with_screen_horizontal]
    fn translate_x(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().transform = Some(Transform::TranslateX(x.into()));
        self
    }

    /// Set translation transforms.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>
    /// ```css
    /// transform: translateY({x});
    /// ```
    #[with_full]
    #[with_vertical_viewport_units]
    #[with_screen_vertical]
    fn translate_y(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().transform = Some(Transform::TranslateY(x.into()));
        self
    }

    /// Set gradient to color.
    /// ```css
    /// background-image: linear-gradient(..., ..., {color}, ...);
    /// ```
    fn via(mut self, color: &'static str) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .via_color = Some(color);
        self
    }

    /// Set gradient via position in percent.
    /// ```css
    /// background-image: linear-gradient(..., ..., via_color {x}, ...);
    /// ```
    fn via_position(mut self, x: i16) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .via_position = Some(Length::Percent(f32::from(x)));
        self
    }

    /// Set gradient via position in percent.
    /// ```css
    /// background-image: linear-gradient(..., ..., via_color {x}, ...);
    /// ```
    fn via_positionf(mut self, x: f32) -> Self {
        self.style_mut()
            .background_image
            .get_or_insert_default()
            .linear_gradient()
            .via_position = Some(Length::Percent(x));
        self
    }

    /// Set the visibility of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/visibility>
    /// ```css
    /// visibility: visible;
    /// ```
    fn visible(mut self) -> Self {
        self.style_mut().visibility = Some("visible");
        self
    }

    /// Set the visibility of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/visibility>
    /// ```css
    /// visibility: hidden;
    /// ```
    fn invisible(mut self) -> Self {
        self.style_mut().visibility = Some("hidden");
        self
    }

    /// Set the visibility of an element.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/visibility>
    /// ```css
    /// visibility: collapse;
    /// ```
    fn collapse(mut self) -> Self {
        self.style_mut().visibility = Some("collapse");
        self
    }

    /// Set the element's width.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/width>
    /// ```css
    /// width: {x};
    /// ```
    #[with_auto]
    #[with_full]
    #[with_screen_horizontal]
    #[with_horizontal_viewport_units]
    #[with_content]
    fn w(mut self, x: impl Into<Length>) -> Self {
        self.style_mut().width = Some(x.into());
        self
    }

    /// Set how white space inside an element is handled.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    /// ```css
    /// white-space: normal;
    /// ```
    fn whitespace_normal(mut self) -> Self {
        self.style_mut().white_space = Some("normal");
        self
    }

    /// Set how white space inside an element is handled.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    /// ```css
    /// white-space: nowrap;
    /// ```
    fn whitespace_nowrap(mut self) -> Self {
        self.style_mut().white_space = Some("nowrap");
        self
    }

    /// Set how white space inside an element is handled.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    /// ```css
    /// white-space: pre;
    /// ```
    fn whitespace_pre(mut self) -> Self {
        self.style_mut().white_space = Some("pre");
        self
    }

    /// Set how white space inside an element is handled.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    /// ```css
    /// white-space: pre-line;
    /// ```
    fn whitespace_pre_line(mut self) -> Self {
        self.style_mut().white_space = Some("pre-line");
        self
    }

    /// Set how white space inside an element is handled.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    /// ```css
    /// white-space: pre-wrap;
    /// ```
    fn whitespace_pre_wrap(mut self) -> Self {
        self.style_mut().white_space = Some("pre-wrap");
        self
    }

    /// Set how white space inside an element is handled.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>
    /// ```css
    /// white-space: break-spaces;
    /// ```
    fn whitespace_break_spaces(mut self) -> Self {
        self.style_mut().white_space = Some("break-spaces");
        self
    }

    /// Set the z-order of a positioned element and its descendants or flex items.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/z-index>
    /// ```css
    /// z-index: {x};
    /// ```
    fn z(mut self, x: i32) -> Self {
        self.style_mut().z_index = Some(Either::Left(x));
        self
    }

    /// Set the z-order of a positioned element and its descendants or flex items.
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/z-index>
    /// ```css
    /// z-index: auto;
    /// ```
    fn z_auto(mut self) -> Self {
        self.style_mut().z_index = Some(Either::Right("auto"));
        self
    }
    /// ```css
    /// clip-path: {clip_path};
    /// ```
    fn clip_path(mut self, clip_path: &'static str) -> Self {
        self.style_mut().clip_path = Some(clip_path);
        self
    }

    /// Hide an element visually without hiding it from screen readers.
    /// ```css
    /// position: absolute;
    /// width: 1px;
    /// height: 1px;
    /// padding: 0;
    /// margin: -1px;
    /// overflow: hidden;
    /// clip-path: inset(50%);
    /// white-space: nowrap;
    /// border-width: 0;
    /// ```
    fn sr_only(self) -> Self {
        self.absolute()
            .w_px(1)
            .h_px(1)
            .p(0)
            .m_px(-1)
            .overflow_hidden()
            .clip_path("inset(50%)")
            .whitespace_nowrap()
            .border_px(0)
    }

    /// Undo [sr_only()].
    /// ```css
    /// position: static;
    /// width: auto;
    /// height: auto;
    /// padding: 0;
    /// margin: 0;
    /// overflow: visible;
    /// clip: auto;
    /// white-space: normal;
    /// ```
    fn not_sr_only(self) -> Self {
        self.static_()
            .w_auto()
            .h_auto()
            .p(0)
            .m_px(0)
            .overflow_visible()
            .clip_path("auto")
            .whitespace_normal()
    }

    /// ```css
    /// opacity: {x}%;
    /// ```
    fn opacity(mut self, x: i16) -> Self {
        self.style_mut().opacity = Some(Length::Percent(f32::from(x)));
        self
    }

    /// ```css
    /// opacity: {x}%;
    /// ```
    fn opacityf(mut self, x: f32) -> Self {
        self.style_mut().opacity = Some(Length::Percent(x));
        self
    }
}

impl<T> StyleExt for T where T: Style {}
