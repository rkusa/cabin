use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // TODO: customize based on env variable or custom config?
    let theme = Theme::default();

    // breakpoints
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("responsive.rs");
    let out = &mut File::create(path).unwrap();
    writeln!(out, r#"pub trait Responsive {{"#).unwrap();
    for (ident, min_width_px) in theme.breakpoints {
        writeln!(out, r#"/// `@media (min-width: {min_width_px}px)`"#).unwrap();
        writeln!(
            out,
            r#"fn {ident}(self) -> pseudo::min_width::MinWidth<Self> where Self: Sized;"#
        )
        .unwrap();
    }
    for window in theme.breakpoints.windows(2) {
        let &[(ident, _), (_, min_width_next_px)] = window else {
            unreachable!()
        };
        let max_width_px = min_width_next_px.saturating_sub(1);
        writeln!(out, r#"/// `@media (max-width: {max_width_px}px)`"#).unwrap();
        writeln!(
            out,
            r#"fn max_{ident}(self) -> pseudo::max_width::MaxWidth<Self> where Self: Sized;"#
        )
        .unwrap();
    }
    writeln!(out, r#"}}"#).unwrap();
    writeln!(out, r#"impl<S: Style> Responsive for S {{"#).unwrap();
    for (ident, min_width_px) in theme.breakpoints {
        writeln!(
            out,
            r#"
                fn {ident}(self) -> pseudo::min_width::MinWidth<Self> {{
                    pseudo::min_width::MinWidth::new({min_width_px}, self)
                }}
            "#
        )
        .unwrap();
    }
    for window in theme.breakpoints.windows(2) {
        let &[(ident, _), (_, min_width_next_px)] = window else {
            unreachable!()
        };
        let max_width_px = min_width_next_px.saturating_sub(1);
        writeln!(
            out,
            r#"
                fn max_{ident}(self) -> pseudo::max_width::MaxWidth<Self> {{
                    pseudo::max_width::MaxWidth::new({max_width_px}, self)
                }}
            "#
        )
        .unwrap();
    }
    writeln!(out, r#"}}"#).unwrap();

    // color
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("text-color.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, color) in theme.colors {
        writeln!(
            out,
            r#"/// `color: {color};` <b style="color:{color}">⏺</b>"#
        )
        .unwrap();
        writeln!(
            out,
            r#"pub const {ident}: Property = Property(COLOR, "{color}");"#
        )
        .unwrap();
    }

    // background-color
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("background-color.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, color) in theme.colors {
        writeln!(
            out,
            r#"/// `background-color: {color};` <b style="color:{color}">⏺</b>"#
        )
        .unwrap();
        writeln!(
            out,
            r#"pub const {ident}: Property = Property(BACKGROUND_COLOR, "{color}");"#
        )
        .unwrap();
    }

    // border-color
    {
        enum Properties {
            One(&'static str, &'static str),
            Two((&'static str, &'static str), (&'static str, &'static str)),
        }
        let variants = [
            (
                "border-color.rs",
                Properties::One("BORDER_COLOR", "border-color"),
            ),
            (
                "border-x-color.rs",
                Properties::Two(
                    ("BORDER_LEFT_COLOR", "border-left-color"),
                    ("BORDER_RIGHT_COLOR", "border-right-color"),
                ),
            ),
            (
                "border-y-color.rs",
                Properties::Two(
                    ("BORDER_TOP_COLOR", "border-top-color"),
                    ("BORDER_BOTTOM_COLOR", "border-bottom-color"),
                ),
            ),
            (
                "border-s-color.rs",
                Properties::One("BORDER_INLINE_START_COLOR", "border-inline-start-color"),
            ),
            (
                "border-e-color.rs",
                Properties::One("BORDER_INLINE_END_COLOR", "border-inline-end-color"),
            ),
            (
                "border-t-color.rs",
                Properties::One("BORDER_TOP_COLOR", "border-top-color"),
            ),
            (
                "border-b-color.rs",
                Properties::One("BORDER_BOTTOM_COLOR", "border-bottom-color"),
            ),
            (
                "border-l-color.rs",
                Properties::One("BORDER_LEFT_COLOR", "border-left-color"),
            ),
            (
                "border-r-color.rs",
                Properties::One("BORDER_RIGHT_COLOR", "border-right-color"),
            ),
        ];

        for (file_name, properties) in variants {
            let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(file_name);
            let out = &mut File::create(path).unwrap();

            for (ident, color) in theme.colors {
                match properties {
                    Properties::One(rust_name, css_name) => {
                        writeln!(out, r#"/// `{css_name}: {color};`"#).unwrap();
                        writeln!(
                            out,
                            "pub const {ident}: Property = \
                            Property({rust_name}, {color:?});"
                        )
                        .unwrap();
                    }
                    Properties::Two((rust_name1, css_name1), (rust_name2, css_name2)) => {
                        writeln!(out, r#"/// <b style="color:{color}">⏺</b>"#).unwrap();
                        writeln!(out, r#"/// ```css"#).unwrap();
                        writeln!(out, r#"/// {css_name1}: {color};"#).unwrap();
                        writeln!(out, r#"/// {css_name2}: {color};"#).unwrap();
                        writeln!(out, r#"/// ```"#).unwrap();
                        writeln!(
                            out,
                            "pub const {ident}: PropertyTwice = \
                                PropertyTwice({rust_name1}, {rust_name2}, {color:?});"
                        )
                        .unwrap();
                    }
                }
            }
        }
    }

    // outline-color
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("outline-color.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, color) in theme.colors {
        writeln!(
            out,
            r#"/// `outline-color: {color};` <b style="color:{color}">⏺</b>"#
        )
        .unwrap();
        writeln!(
            out,
            r#"pub const {ident}: Property = Property(OUTLINE_COLOR, "{color}");"#
        )
        .unwrap();
    }

    // ring-color
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("ring-color.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, color) in theme.colors {
        writeln!(
            out,
            r#"/// `--tw-ring-color: {color};` <b style="color:{color}">⏺</b>"#
        )
        .unwrap();
        writeln!(
            out,
            r#"pub const {ident}: RingColor = RingColor("{color}");"#
        )
        .unwrap();
    }

    // ring-offset-color
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("ring-offset-color.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, color) in theme.colors {
        writeln!(
            out,
            r#"/// `--tw-ring-offset-color: {color};` <b style="color:{color}">⏺</b>"#
        )
        .unwrap();
        writeln!(
                out,
                r#"/// `box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);`"#
            )
            .unwrap();
        writeln!(
            out,
            r#"pub const {ident}: RingOffsetColor = RingOffsetColor("{color}");"#
        )
        .unwrap();
    }

    // font-size
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("font-size.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, font_size, line_height) in theme.font_sizes {
        writeln!(
            out,
            r#"/// `font-size: {font_size}; line-height: {line_height};`"#
        )
        .unwrap();
        writeln!(
            out,
            r#"pub const {ident}: FontSize = FontSize {{ font_size: {font_size:?}, line_height: {line_height:?} }};"#
        )
        .unwrap();
    }

    // font-family
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("font-family.rs");
    let out = &mut File::create(path).unwrap();
    for (ident, font_family) in theme.font_families {
        writeln!(out, r#"/// `font-family: {font_family};`"#).unwrap();
        writeln!(
            out,
            r##"pub const {ident}: Property = Property(FONT_FAMILY, r#"{font_family}"#);"##
        )
        .unwrap();
    }

    // rounded
    {
        enum Properties {
            One(&'static str, &'static str),
            Two((&'static str, &'static str), (&'static str, &'static str)),
        }
        let variants = [
            (
                "rounded.rs",
                Properties::One("BORDER_RADIUS", "border-radius"),
            ),
            (
                "rounded-b.rs",
                Properties::Two(
                    ("BORDER_BOTTOM_RIGHT_RADIUS", "border-bottom-right-radius"),
                    ("BORDER_BOTTOM_LEFT_RADIUS", "border-bottom-left-radius"),
                ),
            ),
            (
                "rounded-bl.rs",
                Properties::One("BORDER_BOTTOM_LEFT_RADIUS", "border-bottom-left-radius"),
            ),
            (
                "rounded-br.rs",
                Properties::One("BORDER_BOTTOM_RIGHT_RADIUS", "border-bottom-right-radius"),
            ),
            (
                "rounded-l.rs",
                Properties::Two(
                    ("BORDER_TOP_LEFT_RADIUS", "border-top-left-radius"),
                    ("BORDER_BOTTOM_LEFT_RADIUS", "border-bottom-left-radius"),
                ),
            ),
            (
                "rounded-r.rs",
                Properties::Two(
                    ("BORDER_TOP_RIGHT_RADIUS", "border-top-right-radius"),
                    ("BORDER_BOTTOM_RIGHT_RADIUS", "border-bottom-right-radius"),
                ),
            ),
            (
                "rounded-t.rs",
                Properties::Two(
                    ("BORDER_TOP_RIGHT_RADIUS", "border-top-right-radius"),
                    ("BORDER_TOP_LEFT_RADIUS", "border-top-left-radius"),
                ),
            ),
            (
                "rounded-tl.rs",
                Properties::One("BORDER_TOP_LEFT_RADIUS", "border-bottom-left-radius"),
            ),
            (
                "rounded-tr.rs",
                Properties::One("BORDER_TOP_RIGHT_RADIUS", "border-bottom-right-radius"),
            ),
        ];

        for (file_name, properties) in variants {
            let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(file_name);
            let out = &mut File::create(path).unwrap();

            for (ident, size) in theme.rounded {
                match properties {
                    Properties::One(rust_name, css_name) => {
                        writeln!(out, r#"/// `{css_name}: {size};`"#).unwrap();
                        writeln!(
                            out,
                            "pub const {ident}: Property<Length> = \
                            Property({rust_name}, {size:?});"
                        )
                        .unwrap();
                    }
                    Properties::Two((rust_name1, css_name1), (rust_name2, css_name2)) => {
                        writeln!(out, r#"/// `{css_name1}: {size}; {css_name2}: {size};`"#)
                            .unwrap();
                        writeln!(
                            out,
                            "pub const {ident}: PropertyTwice<Length> = \
                                PropertyTwice({rust_name1}, {rust_name2}, {size:?});"
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}

struct Theme {
    breakpoints: &'static [(&'static str, u32)],
    colors: &'static [(&'static str, &'static str)],
    font_families: &'static [(&'static str, &'static str)],
    font_sizes: &'static [(&'static str, Length, LineHeight)],
    rounded: &'static [(&'static str, Length)],
}

enum LineHeight {
    Length(Length),
    Multiple(u16),
}

pub enum Length {
    Auto,
    Px(f32),
    Rem(f32),
    Percent(f32),
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            breakpoints: &[
                ("sm", 640),
                ("md", 768),
                ("lg", 1024),
                ("xl", 1280),
                ("xl2", 1536),
            ],
            colors: &[
                ("INHERIT", "inherit"),
                ("CURRENT", "currentColor"),
                ("TRANSPARENT", "transparent"),
                ("BLACK", "#000"),
                ("WHITE", "#fff"),
                ("SLATE_50", "#f8fafc"),
                ("SLATE_100", "#f1f5f9"),
                ("SLATE_200", "#e2e8f0"),
                ("SLATE_300", "#cbd5e1"),
                ("SLATE_400", "#94a3b8"),
                ("SLATE_500", "#64748b"),
                ("SLATE_600", "#475569"),
                ("SLATE_700", "#334155"),
                ("SLATE_800", "#1e293b"),
                ("SLATE_900", "#0f172a"),
                ("GRAY_50", "#f9fafb"),
                ("GRAY_100", "#f3f4f6"),
                ("GRAY_200", "#e5e7eb"),
                ("GRAY_300", "#d1d5db"),
                ("GRAY_400", "#9ca3af"),
                ("GRAY_500", "#6b7280"),
                ("GRAY_600", "#4b5563"),
                ("GRAY_700", "#374151"),
                ("GRAY_800", "#1f2937"),
                ("GRAY_900", "#111827"),
                ("ZINC_50", "#fafafa"),
                ("ZINC_100", "#f4f4f5"),
                ("ZINC_200", "#e4e4e7"),
                ("ZINC_300", "#d4d4d8"),
                ("ZINC_400", "#a1a1aa"),
                ("ZINC_500", "#71717a"),
                ("ZINC_600", "#52525b"),
                ("ZINC_700", "#3f3f46"),
                ("ZINC_800", "#27272a"),
                ("ZINC_900", "#18181b"),
                ("NEUTRAL_50", "#fafafa"),
                ("NEUTRAL_100", "#f5f5f5"),
                ("NEUTRAL_200", "#e5e5e5"),
                ("NEUTRAL_300", "#d4d4d4"),
                ("NEUTRAL_400", "#a3a3a3"),
                ("NEUTRAL_500", "#737373"),
                ("NEUTRAL_600", "#525252"),
                ("NEUTRAL_700", "#404040"),
                ("NEUTRAL_800", "#262626"),
                ("NEUTRAL_900", "#171717"),
                ("STONE_50", "#fafaf9"),
                ("STONE_100", "#f5f5f4"),
                ("STONE_200", "#e7e5e4"),
                ("STONE_300", "#d6d3d1"),
                ("STONE_400", "#a8a29e"),
                ("STONE_500", "#78716c"),
                ("STONE_600", "#57534e"),
                ("STONE_700", "#44403c"),
                ("STONE_800", "#292524"),
                ("STONE_900", "#1c1917"),
                ("RED_50", "#fef2f2"),
                ("RED_100", "#fee2e2"),
                ("RED_200", "#fecaca"),
                ("RED_300", "#fca5a5"),
                ("RED_400", "#f87171"),
                ("RED_500", "#ef4444"),
                ("RED_600", "#dc2626"),
                ("RED_700", "#b91c1c"),
                ("RED_800", "#991b1b"),
                ("RED_900", "#7f1d1d"),
                ("ORANGE_50", "#fff7ed"),
                ("ORANGE_100", "#ffedd5"),
                ("ORANGE_200", "#fed7aa"),
                ("ORANGE_300", "#fdba74"),
                ("ORANGE_400", "#fb923c"),
                ("ORANGE_500", "#f97316"),
                ("ORANGE_600", "#ea580c"),
                ("ORANGE_700", "#c2410c"),
                ("ORANGE_800", "#9a3412"),
                ("ORANGE_900", "#7c2d12"),
                ("AMBER_50", "#fffbeb"),
                ("AMBER_100", "#fef3c7"),
                ("AMBER_200", "#fde68a"),
                ("AMBER_300", "#fcd34d"),
                ("AMBER_400", "#fbbf24"),
                ("AMBER_500", "#f59e0b"),
                ("AMBER_600", "#d97706"),
                ("AMBER_700", "#b45309"),
                ("AMBER_800", "#92400e"),
                ("AMBER_900", "#78350f"),
                ("YELLOW_50", "#fefce8"),
                ("YELLOW_100", "#fef9c3"),
                ("YELLOW_200", "#fef08a"),
                ("YELLOW_300", "#fde047"),
                ("YELLOW_400", "#facc15"),
                ("YELLOW_500", "#eab308"),
                ("YELLOW_600", "#ca8a04"),
                ("YELLOW_700", "#a16207"),
                ("YELLOW_800", "#854d0e"),
                ("YELLOW_900", "#713f12"),
                ("LIME_50", "#f7fee7"),
                ("LIME_100", "#ecfccb"),
                ("LIME_200", "#d9f99d"),
                ("LIME_300", "#bef264"),
                ("LIME_400", "#a3e635"),
                ("LIME_500", "#84cc16"),
                ("LIME_600", "#65a30d"),
                ("LIME_700", "#4d7c0f"),
                ("LIME_800", "#3f6212"),
                ("LIME_900", "#365314"),
                ("GREEN_50", "#f0fdf4"),
                ("GREEN_100", "#dcfce7"),
                ("GREEN_200", "#bbf7d0"),
                ("GREEN_300", "#86efac"),
                ("GREEN_400", "#4ade80"),
                ("GREEN_500", "#22c55e"),
                ("GREEN_600", "#16a34a"),
                ("GREEN_700", "#15803d"),
                ("GREEN_800", "#166534"),
                ("GREEN_900", "#14532d"),
                ("EMERALD_50", "#ecfdf5"),
                ("EMERALD_100", "#d1fae5"),
                ("EMERALD_200", "#a7f3d0"),
                ("EMERALD_300", "#6ee7b7"),
                ("EMERALD_400", "#34d399"),
                ("EMERALD_500", "#10b981"),
                ("EMERALD_600", "#059669"),
                ("EMERALD_700", "#047857"),
                ("EMERALD_800", "#065f46"),
                ("EMERALD_900", "#064e3b"),
                ("TEAL_50", "#f0fdfa"),
                ("TEAL_100", "#ccfbf1"),
                ("TEAL_200", "#99f6e4"),
                ("TEAL_300", "#5eead4"),
                ("TEAL_400", "#2dd4bf"),
                ("TEAL_500", "#14b8a6"),
                ("TEAL_600", "#0d9488"),
                ("TEAL_700", "#0f766e"),
                ("TEAL_800", "#115e59"),
                ("TEAL_900", "#134e4a"),
                ("CYAN_50", "#ecfeff"),
                ("CYAN_100", "#cffafe"),
                ("CYAN_200", "#a5f3fc"),
                ("CYAN_300", "#67e8f9"),
                ("CYAN_400", "#22d3ee"),
                ("CYAN_500", "#06b6d4"),
                ("CYAN_600", "#0891b2"),
                ("CYAN_700", "#0e7490"),
                ("CYAN_800", "#155e75"),
                ("CYAN_900", "#164e63"),
                ("SKY_50", "#f0f9ff"),
                ("SKY_100", "#e0f2fe"),
                ("SKY_200", "#bae6fd"),
                ("SKY_300", "#7dd3fc"),
                ("SKY_400", "#38bdf8"),
                ("SKY_500", "#0ea5e9"),
                ("SKY_600", "#0284c7"),
                ("SKY_700", "#0369a1"),
                ("SKY_800", "#075985"),
                ("SKY_900", "#0c4a6e"),
                ("BLUE_50", "#eff6ff"),
                ("BLUE_100", "#dbeafe"),
                ("BLUE_200", "#bfdbfe"),
                ("BLUE_300", "#93c5fd"),
                ("BLUE_400", "#60a5fa"),
                ("BLUE_500", "#3b82f6"),
                ("BLUE_600", "#2563eb"),
                ("BLUE_700", "#1d4ed8"),
                ("BLUE_800", "#1e40af"),
                ("BLUE_900", "#1e3a8a"),
                ("INDIGO_50", "#eef2ff"),
                ("INDIGO_100", "#e0e7ff"),
                ("INDIGO_200", "#c7d2fe"),
                ("INDIGO_300", "#a5b4fc"),
                ("INDIGO_400", "#818cf8"),
                ("INDIGO_500", "#6366f1"),
                ("INDIGO_600", "#4f46e5"),
                ("INDIGO_700", "#4338ca"),
                ("INDIGO_800", "#3730a3"),
                ("INDIGO_900", "#312e81"),
                ("VIOLET_50", "#f5f3ff"),
                ("VIOLET_100", "#ede9fe"),
                ("VIOLET_200", "#ddd6fe"),
                ("VIOLET_300", "#c4b5fd"),
                ("VIOLET_400", "#a78bfa"),
                ("VIOLET_500", "#8b5cf6"),
                ("VIOLET_600", "#7c3aed"),
                ("VIOLET_700", "#6d28d9"),
                ("VIOLET_800", "#5b21b6"),
                ("VIOLET_900", "#4c1d95"),
                ("PURPLE_50", "#faf5ff"),
                ("PURPLE_100", "#f3e8ff"),
                ("PURPLE_200", "#e9d5ff"),
                ("PURPLE_300", "#d8b4fe"),
                ("PURPLE_400", "#c084fc"),
                ("PURPLE_500", "#a855f7"),
                ("PURPLE_600", "#9333ea"),
                ("PURPLE_700", "#7e22ce"),
                ("PURPLE_800", "#6b21a8"),
                ("PURPLE_900", "#581c87"),
                ("FUCHSIA_50", "#fdf4ff"),
                ("FUCHSIA_100", "#fae8ff"),
                ("FUCHSIA_200", "#f5d0fe"),
                ("FUCHSIA_300", "#f0abfc"),
                ("FUCHSIA_400", "#e879f9"),
                ("FUCHSIA_500", "#d946ef"),
                ("FUCHSIA_600", "#c026d3"),
                ("FUCHSIA_700", "#a21caf"),
                ("FUCHSIA_800", "#86198f"),
                ("FUCHSIA_900", "#701a75"),
                ("PINK_50", "#fdf2f8"),
                ("PINK_100", "#fce7f3"),
                ("PINK_200", "#fbcfe8"),
                ("PINK_300", "#f9a8d4"),
                ("PINK_400", "#f472b6"),
                ("PINK_500", "#ec4899"),
            ],
            font_families: &[
                (
                    "SANS",
                    r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji""#,
                ),
                (
                    "SERIF",
                    r#"ui-serif, Georgia, Cambria, "Times New Roman", Times, serif"#,
                ),
                (
                    "MONO",
                    r#"ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#,
                ),
            ],
            font_sizes: &[
                (
                    "XS",
                    Length::Rem(0.75),
                    LineHeight::Length(Length::Rem(1.0)),
                ),
                (
                    "SM",
                    Length::Rem(0.875),
                    LineHeight::Length(Length::Rem(1.25)),
                ),
                (
                    "BASE",
                    Length::Rem(1.0),
                    LineHeight::Length(Length::Rem(1.5)),
                ),
                (
                    "LG",
                    Length::Rem(1.125),
                    LineHeight::Length(Length::Rem(1.75)),
                ),
                (
                    "XL",
                    Length::Rem(1.25),
                    LineHeight::Length(Length::Rem(1.75)),
                ),
                (
                    "XL2",
                    Length::Rem(1.5),
                    LineHeight::Length(Length::Rem(2.0)),
                ),
                (
                    "XL3",
                    Length::Rem(1.875),
                    LineHeight::Length(Length::Rem(2.25)),
                ),
                (
                    "XL4",
                    Length::Rem(2.25),
                    LineHeight::Length(Length::Rem(2.5)),
                ),
                ("XL5", Length::Rem(3.0), LineHeight::Multiple(1)),
                ("XL6", Length::Rem(3.75), LineHeight::Multiple(1)),
                ("XL7", Length::Rem(4.5), LineHeight::Multiple(1)),
                ("XL8", Length::Rem(6.0), LineHeight::Multiple(1)),
                ("XL9", Length::Rem(8.0), LineHeight::Multiple(1)),
            ],
            rounded: &[
                ("SM", Length::Rem(0.125)),
                ("MD", Length::Rem(0.375)),
                ("LG", Length::Rem(0.5)),
                ("XL", Length::Rem(0.75)),
                ("XL2", Length::Rem(1.0)),
                ("XL3", Length::Rem(1.5)),
            ],
        }
    }
}

impl Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Auto => false,
            Length::Px(v) | Length::Rem(v) | Length::Percent(v) => v.abs() < f32::EPSILON,
        }
    }
}

impl fmt::Debug for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Auto => f.debug_tuple("Length::Auto").finish(),
            Length::Px(x) => f.debug_tuple("Length::Px").field(x).finish(),
            Length::Rem(x) => f.debug_tuple("Length::Rem").field(x).finish(),
            Length::Percent(x) => f.debug_tuple("Length::Percent").field(x).finish(),
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }

        match self {
            Length::Auto => f.write_str("auto"),
            Length::Px(v) => write!(f, "{v}px"),
            Length::Rem(v) => write!(f, "{v}rem"),
            Length::Percent(v) => write!(f, "{v}%"),
        }
    }
}

impl fmt::Debug for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineHeight::Length(x) => f.debug_tuple("LineHeight::Length").field(x).finish(),
            LineHeight::Multiple(x) => f.debug_tuple("LineHeight::Multiple").field(x).finish(),
        }
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineHeight::Length(l) => l.fmt(f),
            LineHeight::Multiple(x) => write!(f, "{x}"),
        }
    }
}
