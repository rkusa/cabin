pub mod aspect;
pub mod basis;
pub mod bg;
pub mod display;
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
pub mod text;
pub mod w;

pub use display::*;
pub use position::*;

use crate::StaticClass;

pub const GROUP: StaticClass = StaticClass("group");
