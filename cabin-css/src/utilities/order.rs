//! The order to lay out an item in a flex or grid container (`order`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/order>

use crate::Property;

const ORDER: &str = "order";

/// `order: -9999;`
pub const FIRST: Property<i32> = Property(ORDER, -9999);

/// `order: 9999;`
pub const LAST: Property<i32> = Property(ORDER, 9999);

/// `order: 0;`
pub const NONE: Property<i32> = Property(ORDER, 0);

/// `order: {n};`
pub fn order(n: u16) -> Property<u16> {
    Property(ORDER, n)
}
