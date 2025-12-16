use std::fmt::{self, Write as _};
use std::io::Write as _;

use smallvec::SmallVec;
use twox_hash::XxHash32;

use crate::render::WriteInto;
use crate::style::modifier::StyleModifier;
use crate::style::property_display::PropertyDisplay as _;
use crate::style::units::aspect::Aspect;
use crate::style::units::box_shadow::BoxShadow;
use crate::style::units::corners::Corners;
use crate::style::units::duration::Duration;
use crate::style::units::either::Either;
use crate::style::units::four_sided::FourSided;
use crate::style::units::grid_lines::GridLines;
use crate::style::units::image::Image;
use crate::style::units::inlined::Inlined;
use crate::style::units::iterations::Iterations;
use crate::style::units::length::Length;
use crate::style::units::line_clamp::LineClamp;
use crate::style::units::row_column::RowColumn;
use crate::style::units::track_repeat_equally::TrackRepeatEqually;
use crate::style::units::transform::Transform;
use crate::style::units::xy::Xy;

// FIXME: move rarely used properties into sub struct indirected via Box?
#[derive(Default)]
pub struct StyleDefinition {
    pub modifier: StyleModifier,
    //
    pub align_items: Option<&'static str>,
    pub align_self: Option<&'static str>,
    pub animation_delay: Option<Duration>,
    pub animation_duration: Option<Duration>,
    pub animation_iterations: Option<Iterations>,
    pub appearance: Option<bool>,
    pub aspect: Option<Aspect>,
    pub auto_cols: Option<Either<Length>>,
    pub auto_rows: Option<Either<Length>>,
    pub background_clip: Option<&'static str>,
    pub background_color: Option<&'static str>,
    pub background_image: Option<Image>,
    pub basis: Option<Length>,
    pub border_color: Option<FourSided<&'static str>>,
    pub border_inline_color: Option<Inlined<&'static str>>,
    // FIXME: how to handle inline style vs border style?
    pub border_inline_style: Option<&'static str>,
    pub border_inline_width: Option<Inlined<Length>>,
    pub border_radius: Option<Corners<Either<Length>>>,
    pub border_style: Option<&'static str>,
    pub border_width: Option<FourSided<Length>>,
    pub box_shadow: Option<BoxShadow>,
    pub box_sizing: Option<&'static str>,
    pub break_after: Option<&'static str>,
    pub break_before: Option<&'static str>,
    pub break_inside: Option<&'static str>,
    pub clip_path: Option<&'static str>,
    pub color: Option<&'static str>,
    pub container: Option<&'static str>,
    pub content: Option<&'static str>,
    pub cursor: Option<&'static str>,
    pub decoration: Option<&'static str>,
    pub display: Option<&'static str>,
    pub divide_x_reversed: bool,
    pub divide_y_reversed: bool,
    pub flex_direction: Option<&'static str>,
    pub flex_grow: Option<bool>,
    pub flex_shrink: Option<bool>,
    pub flex_wrap: Option<&'static str>,
    pub font_family: Option<&'static str>,
    pub font_size: Option<Length>,
    pub font_style: Option<&'static str>,
    pub font_weight: Option<u16>,
    pub gap: Option<RowColumn<Length>>,
    pub grid_auto_flow: Option<&'static str>,
    pub grid_column: Option<GridLines>,
    pub grid_row: Option<GridLines>,
    pub grid_template_columns: Option<Either<TrackRepeatEqually>>,
    pub grid_template_rows: Option<Either<TrackRepeatEqually>>,
    pub height: Option<Length>,
    pub inset: Option<FourSided<Length>>,
    pub inset_inline_end: Option<Length>,
    pub inset_inline_start: Option<Length>,
    pub justify_content: Option<&'static str>,
    pub justify_self: Option<&'static str>,
    pub letter_spacing: Option<Length>,
    pub line_clamp: Option<LineClamp>,
    // FIXME: validate below assumption
    // NOTE: Should take precedence over text::LG etc.
    pub line_height: Option<Either<Length, f32>>,
    pub list_style_type: Option<&'static str>,
    pub margin: Option<FourSided<Length>>,
    pub margin_inline: Option<Inlined<Length>>,
    pub max_height: Option<Length>,
    pub max_width: Option<Length>,
    pub min_height: Option<Length>,
    pub min_width: Option<Length>,
    pub object_fit: Option<&'static str>,
    pub object_position: Option<&'static str>,
    pub opacity: Option<Length>,
    pub order: Option<i32>,
    pub outline_color: Option<&'static str>,
    pub outline_offset: Option<Length>,
    pub outline_style: Option<&'static str>,
    pub outline_width: Option<Length>,
    pub overflow: Option<Xy<&'static str>>,
    pub padding: Option<FourSided<Length>>,
    pub place_content: Option<&'static str>,
    pub place_items: Option<&'static str>,
    pub place_self: Option<&'static str>,
    pub pointer_events: Option<&'static str>,
    pub position: Option<&'static str>,
    pub space_x_reversed: bool,
    pub space_y_reversed: bool,
    pub text_align: Option<&'static str>,
    pub text_overflow: Option<&'static str>,
    pub text_transform: Option<&'static str>,
    pub transform: Option<Transform>,
    pub transition_delay: Option<Duration>,
    pub transition_duration: Option<Duration>,
    pub transition_property: Option<&'static str>,
    pub transition_timing_function: Option<&'static str>,
    pub user_select: Option<&'static str>,
    pub visibility: Option<&'static str>,
    pub white_space: Option<&'static str>,
    pub width: Option<Length>,
    pub word_break: Option<&'static str>,
    pub z_index: Option<Either<i32>>,
}

impl fmt::Display for StyleDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let StyleDefinition {
            align_items,
            align_self,
            animation_delay,
            animation_duration,
            animation_iterations,
            appearance,
            aspect,
            auto_cols,
            auto_rows,
            background_clip,
            background_color,
            background_image,
            basis,
            border_color,
            border_inline_color,
            border_inline_style,
            border_inline_width,
            border_radius,
            border_style,
            border_width,
            box_shadow,
            box_sizing,
            break_after,
            break_before,
            break_inside,
            clip_path,
            color,
            container,
            content,
            cursor,
            decoration,
            display,
            divide_x_reversed,
            divide_y_reversed,
            flex_direction,
            flex_grow,
            flex_shrink,
            flex_wrap,
            font_family,
            font_size,
            font_style,
            font_weight,
            gap,
            grid_auto_flow,
            grid_column,
            grid_row,
            grid_template_columns,
            grid_template_rows,
            height,
            inset,
            inset_inline_end,
            inset_inline_start,
            justify_content,
            justify_self,
            letter_spacing,
            line_clamp,
            line_height,
            list_style_type,
            margin,
            margin_inline,
            max_height,
            max_width,
            min_height,
            min_width,
            modifier: _,
            object_fit,
            object_position,
            opacity,
            order,
            outline_color,
            outline_offset,
            outline_style,
            outline_width,
            overflow,
            padding,
            place_content,
            place_items,
            place_self,
            pointer_events,
            position,
            space_x_reversed,
            space_y_reversed,
            text_align,
            text_overflow,
            text_transform,
            transform,
            transition_delay,
            transition_duration,
            transition_property,
            transition_timing_function,
            user_select,
            visibility,
            white_space,
            width,
            word_break,
            z_index,
        } = self;
        align_items.fmt_property("align-items", f)?;
        align_self.fmt_property("align-self", f)?;
        animation_delay.fmt_property("animation-delay", f)?;
        animation_duration.fmt_property("animation-duration", f)?;
        animation_iterations.fmt_property("animation-iteration-count", f)?;
        if let Some(appearance) = appearance {
            let appearance = if *appearance { "auto" } else { "none" };
            appearance.fmt_property("-webkit-appearance", f)?;
            appearance.fmt_property("-moz-appearance", f)?;
            appearance.fmt_property("appearance", f)?;
        }
        aspect.fmt_property("aspect-ratio", f)?;
        auto_cols.fmt_property("grid-auto-columns", f)?;
        auto_rows.fmt_property("grid-auto-rows", f)?;
        background_clip.fmt_property("background-clip", f)?;
        background_color.fmt_property("background-color", f)?;
        background_image.fmt_property("background-image", f)?;
        basis.fmt_property("flex-basis", f)?;
        border_color.fmt_property("border-color", f)?;
        border_inline_color.fmt_property("border-color", f)?;
        border_inline_style.fmt_property("border-inline-style", f)?;
        border_inline_width
            .as_ref()
            .map(|border_width| border_width.to_reversed(*divide_x_reversed, *divide_y_reversed))
            .fmt_property("border-width", f)?;
        border_radius.fmt_property("border-radius", f)?;
        border_style.fmt_property("border-style", f)?;
        border_width.fmt_property("border-width", f)?;
        box_shadow.fmt_property("box-shadow", f)?;
        box_sizing.fmt_property("box-sizing", f)?;
        break_after.fmt_property("break-after", f)?;
        break_before.fmt_property("break-before", f)?;
        break_inside.fmt_property("break-inside", f)?;
        clip_path.fmt_property("clip-path", f)?;
        color.fmt_property("color", f)?;
        container.fmt_property("container-type", f)?;
        content.fmt_property("--tw-content", f)?;
        cursor.fmt_property("cursor", f)?;
        decoration.fmt_property("text-decoration-line", f)?;
        if line_clamp.is_none() {
            display.fmt_property("display", f)?;
        }
        flex_direction.fmt_property("flex-direction", f)?;
        flex_grow.fmt_property("flex-grow", f)?;
        flex_shrink.fmt_property("flex-shrink", f)?;
        flex_wrap.fmt_property("flex-wrap", f)?;
        font_family.fmt_property("font-family", f)?;
        font_size.fmt_property("font-size", f)?;
        font_style.fmt_property("font-style", f)?;
        font_weight.fmt_property("font-weight", f)?;
        gap.fmt_property("gap", f)?;
        grid_auto_flow.fmt_property("grid-auto-flow", f)?;
        grid_column.fmt_property("grid-column", f)?;
        grid_row.fmt_property("grid-row", f)?;
        grid_template_columns.fmt_property("grid-template-columns", f)?;
        grid_template_rows.fmt_property("grid-template-rows", f)?;
        height.fmt_property("height", f)?;
        inset.fmt_property("inset", f)?;
        inset_inline_end.fmt_property("inset-inline-end", f)?;
        inset_inline_start.fmt_property("inset-inline-start", f)?;
        justify_content.fmt_property("justify-content", f)?;
        justify_self.fmt_property("justify-self", f)?;
        letter_spacing.fmt_property("letter-spacing", f)?;
        if let Some(line_clamp) = line_clamp {
            line_clamp.fmt(f)?;
        }
        line_height.fmt_property("line-height", f)?;
        list_style_type.fmt_property("list-style-type", f)?;
        margin.fmt_property("margin", f)?;
        margin_inline
            .as_ref()
            .map(|margin_inline| margin_inline.to_reversed(*space_x_reversed, *space_y_reversed))
            .fmt_property("margin", f)?;
        max_height.fmt_property("max-height", f)?;
        max_width.fmt_property("max-width", f)?;
        min_height.fmt_property("min-height", f)?;
        min_width.fmt_property("min-width", f)?;
        object_fit.fmt_property("object-fit", f)?;
        object_position.fmt_property("object-position", f)?;
        opacity.fmt_property("opacity", f)?;
        order.fmt_property("order", f)?;
        outline_color.fmt_property("outline-color", f)?;
        outline_offset.fmt_property("outline-offset", f)?;
        outline_style.fmt_property("outline-style", f)?;
        outline_width.fmt_property("outline-width", f)?;
        if line_clamp.is_none() {
            overflow.fmt_property("overflow", f)?;
        }
        padding.fmt_property("padding", f)?;
        place_content.fmt_property("place-content", f)?;
        place_items.fmt_property("place-items", f)?;
        place_self.fmt_property("place-self", f)?;
        pointer_events.fmt_property("pointer-events", f)?;
        position.fmt_property("position", f)?;
        text_align.fmt_property("text-align", f)?;
        text_overflow.fmt_property("text-overflow", f)?;
        text_transform.fmt_property("text-transform", f)?;
        transform.fmt_property("box-shadow", f)?;
        transition_delay.fmt_property("transition-delay", f)?;
        transition_duration.fmt_property("transition-duration", f)?;
        transition_property.fmt_property("transition-property", f)?;
        transition_timing_function.fmt_property("transition-timing-function", f)?;
        user_select.fmt_property("-webkit-user-select", f)?;
        user_select.fmt_property("user-select", f)?;
        visibility.fmt_property("visibility", f)?;
        white_space.fmt_property("white-space", f)?;
        width.fmt_property("visibility", f)?;
        word_break.fmt_property("word-break", f)?;
        z_index.fmt_property("z-index", f)?;
        if *word_break == Some("normal") {
            word_break.fmt_property("overflow-wrap", f)?;
        }

        Ok(())
    }
}

impl StyleDefinition {
    pub(crate) fn new(modifier: StyleModifier) -> Self {
        Self {
            modifier,
            ..Default::default()
        }
    }

    pub(crate) fn write_to<const N: usize>(&self, out: &mut SmallVec<u8, N>) {
        let pos = out.len();

        let StyleModifier {
            active,
            disabled,
            enabled,
            focus,
            focus_visible,
            focus_within,
            hover,
            visited,
            after,
            before,
            group_hover,
            all_children,
            all_but_last_children,
            max_width,
            min_width,
            max_container_width,
            min_container_width,
            print,
            dark,
        } = self.modifier;

        // @media {}
        let has_media_query = print || dark || max_width.is_some() || min_width.is_some();
        if has_media_query {
            write!(out, "@media ").unwrap();
            let mut write_and = false;
            if print {
                write!(out, "print").unwrap();
                write_and = true;
            }
            if dark {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(prefers-color-scheme: dark)").unwrap();
                write_and = true;
            }
            if let Some(min_width) = min_width {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(min-width: {min_width}px)").unwrap();
                write_and = true;
            }
            if let Some(max_width) = max_width {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(max-width: {max_width}px)").unwrap();
                // write_and = true;
            }
            write!(out, "{{ ").unwrap();
        }

        // @container {}
        let has_container_query = max_container_width.is_some() || min_container_width.is_some();
        if has_container_query {
            write!(out, "@container ").unwrap();
            let mut write_and = false;
            if let Some(min_width) = min_container_width {
                write!(out, "(min-width: {min_width}px)").unwrap();
                write_and = true;
            }
            if let Some(max_width) = max_container_width {
                if write_and {
                    write!(out, " and ").unwrap();
                }
                write!(out, "(max-width: {max_width}px)").unwrap();
                // write_and = true;
            }
            write!(out, "{{ ").unwrap();
        }

        if group_hover {
            write!(out, ".group:hover ").unwrap();
        }
        if all_children || all_but_last_children {
            write!(out, ":where(").unwrap();
        }

        let class_name_offset = out.len();
        write!(out, "          ").unwrap();

        if active {
            write!(out, ":active").unwrap();
        }
        if disabled {
            write!(out, ":disabled").unwrap();
        }
        if enabled {
            write!(out, ":enabled").unwrap();
        }
        if focus {
            write!(out, ":focus").unwrap();
        }
        if focus_visible {
            write!(out, ":focus-focus_visible").unwrap();
        }
        if focus_within {
            write!(out, ":focus-within").unwrap();
        }
        if hover {
            write!(out, ":hover").unwrap();
        }
        if visited {
            write!(out, ":visited").unwrap();
        }
        if dark {
            write!(out, ":not(.force-light *)").unwrap();
        }
        if after {
            write!(out, "::after").unwrap();
        }
        if before {
            write!(out, "::before").unwrap();
        }

        if all_children {
            write!(out, " > *)").unwrap();
        } else if all_but_last_children {
            write!(out, " > :not(:last-child))").unwrap();
        }

        writeln!(out, " {{").unwrap();
        write!(out, "{self}").unwrap();
        write!(out, "}}").unwrap();

        if has_container_query {
            write!(out, " }}").unwrap();
        }
        if has_media_query {
            write!(out, " }}").unwrap();
        }

        writeln!(out, "").unwrap();

        // write actual class name, prepend `_` as it class names must not start with a number
        let hash = XxHash32::oneshot(0, &out[pos..]);
        write!(WriteInto::new(out, class_name_offset), "._{hash:_<8x}").unwrap();
    }
}
