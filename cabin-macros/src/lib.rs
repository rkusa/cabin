mod boundaries_macro;
mod boundary_attribute;
mod derive_attribute;
mod derive_event;
mod tw_macro;
mod view_macro_attribute;

use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{DeriveInput, ItemFn, Type, parse_macro_input};

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
pub fn tw0(item: TokenStream) -> TokenStream {
    tw_macro::tw_macro(item, 0)
}

#[proc_macro]
pub fn tw(item: TokenStream) -> TokenStream {
    tw_macro::tw_macro(item, 1)
}

#[proc_macro]
pub fn tw2(item: TokenStream) -> TokenStream {
    tw_macro::tw_macro(item, 2)
}

#[proc_macro]
pub fn tw3(item: TokenStream) -> TokenStream {
    tw_macro::tw_macro(item, 2)
}

#[proc_macro]
#[allow(non_snake_case)]
pub fn BOUNDARIES(item: TokenStream) -> TokenStream {
    if !item.is_empty() {
        panic!("no arguments expected");
    }
    boundaries_macro::boundaries_macro()
}
