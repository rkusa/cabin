#[macro_export]
macro_rules! div {
    ($($x:tt)*) => ($crate::html::custom("div", $crate::view![$($x)*]))
}

#[macro_export]
macro_rules! ul {
    ($($x:tt)*) => ($crate::html::custom("ul", $crate::view![$($x)*]))
}

#[macro_export]
macro_rules! li {
    ($($x:tt)*) => ($crate::html::custom("li", $crate::view![$($x)*]))
}

#[macro_export]
macro_rules! button {
    ($($x:tt)*) => ($crate::html::custom("button", $crate::view![$($x)*]))
}

pub use {button, div, li, ul};
