use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn boundaries_macro() -> TokenStream {
    let ident = format_ident!(
        "{}_BOUNDARIES",
        std::env::var("CARGO_CRATE_NAME")
            .unwrap()
            .to_ascii_uppercase()
    );
    quote! {
        #[cfg(not(target_arch = "wasm32"))]
        mod __boundaries {
            #[::cabin::private::linkme::distributed_slice]
            #[linkme(crate = ::cabin::private::linkme)]
            pub static #ident: [fn(&mut ::cabin::boundary_registry::BoundaryRegistry)] = [..];
        }
        #[cfg(not(target_arch = "wasm32"))]
        pub use __boundaries::#ident as BOUNDARIES;
    }
    .into()
}
