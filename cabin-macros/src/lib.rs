mod boundary_attribute;
mod derive_attribute;
mod derive_event;
mod styles_macro;
mod tw_macro;

use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Eq};
use syn::{parse_macro_input, DeriveInput, Expr, Ident, ItemFn, Type};

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

#[derive(Debug, Hash)]
struct OptionExpr {
    key: Ident,
    value: Option<Expr>,
}

impl Parse for OptionExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = Ident::parse(input)?;
        let value = if Option::<Eq>::parse(input)?.is_some() {
            Some(Expr::parse(input)?)
        } else {
            None
        };
        Ok(OptionExpr { key, value })
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
pub fn STYLES(item: TokenStream) -> TokenStream {
    if !item.is_empty() {
        panic!("no arguments expected");
    }
    styles_macro::styles_macro()
}
