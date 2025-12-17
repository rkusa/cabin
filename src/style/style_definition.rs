use std::fmt;

use crate::style::ClassName;
use crate::style::animation::AnimationStyle;
use crate::style::modifier::StyleModifier;
use crate::style::property_display::PropertyDisplay as _;
use crate::style::units::aspect::Aspect;
use crate::style::units::box_shadow::BoxShadow;
use crate::style::units::corners::Corners;
use crate::style::units::duration::Duration;
use crate::style::units::either::Either;
use crate::style::units::float::Float;
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
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct StyleDefinition {
    pub modifier: StyleModifier,
    pub animation_from: Option<AnimationStyle>,
    pub animation_to: Option<AnimationStyle>,
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
    pub line_height: Option<Either<Length, Float>>,
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
            modifier: _,
            animation_from: _,
            animation_to: _,
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
        width.fmt_property("width", f)?;
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

    pub fn class_name(&self) -> ClassName {
        ClassName::new(self)
    }

    pub fn is_empty(&self) -> bool {
        let Self {
            modifier: _,
            animation_from,
            animation_to,
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
            divide_x_reversed: _,
            divide_y_reversed: _,
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
            space_x_reversed: _,
            space_y_reversed: _,
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

        animation_from.is_none()
            && animation_to.is_none()
            && align_items.is_none()
            && align_self.is_none()
            && animation_delay.is_none()
            && animation_duration.is_none()
            && animation_iterations.is_none()
            && appearance.is_none()
            && aspect.is_none()
            && auto_cols.is_none()
            && auto_rows.is_none()
            && background_clip.is_none()
            && background_color.is_none()
            && background_image.is_none()
            && basis.is_none()
            && border_color.is_none()
            && border_inline_color.is_none()
            && border_inline_style.is_none()
            && border_inline_width.is_none()
            && border_radius.is_none()
            && border_style.is_none()
            && border_width.is_none()
            && box_shadow.is_none()
            && box_sizing.is_none()
            && break_after.is_none()
            && break_before.is_none()
            && break_inside.is_none()
            && clip_path.is_none()
            && color.is_none()
            && container.is_none()
            && content.is_none()
            && cursor.is_none()
            && decoration.is_none()
            && display.is_none()
            && flex_direction.is_none()
            && flex_grow.is_none()
            && flex_shrink.is_none()
            && flex_wrap.is_none()
            && font_family.is_none()
            && font_size.is_none()
            && font_style.is_none()
            && font_weight.is_none()
            && gap.is_none()
            && grid_auto_flow.is_none()
            && grid_column.is_none()
            && grid_row.is_none()
            && grid_template_columns.is_none()
            && grid_template_rows.is_none()
            && height.is_none()
            && inset.is_none()
            && inset_inline_end.is_none()
            && inset_inline_start.is_none()
            && justify_content.is_none()
            && justify_self.is_none()
            && letter_spacing.is_none()
            && line_clamp.is_none()
            && line_height.is_none()
            && list_style_type.is_none()
            && margin.is_none()
            && margin_inline.is_none()
            && max_height.is_none()
            && max_width.is_none()
            && min_height.is_none()
            && min_width.is_none()
            && object_fit.is_none()
            && object_position.is_none()
            && opacity.is_none()
            && order.is_none()
            && outline_color.is_none()
            && outline_offset.is_none()
            && outline_style.is_none()
            && outline_width.is_none()
            && overflow.is_none()
            && padding.is_none()
            && place_content.is_none()
            && place_items.is_none()
            && place_self.is_none()
            && pointer_events.is_none()
            && position.is_none()
            && text_align.is_none()
            && text_overflow.is_none()
            && text_transform.is_none()
            && transform.is_none()
            && transition_delay.is_none()
            && transition_duration.is_none()
            && transition_property.is_none()
            && transition_timing_function.is_none()
            && user_select.is_none()
            && visibility.is_none()
            && white_space.is_none()
            && width.is_none()
            && word_break.is_none()
            && z_index.is_none()
    }

    pub fn merge_from(&mut self, other: Self) {
        if other.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = other;
            return;
        }
        self.animation_from = other.animation_from.or(self.animation_from.take());
        self.animation_to = other.animation_to.or(self.animation_to.take());
        self.align_items = other.align_items.or(self.align_items);
        self.align_self = other.align_self.or(self.align_self);
        self.animation_delay = other.animation_delay.or(self.animation_delay);
        self.animation_duration = other.animation_duration.or(self.animation_duration);
        self.animation_iterations = other.animation_iterations.or(self.animation_iterations);
        self.appearance = other.appearance.or(self.appearance);
        self.aspect = other.aspect.or(self.aspect.take());
        self.auto_cols = other.auto_cols.or(self.auto_cols);
        self.auto_rows = other.auto_rows.or(self.auto_rows);
        self.background_clip = other.background_clip.or(self.background_clip);
        self.background_color = other.background_color.or(self.background_color);
        self.background_image = other.background_image.or(self.background_image.take());
        self.basis = other.basis.or(self.basis);
        self.border_color = other.border_color.or(self.border_color.take());
        self.border_inline_color = other
            .border_inline_color
            .or(self.border_inline_color.take());
        self.border_inline_style = other.border_inline_style.or(self.border_inline_style);
        self.border_inline_width = other
            .border_inline_width
            .or(self.border_inline_width.take());
        self.border_radius = other.border_radius.or(self.border_radius.take());
        self.border_style = other.border_style.or(self.border_style);
        self.border_width = other.border_width.or(self.border_width.take());
        self.box_shadow = other.box_shadow.or(self.box_shadow.take());
        self.box_sizing = other.box_sizing.or(self.box_sizing);
        self.break_after = other.break_after.or(self.break_after);
        self.break_before = other.break_before.or(self.break_before);
        self.break_inside = other.break_inside.or(self.break_inside);
        self.clip_path = other.clip_path.or(self.clip_path);
        self.color = other.color.or(self.color);
        self.container = other.container.or(self.container);
        self.content = other.content.or(self.content);
        self.cursor = other.cursor.or(self.cursor);
        self.decoration = other.decoration.or(self.decoration);
        self.display = other.display.or(self.display);
        self.divide_x_reversed = other.divide_x_reversed || self.divide_x_reversed;
        self.divide_y_reversed = other.divide_y_reversed || self.divide_y_reversed;
        self.flex_direction = other.flex_direction.or(self.flex_direction);
        self.flex_grow = other.flex_grow.or(self.flex_grow);
        self.flex_shrink = other.flex_shrink.or(self.flex_shrink);
        self.flex_wrap = other.flex_wrap.or(self.flex_wrap);
        self.font_family = other.font_family.or(self.font_family);
        self.font_size = other.font_size.or(self.font_size);
        self.font_style = other.font_style.or(self.font_style);
        self.font_weight = other.font_weight.or(self.font_weight);
        self.gap = other.gap.or(self.gap.take());
        self.grid_auto_flow = other.grid_auto_flow.or(self.grid_auto_flow);
        self.grid_column = other.grid_column.or(self.grid_column.take());
        self.grid_row = other.grid_row.or(self.grid_row.take());
        self.grid_template_columns = other.grid_template_columns.or(self.grid_template_columns);
        self.grid_template_rows = other.grid_template_rows.or(self.grid_template_rows);
        self.height = other.height.or(self.height);
        self.inset = other.inset.or(self.inset.take());
        self.inset_inline_end = other.inset_inline_end.or(self.inset_inline_end);
        self.inset_inline_start = other.inset_inline_start.or(self.inset_inline_start);
        self.justify_content = other.justify_content.or(self.justify_content);
        self.justify_self = other.justify_self.or(self.justify_self);
        self.letter_spacing = other.letter_spacing.or(self.letter_spacing);
        self.line_clamp = other.line_clamp.or(self.line_clamp);
        self.line_height = other.line_height.or(self.line_height);
        self.list_style_type = other.list_style_type.or(self.list_style_type);
        self.margin = other.margin.or(self.margin.take());
        self.margin_inline = other.margin_inline.or(self.margin_inline.take());
        self.max_height = other.max_height.or(self.max_height);
        self.max_width = other.max_width.or(self.max_width);
        self.min_height = other.min_height.or(self.min_height);
        self.min_width = other.min_width.or(self.min_width);
        self.object_fit = other.object_fit.or(self.object_fit);
        self.object_position = other.object_position.or(self.object_position);
        self.opacity = other.opacity.or(self.opacity);
        self.order = other.order.or(self.order);
        self.outline_color = other.outline_color.or(self.outline_color);
        self.outline_offset = other.outline_offset.or(self.outline_offset);
        self.outline_style = other.outline_style.or(self.outline_style);
        self.outline_width = other.outline_width.or(self.outline_width);
        self.overflow = other.overflow.or(self.overflow.take());
        self.padding = other.padding.or(self.padding.take());
        self.place_content = other.place_content.or(self.place_content);
        self.place_items = other.place_items.or(self.place_items);
        self.place_self = other.place_self.or(self.place_self);
        self.pointer_events = other.pointer_events.or(self.pointer_events);
        self.position = other.position.or(self.position);
        self.space_x_reversed = other.space_x_reversed || self.space_x_reversed;
        self.space_y_reversed = other.space_y_reversed || self.space_y_reversed;
        self.text_align = other.text_align.or(self.text_align);
        self.text_overflow = other.text_overflow.or(self.text_overflow);
        self.text_transform = other.text_transform.or(self.text_transform);
        self.transform = other.transform.or(self.transform.take());
        self.transition_delay = other.transition_delay.or(self.transition_delay);
        self.transition_duration = other.transition_duration.or(self.transition_duration);
        self.transition_property = other.transition_property.or(self.transition_property);
        self.transition_timing_function = other
            .transition_timing_function
            .or(self.transition_timing_function);
        self.user_select = other.user_select.or(self.user_select);
        self.visibility = other.visibility.or(self.visibility);
        self.white_space = other.white_space.or(self.white_space);
        self.width = other.width.or(self.width);
        self.word_break = other.word_break.or(self.word_break);
        self.z_index = other.z_index.or(self.z_index);
    }

    pub(crate) fn write_to(&self, out: &mut dyn fmt::Write) {
        let class_name = self.class_name();

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
            other_pseudo_element,
        } = self.modifier;

        // @keyframes
        let has_animation = self.animation_from.is_some() || self.animation_to.is_some();
        if has_animation {
            write!(out, "@keyframes {class_name} {{ from {{").unwrap();
            if let Some(animation_from) = &self.animation_from {
                write!(out, "{animation_from}").unwrap();
            }
            writeln!(out, "}} to {{").unwrap();
            if let Some(animation_to) = &self.animation_to {
                write!(out, "{animation_to}").unwrap();
            }
            writeln!(out, "}} }}").unwrap();
        }

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

        write!(out, ".{class_name}").unwrap();

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
        if let Some(pseudo) = other_pseudo_element {
            write!(out, "::{pseudo}").unwrap();
        }

        if all_children {
            write!(out, " > *)").unwrap();
        } else if all_but_last_children {
            write!(out, " > :not(:last-child))").unwrap();
        }

        writeln!(out, " {{").unwrap();
        if has_animation {
            write!(out, "animation: 250ms ease-in-out 1 forwards {class_name};").unwrap();
        }
        write!(out, "{self}").unwrap();
        write!(out, "}}").unwrap();

        if has_container_query {
            write!(out, " }}").unwrap();
        }
        if has_media_query {
            write!(out, " }}").unwrap();
        }

        writeln!(out, "").unwrap();
    }
}
