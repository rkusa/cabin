use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn derive_event(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data: _,
    } = input;

    if !generics.params.is_empty() {
        return Err(Error::new(ident.span(), "Events cannot have generics"));
    }

    let name = ident.to_string();
    Ok(quote! {
        #[automatically_derived]
        impl ::cabin::event::Event for #ident {
            const ID: &'static str =  concat!(module_path!(), "::", #name);
        }
    })
}
