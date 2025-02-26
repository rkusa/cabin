#![cfg_attr(not(target_arch = "wasm32"), forbid(unsafe_code))]

extern crate self as cabin;

pub use cabin_macros::{Attribute, Event, boundary};
pub use error::Error;
pub use html::h;
pub use http::StatusCode;
pub use redirect::Redirect;
#[cfg(not(target_arch = "wasm32"))]
pub use server::{
    CABIN_JS, LIVERELOAD_JS, basic_document, cabin_scripts, content_hash, get_page, put_page,
};
pub use view::View;

#[cfg(not(target_arch = "wasm32"))]
pub mod boundary_registry;
pub mod error;
pub mod event;
pub mod fire_event;
pub mod html;
pub mod multipart;
pub mod pack;
pub mod prelude;
pub mod private;
mod redirect;
pub mod render;
pub mod scope;
pub mod serde;
#[cfg(not(target_arch = "wasm32"))]
mod server;
pub mod view;
#[cfg(target_arch = "wasm32")]
mod wasm_exports;

pub mod wasm {
    pub use cabin_macros::wasm_boundary as boundary;

    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm_exports::fail;
}
