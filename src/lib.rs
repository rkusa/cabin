#![cfg_attr(not(target_arch = "wasm32"), forbid(unsafe_code))]

extern crate self as cabin;

pub use cabin_macros::{Attribute, BOUNDARIES, Event, boundary};
pub use context::Context;
pub use error::Error;
pub use http::StatusCode;
pub use redirect::Redirect;
#[cfg(not(target_arch = "wasm32"))]
pub use server::{
    CABIN_JS, LIVERELOAD_JS, RenderFn, basic_document, cabin_scripts, content_hash, get_page,
    put_page,
};
pub use view::View;

mod attribute;
#[cfg(not(target_arch = "wasm32"))]
pub mod boundary_registry;
mod context;
pub mod element;
pub mod error;
pub mod event;
pub mod fire_event;
mod fragment;
pub mod html;
pub mod local_pool;
pub mod multipart;
pub mod pack;
pub mod prelude;
pub mod private;
mod redirect;
pub mod render;
mod renderer_pool;
pub mod serde;
#[cfg(not(target_arch = "wasm32"))]
mod server;
pub mod view;
pub mod void_element;
#[cfg(target_arch = "wasm32")]
mod wasm_exports;

pub mod wasm {
    pub use cabin_macros::wasm_boundary as boundary;

    #[cfg(target_arch = "wasm32")]
    pub use crate::wasm_exports::fail;
}
