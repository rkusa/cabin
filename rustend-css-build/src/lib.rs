use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("rustend-css-build.rs");
    let mut b = Builder {
        out: &mut File::create(path)?,
    };

    b.add_utility(
        "AspectRatio",
        Some("aspect"),
        |v| format!("aspect-ratio: {v};"),
        [("auto", "auto"), ("square", "1 / 1"), ("video", "16 / 9")],
    )?;

    b.add_utility(
        "Display",
        None,
        |v| format!("display: {v};"),
        [
            ("block", "block"),
            ("inline-block", "inline-block"),
            ("inline", "inline"),
            ("flex", "flex"),
            ("inline-flex", "inline-flex"),
            ("table", "table"),
            ("inline-table", "inline-table"),
            ("table-caption", "table-caption"),
            ("table-cell", "table-cell"),
            ("table-column", "table-column"),
            ("table-column-group", "table-column-group"),
            ("table-footer-group", "table-footer-group"),
            ("table-header-group", "table-header-group"),
            ("table-row-group", "table-row-group"),
            ("table-row", "table-row"),
            ("flow-root", "flow-root"),
            ("grid", "grid"),
            ("inline-grid", "inline-grid"),
            ("contents", "contents"),
            ("list-item", "list-item"),
            ("hidden", "hidden"),
        ],
    )?;

    b.add_utility(
        "Position",
        None,
        |v| format!("position: {v};"),
        [
            ("static", "static"),
            ("relative", "relative"),
            ("absolute", "absolute"),
            ("sticky", "sticky"),
            ("fixed", "fixed"),
        ],
    )?;

    // Flexbox & Grid

    b.add_utility(
        "FlexBasis",
        Some("basis"),
        |v| format!("flex-basis: {v};"),
        Spacing,
    )?;

    Ok(())
}

struct Builder<'a> {
    out: &'a mut dyn io::Write,
}

impl<'a> Builder<'a> {
    fn add_utility(
        &mut self,
        ident: &str,
        prefix: Option<&str>,
        template: impl Fn(&str) -> String,
        variants: impl Variants,
    ) -> Result<(), io::Error> {
        let mod_ident = prefix
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(ident.to_lowercase()));
        writeln!(self.out, r#"pub mod {mod_ident} {{"#,)?;
        {
            writeln!(self.out, r#"#[allow(unused_imports)]"#)?;
            writeln!(self.out, r#"use super::{{Length, Style}};"#)?;
            variants.write_variants(ident, &template, self.out)?;
            writeln!(self.out, r#"mod internal {{"#)?;
            {
                writeln!(
                    self.out,
                    r#"pub struct {ident}(pub(super) {});"#,
                    variants.inner_type()
                )?;
                writeln!(self.out, r#"impl super::Style for {ident} {{"#)?;
                {
                    writeln!(
                        self.out,
                        r#"fn declarations(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{"#
                    )?;
                    {
                        writeln!(self.out, r#"writeln!(f, "{}", self.0)"#, template("{}"))?;
                    }
                    writeln!(self.out, r#"}}"#)?;
                }
                writeln!(self.out, r#"}}"#)?;
            }
            writeln!(self.out, r#"}}"#)?;
        }
        writeln!(self.out, r#"}}"#)?;

        if prefix.is_none() {
            writeln!(self.out, r#"pub use {mod_ident}::*;"#)?;
        }

        Ok(())
    }
}

trait Variants {
    fn write_variants<T: Fn(&str) -> String>(
        &self,
        ident: &str,
        template: T,
        out: &mut dyn io::Write,
    ) -> io::Result<()>;

    fn inner_type(&self) -> &'static str {
        "&'static str"
    }
}

struct Spacing;

impl Variants for Spacing {
    fn write_variants<T: Fn(&str) -> String>(
        &self,
        ident: &str,
        template: T,
        out: &mut dyn io::Write,
    ) -> io::Result<()> {
        for (name, value, formatted) in [
            ("ZERO", "super::Length::Px(0.0)", "0"),
            ("auto", "super::Length::Auto", "auto"),
            ("px", "super::Length::Px(1.0)", "1px"),
            ("full", "super::Length::Percent(100.0)", "100%"),
        ] {
            let name = name.to_uppercase().replace('-', "_");
            writeln!(out, r#"/// ```css"#)?;
            writeln!(out, r#"/// {}"#, (template)(formatted))?;
            writeln!(out, r#"/// ```"#)?;
            writeln!(
                out,
                r#"pub const {name}: internal::{ident} = internal::{ident}({value});"#
            )?;
        }

        writeln!(out, r#"/// Multiple of `0.25rem` (`4px` by default)."#,)?;
        writeln!(out, r#"/// ```css"#)?;
        writeln!(out, r#"/// {}"#, (template)("{unit * 0.25}rem"))?;
        writeln!(out, r#"/// ```"#)?;
        writeln!(out, r#"pub fn unit(unit: u16) -> internal::{ident} {{"#)?;
        writeln!(
            out,
            r#"  internal::{ident}(super::Length::Rem(f32::from(unit) * 0.25))"#
        )?;
        writeln!(out, r#"}}"#)?;

        writeln!(out, r#"/// Multiple of `0.25rem` (`4px` by default)."#,)?;
        writeln!(out, r#"/// ```css"#)?;
        writeln!(out, r#"/// {}"#, (template)("{unit * 0.25}rem"))?;
        writeln!(out, r#"/// ```"#)?;
        writeln!(out, r#"pub fn unitf(unit: f32) -> internal::{ident} {{"#)?;
        writeln!(
            out,
            r#"  internal::{ident}(super::Length::Rem(unit * 0.25))"#
        )?;
        writeln!(out, r#"}}"#)?;

        writeln!(out, r#"/// ```css"#)?;
        writeln!(out, r#"/// {}"#, (template)("{rem}rem"))?;
        writeln!(out, r#"/// ```"#)?;
        writeln!(out, r#"pub fn rem(rem: f32) -> internal::{ident} {{"#)?;
        writeln!(out, r#"  internal::{ident}(super::Length::Rem(rem))"#)?;
        writeln!(out, r#"}}"#)?;

        writeln!(out, r#"/// ```css"#)?;
        writeln!(out, r#"/// {}"#, (template)("{px}px"))?;
        writeln!(out, r#"/// ```"#)?;
        writeln!(out, r#"pub fn px(px: f32) -> internal::{ident} {{"#)?;
        writeln!(out, r#"  internal::{ident}(super::Length::Px(px))"#)?;
        writeln!(out, r#"}}"#)?;

        writeln!(out, r#"/// ```css"#)?;
        writeln!(out, r#"/// {}"#, (template)("{percent}%"))?;
        writeln!(out, r#"/// ```"#)?;
        writeln!(
            out,
            r#"pub fn percent(percent: f32) -> internal::{ident} {{"#
        )?;
        writeln!(
            out,
            r#"  internal::{ident}(super::Length::Percent(percent))"#
        )?;
        writeln!(out, r#"}}"#)?;

        Ok(())
    }

    fn inner_type(&self) -> &'static str {
        "super::Length"
    }
}

impl<const N: usize> Variants for [(&'static str, &'static str); N] {
    fn write_variants<T: Fn(&str) -> String>(
        &self,
        ident: &str,
        template: T,
        out: &mut dyn io::Write,
    ) -> io::Result<()> {
        for (name, value) in self.iter() {
            let name = name.to_uppercase().replace('-', "_");
            writeln!(out, r#"/// ```css"#)?;
            writeln!(out, r#"/// {}"#, (template)(value))?;
            writeln!(out, r#"/// ```"#)?;
            writeln!(
                out,
                r#"pub const {name}: internal::{ident} = internal::{ident}("{value}");"#
            )?;
        }
        Ok(())
    }
}
