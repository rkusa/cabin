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
        variants: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Result<(), io::Error> {
        let mod_ident = prefix
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(ident.to_lowercase()));
        writeln!(self.out, r#"pub mod {mod_ident} {{"#,)?;
        {
            writeln!(self.out, r#"use super::Style;"#)?;
            for (name, value) in variants {
                let name = name.to_uppercase().replace('-', "_");
                writeln!(self.out, r#"/// ```css"#)?;
                writeln!(self.out, r#"/// {}"#, template(value))?;
                writeln!(self.out, r#"/// ```"#)?;
                writeln!(
                    self.out,
                    r#"pub const {name}: internal::{ident} = internal::{ident}("{value}");"#
                )?;
            }
            writeln!(self.out, r#"mod internal {{"#)?;
            {
                writeln!(self.out, r#"pub struct {ident}(pub(super) &'static str);"#)?;
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
