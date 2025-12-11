use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn styles_macro() -> TokenStream {
    let ident = format_ident!(
        "{}_STYLES",
        std::env::var("CARGO_CRATE_NAME")
            .unwrap()
            .to_ascii_uppercase()
    );
    quote! {
        #[cfg(not(target_arch = "wasm32"))]
        mod __styles {
            #[::cabin::private::linkme::distributed_slice]
            #[linkme(crate = ::cabin::private::linkme)]
            pub static #ident: [fn(&mut ::cabin::tailwind::registry::StyleRegistry)] = [..];
        }
        #[cfg(not(target_arch = "wasm32"))]
        pub use __styles::#ident as STYLES;
    }
    .into()
}
