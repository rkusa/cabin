pub mod aspect;
pub mod basis;
pub mod bg;
pub mod display;
pub mod font;
pub mod grid_cols;
pub mod m;
pub mod max_w;
pub mod mb;
pub mod min_w;
pub mod ml;
pub mod mr;
pub mod mt;
pub mod mx;
pub mod my;
pub mod p;
pub mod pb;
pub mod pl;
pub mod position;
pub mod pr;
pub mod pt;
pub mod px;
pub mod py;
pub mod rounded;
pub mod text;
pub mod w;

pub use basis::unit as basis;
pub use display::*;
pub use m::unit as m;
pub use max_w::unit as max_w;
pub use mb::unit as mb;
pub use min_w::unit as min_w;
pub use ml::unit as ml;
pub use mr::unit as mr;
pub use mt::unit as mt;
pub use mx::unit as mx;
pub use my::unit as my;
pub use p::unit as p;
pub use pb::unit as pb;
pub use pl::unit as pl;
pub use position::*;
pub use pr::unit as pr;
pub use pt::unit as pt;
pub use px::unit as px;
pub use py::unit as py;
pub use w::unit as w;

use crate::StaticClass;

pub const GROUP: StaticClass = StaticClass("group");
