use std::fmt;

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;
use crate::style::units::length::Length;

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct Gradient {
    pub gradient_line: Option<&'static str>,
    pub from_color: Option<&'static str>,
    pub from_position: Option<Length>,
    pub via_color: Option<&'static str>,
    pub via_position: Option<Length>,
    pub to_color: Option<&'static str>,
    pub to_position: Option<Length>,
}

impl PropertyDisplay for Gradient {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(((gradient_line, from_color), to_color)) =
            self.gradient_line.zip(self.from_color).zip(self.to_color)
        {
            write!(f, "{name}: linear-gradient({gradient_line}, {from_color}")?;
            if let Some(from_position) = self.from_position {
                write!(f, " {from_position}")?;
            }
            if let Some(via_color) = self.via_color {
                write!(f, ", {via_color}")?;
                if let Some(via_position) = self.via_position {
                    write!(f, " {via_position}")?;
                }
            }
            write!(f, ", {to_color}")?;
            if let Some(to_position) = self.to_position {
                write!(f, " {to_position}")?;
            }
            writeln!(f, ");")?;
        }
        Ok(())
    }
}

impl MergeFrom for Gradient {
    fn merge_from(&mut self, other: Self) {
        let Gradient {
            gradient_line,
            from_color,
            from_position,
            via_color,
            via_position,
            to_color,
            to_position,
        } = other;
        self.gradient_line.merge_from(gradient_line);
        self.from_color.merge_from(from_color);
        self.from_position.merge_from(from_position);
        self.via_color.merge_from(via_color);
        self.via_position.merge_from(via_position);
        self.to_color.merge_from(to_color);
        self.to_position.merge_from(to_position);
    }
}
