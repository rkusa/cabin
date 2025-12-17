use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // TODO: customize based on env variable or custom config?
    let theme = Theme::default();

    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("theme.rs");
    let out = &mut File::create(path).unwrap();
    writeln!(out, r#"pub trait ThemeExt: Style {{"#).unwrap();

    // color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r#"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// color: {color};
            /// ```
            fn text_{ident}(self) -> Self {{
                self.text_color("{color}")
            }}
            "#
        )
        .unwrap();
    }

    // background-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r#"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// background-color: {color};
            /// ```
            fn bg_{ident}(self) -> Self {{
                self.bg("{color}")
            }}
            "#
        )
        .unwrap();
    }

    // border-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r#"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-color: {color};
            /// ```
            fn border_{ident}(self) -> Self {{
                self.border_color("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-inline-color: {color};
            /// ```
            fn border_x_{ident}(self) -> Self {{
                self.border_color_x("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-block-color: {color};
            /// ```
            fn border_y_{ident}(self) -> Self {{
                self.border_color_y("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-top-color: {color};
            /// ```
            fn border_t_{ident}(self) -> Self {{
                self.border_color_t("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-right-color: {color};
            /// ```
            fn border_r_{ident}(self) -> Self {{
                self.border_color_r("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-bottom-color: {color};
            /// ```
            fn border_b_{ident}(self) -> Self {{
                self.border_color_b("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-left-color: {color};
            /// ```
            fn border_l_{ident}(self) -> Self {{
                self.border_color_l("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-inline-start-color: {color};
            /// ```
            fn border_s_{ident}(self) -> Self {{
                self.border_color_s("{color}")
            }}

            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// border-inline-end-color: {color};
            /// ```
            fn border_e_{ident}(self) -> Self {{
                self.border_color_e("{color}")
            }}
            "#
        )
        .unwrap();
    }

    // outline-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// outline-color: {color};
            /// ```
            fn outline_{ident}(self) -> Self {{
                self.outline_color("{color}")
            }}
        "##
        )
        .unwrap();
    }

    // ring-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// box-shadow: ... {color};
            /// ```
            fn ring_{ident}(self) -> Self {{
                self.ring_color("{color}")
            }}
            "##
        )
        .unwrap();
    }

    // shadow-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// <b style="color:{color}">⏺</b>
            fn shadow_{ident}(self) -> Self {{
                self.shadow_color("{color}")
            }}
            "##
        )
        .unwrap();
    }

    // from-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// background-image: linear-gradient(..., {color}, ...);
            /// ```
            fn from_{ident}(self) -> Self {{
                self.from("{color}")
            }}
            "##
        )
        .unwrap();
    }

    // via-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// background-image: linear-gradient(..., ..., {color}, ...);
            /// ```
            fn via_{ident}(self) -> Self {{
                self.via("{color}")
            }}
            "##
        )
        .unwrap();
    }

    // to-color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// background-image: linear-gradient(..., ..., {color});
            /// ```
            fn to_{ident}(self) -> Self {{
                self.to("{color}")
            }}
            "##
        )
        .unwrap();
    }

    // font-size
    for (ident, font_size, line_height) in theme.font_sizes {
        let ident = ident.to_lowercase();
        match line_height {
            LineHeight::Length(line_height) => {
                writeln!(
                    out,
                    r##"
                    /// ```css
                    /// font-size: {font_size};
                    /// line-height: {line_height};
                    /// ```
                    fn text_{ident}(self) -> Self {{
                        self.text_size({font_size:?}).leading({line_height:?})
                    }}
                    "##
                )
                .unwrap();
            }
            LineHeight::Multiple(multiple) => {
                writeln!(
                    out,
                    r##"
                    /// ```css
                    /// font-size: {font_size};
                    /// line-height: {line_height};
                    /// ```
                    fn text_{ident}(self) -> Self {{
                        self.text_size({font_size:?}).leading_multiple({multiple:.2})
                    }}
                    "##,
                    multiple = f32::from(*multiple)
                )
                .unwrap();
            }
        }
    }

    // font-family
    for (ident, font_family) in theme.font_families {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// ```css
            /// font-family: {font_family};
            /// ```
            fn font_{ident}(self) -> Self {{
                self.font_family(r#"{font_family}"#)
            }}
            "##
        )
        .unwrap();
    }

    // rounded
    for (ident, size) in theme.rounded {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-radius: {size};
            /// ```
            fn rounded_{ident}(self) -> Self {{
                self.rounded({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-top-right-radius: {size};
            /// border-top-left-radius: {size};
            /// ```
            fn rounded_t_{ident}(self) -> Self {{
                self.rounded_t({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-top-right-radius: {size};
            /// border-bottom-right-radius: {size};
            /// ```
            fn rounded_r_{ident}(self) -> Self {{
                self.rounded_r({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-bottom-right-radius: {size};
            /// border-bottom-left-radius: {size};
            /// ```
            fn rounded_b_{ident}(self) -> Self {{
                self.rounded_b({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-top-left-radius: {size};
            /// border-bottom-left-radius: {size};
            /// ```
            fn rounded_l_{ident}(self) -> Self {{
                self.rounded_l({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-top-left-radius: {size};
            /// ```
            fn rounded_tl_{ident}(self) -> Self {{
                self.rounded_tl({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-top-right-radius: {size};
            /// ```
            fn rounded_tr_{ident}(self) -> Self {{
                self.rounded_tr({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-bottom-right-radius: {size};
            /// ```
            fn rounded_br_{ident}(self) -> Self {{
                self.rounded_br({size:?})
            }}
            "##
        )
        .unwrap();
        writeln!(
            out,
            r##"
            /// ```css
            /// border-bottom-left-radius: {size};
            /// ```
            fn rounded_bl_{ident}(self) -> Self {{
                self.rounded_bl({size:?})
            }}
            "##
        )
        .unwrap();
    }

    writeln!(out, r#"}}"#).unwrap();

    writeln!(out, r#"pub trait ThemeSubExt: SubStyle {{"#).unwrap();

    // breakpoints
    for (ident, min_width_px) in theme.breakpoints {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// ```css
            /// @media (min-width: {min_width_px}px)
            /// ```
            fn {ident}<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {{
                self.min_page_width({min_width_px:?}, f)
            }}
            "##
        )
        .unwrap();
    }
    for window in theme.breakpoints.windows(2) {
        let &[(ident, _), (_, min_width_next_px)] = window else {
            unreachable!()
        };
        let max_width_px = min_width_next_px.saturating_sub(1);
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r##"
            /// ```css
            /// @media (max-width: {max_width_px}px)
            /// ```
            fn max_{ident}<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(self, f: F) -> Self {{
                self.max_page_width({max_width_px:?}, f)
            }}
            "##
        )
        .unwrap();
    }

    // divide color
    for (ident, color) in theme.colors {
        let ident = ident.to_lowercase();
        writeln!(
            out,
            r#"
            /// <b style="color:{color}">⏺</b>
            /// ```css
            /// & > :not(:last-child) {{
            ///     border-color: {color};
            /// }}
            /// ```
            fn divide_{ident}(self) -> Self {{
                self.divide_color("{color}")
            }}
            "#
        )
        .unwrap();
    }

    writeln!(out, r#"}}"#).unwrap();
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
                ("BLACK", "#000000"),
                ("WHITE", "#ffffff"),
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
                ("SLATE_950", "#020617"),
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
                ("GRAY_950", "#030712"),
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
                ("ZINC_950", "#09090b"),
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
                ("NEUTRAL_950", "#0a0a0a"),
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
                ("STONE_950", "#0c0a09"),
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
                ("RED_950", "#450a0a"),
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
                ("ORANGE_950", "#431407"),
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
                ("AMBER_950", "#451a03"),
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
                ("YELLOW_950", "#422006"),
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
                ("LIME_950", "#1a2e05"),
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
                ("GREEN_950", "#052e16"),
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
                ("EMERALD_950", "#022c22"),
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
                ("TEAL_950", "#042f2e"),
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
                ("CYAN_950", "#083344"),
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
                ("SKY_950", "#082f49"),
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
                ("BLUE_950", "#172554"),
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
                ("INDIGO_950", "#1e1b4b"),
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
                ("VIOLET_950", "#2e1065"),
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
                ("PURPLE_950", "#3b0764"),
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
                ("FUCHSIA_950", "#4a044e"),
                ("PINK_50", "#fdf2f8"),
                ("PINK_100", "#fce7f3"),
                ("PINK_200", "#fbcfe8"),
                ("PINK_300", "#f9a8d4"),
                ("PINK_400", "#f472b6"),
                ("PINK_500", "#ec4899"),
                ("PINK_600", "#db2777"),
                ("PINK_700", "#be185d"),
                ("PINK_800", "#9d174d"),
                ("PINK_900", "#831843"),
                ("PINK_950", "#500724"),
                ("ROSE_50", "#fff1f2"),
                ("ROSE_100", "#ffe4e6"),
                ("ROSE_200", "#fecdd3"),
                ("ROSE_300", "#fda4af"),
                ("ROSE_400", "#fb7185"),
                ("ROSE_500", "#f43f5e"),
                ("ROSE_600", "#e11d48"),
                ("ROSE_700", "#be123c"),
                ("ROSE_800", "#9f1239"),
                ("ROSE_900", "#881337"),
                ("ROSE_950", "#4c0519"),
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
            Length::Auto => f.write_str("Length::Auto"),
            Length::Px(x) => write!(f, "Length::Px(Float::from({x}))"),
            Length::Rem(x) => write!(f, "Length::Rem(Float::from({x}))"),
            Length::Percent(x) => write!(f, "Length::Percent(Float::from({x}))"),
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

// FIXME: delete
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
