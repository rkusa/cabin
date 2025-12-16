use std::fmt;

use crate::style::property_display::PropertyDisplay;

#[derive(Clone)]
pub struct GridLines {
    pub start: Option<GridLine>,
    pub end: Option<GridLine>,
}

#[derive(Clone, Copy)]
pub enum GridLine {
    Auto,
    Nth(i16),
    Span(u16),
}

impl GridLines {
    pub fn set(&mut self, grid_line: GridLine) {
        self.start = Some(grid_line);
        self.end = Some(grid_line);
    }

    pub fn set_start(&mut self, grid_line: GridLine) {
        self.start = Some(grid_line);
    }

    pub fn set_end(&mut self, grid_line: GridLine) {
        self.end = Some(grid_line);
    }
}

impl PropertyDisplay for GridLines {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((start, end)) = self.start.zip(self.end) {
            writeln!(f, "{name}: {start} / {end};")?;
        } else {
            if let Some(start) = self.start {
                writeln!(f, "{name}-start: {start};")?;
            }
            if let Some(end) = self.end {
                writeln!(f, "{name}-end: {end};")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for GridLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridLine::Auto => f.write_str("auto"),
            GridLine::Nth(n) => write!(f, "{n}"),
            GridLine::Span(n) => write!(f, "span {n}"),
        }
    }
}

impl Default for GridLines {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
}
