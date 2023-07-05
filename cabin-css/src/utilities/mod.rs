pub mod aspect;
pub mod basis;
pub mod bg;
pub mod border;
pub mod bottom;
pub mod display;
pub mod end;
pub mod flex;
pub mod font;
pub mod gap;
pub mod gap_x;
pub mod gap_y;
pub mod grid;
pub mod h;
pub mod inset;
pub mod items;
pub mod justify;
pub mod left;
pub mod m;
pub mod max_h;
pub mod max_w;
pub mod mb;
pub mod me;
pub mod min_h;
pub mod min_w;
pub mod ml;
pub mod mr;
pub mod ms;
pub mod mt;
pub mod mx;
pub mod my;
pub mod order;
pub mod outline;
pub mod p;
pub mod pb;
pub mod pl;
pub mod position;
pub mod pr;
pub mod pt;
pub mod px;
pub mod py;
pub mod right;
pub mod ring;
pub mod rounded;
pub mod shadow;
pub mod start;
pub mod text;
pub mod top;
pub mod w;
pub mod whitespace;
pub mod z;

use std::fmt;

pub use basis::unit as basis;
pub use border::{px as border, PX as BORDER};
pub use bottom::unit as bottom;
pub use display::*;
pub use end::unit as end;
pub use flex::{NO_SHRINK, SHRINK};
pub use gap::unit as gap;
pub use gap_x::unit as gap_x;
pub use gap_y::unit as gap_y;
pub use h::unit as h;
pub use inset::unit as inset;
pub use left::unit as left;
pub use m::unit as m;
pub use max_h::unit as max_h;
pub use max_w::unit as max_w;
pub use mb::unit as mb;
pub use me::unit as me;
pub use min_h::unit as min_h;
pub use min_w::unit as min_w;
pub use ml::unit as ml;
pub use mr::unit as mr;
pub use ms::unit as ms;
pub use mt::unit as mt;
pub use mx::unit as mx;
pub use my::unit as my;
pub use order::order;
pub use outline::{offset as outline, SOLID as OUTLINE};
pub use p::unit as p;
pub use pb::unit as pb;
pub use pl::unit as pl;
pub use position::*;
pub use pr::unit as pr;
pub use pt::unit as pt;
pub use px::unit as px;
pub use py::unit as py;
pub use right::unit as right;
pub use ring::width as ring;
pub use ring::DEFAULT as RING;
pub use rounded::DEFAULT as ROUNDED;
pub use shadow::DEFAULT as SHADOW;
pub use start::unit as start;
pub use top::unit as top;
pub use w::unit as w;
pub use z::index as z;

use crate::{StaticClass, Style};

pub const GROUP: StaticClass = StaticClass("group");

pub struct SrOnly(());
pub struct NotSrOnly(());

/// Hide an element visually without hiding it from screen readers.
pub const SR_ONLY: SrOnly = SrOnly(());

/// Undo [SR_ONLY].
pub const NOT_SR_ONLY: NotSrOnly = NotSrOnly(());

impl Style for SrOnly {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "position: absolute;")?;
        writeln!(f, "width: 1px;")?;
        writeln!(f, "height: 1px;")?;
        writeln!(f, "padding: 0;")?;
        writeln!(f, "margin: -1px;")?;
        writeln!(f, "overflow: hidden;")?;
        writeln!(f, "clip: rect(0, 0, 0, 0);")?;
        writeln!(f, "white-space: nowrap;")?;
        writeln!(f, "border-width: 0;")?;
        Ok(())
    }
}

impl Style for NotSrOnly {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "position: static;")?;
        writeln!(f, "width: auto;")?;
        writeln!(f, "height: auto;")?;
        writeln!(f, "padding: 0;")?;
        writeln!(f, "margin: 0;")?;
        writeln!(f, "overflow: visible;")?;
        writeln!(f, "clip: auto;")?;
        writeln!(f, "white-space: normal;")?;
        Ok(())
    }
}
