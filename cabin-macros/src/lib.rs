mod boundaries_macro;
mod boundary_attribute;
mod derive_attribute;
mod derive_event;
mod length_aliases_attribute;
mod view_macro_attribute;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{DeriveInput, ItemFn, ItemTrait, Type, parse_macro_input};

#[proc_macro_derive(Attribute, attributes(attribute))]
pub fn derive_attribute(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match derive_attribute::derive_attribute(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(Event)]
pub fn derive_event(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match derive_event::derive_event(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn boundary(attr: TokenStream, item: TokenStream) -> TokenStream {
    let events = parse_macro_input!(attr with Punctuated::<Type, Comma>::parse_terminated);
    let input = parse_macro_input!(item as ItemFn);
    match boundary_attribute::boundary_attribute(input, events, false) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn length_aliases(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "no arguments accepted",
        )
        .to_compile_error()
        .into();
    }
    let input = parse_macro_input!(item as ItemTrait);
    match length_aliases_attribute::length_aliases_attribute(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn wasm_boundary(attr: TokenStream, item: TokenStream) -> TokenStream {
    let events = parse_macro_input!(attr with Punctuated::<Type, Comma>::parse_terminated);
    let input = parse_macro_input!(item as ItemFn);
    match boundary_attribute::boundary_attribute(input, events, true) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn view_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as view_macro_attribute::ModulePathAttribute);
    let input = parse_macro_input!(item as ItemFn);
    match view_macro_attribute::view_macro_attribute(attr, input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn BOUNDARIES(item: TokenStream) -> TokenStream {
    if !item.is_empty() {
        panic!("no arguments expected");
    }
    boundaries_macro::boundaries_macro()
}
