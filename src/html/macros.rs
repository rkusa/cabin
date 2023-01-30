#[macro_export]
macro_rules! div {
    ($($x:tt)*) => ($crate::view![$crate::html::div(()); $($x)*])
}

#[macro_export]
macro_rules! ul {
    ($($x:tt)*) => ($crate::view![$crate::html::ul(()); $($x)*])
}

#[macro_export]
macro_rules! li {
    ($($x:tt)*) => ($crate::view![$crate::html::li(()); $($x)*])
}

#[macro_export]
macro_rules! fieldset {
    ($($x:tt)*) => ($crate::view![$crate::html::fieldset(()); $($x)*])
}

#[macro_export]
macro_rules! button {
    ($($x:tt)*) => ($crate::view![$crate::html::button(()); $($x)*])
}

pub use {button, div, fieldset, li, ul};
