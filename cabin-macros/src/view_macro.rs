use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

/// Expand a `view![a, b, c]` invocation.
pub fn view_macro(input: TokenStream) -> TokenStream {
    // Split into chunks on top-level commas via raw token iteration (which never fails, even for
    // half-typed code), and each chunk is emitted in expression position. Keeping the original
    // spans and placing every chunk into a normal expression context lets rust-analyzer parse the
    // expansion with its usual error recovery and offer completion inside each item even while a
    // sibling item is still malformed.
    let mut chunks: Vec<TokenStream> = vec![TokenStream::new()];
    for tt in input {
        match &tt {
            TokenTree::Punct(punct) if punct.as_char() == ',' => chunks.push(TokenStream::new()),
            _ => chunks.last_mut().unwrap().extend(std::iter::once(tt)),
        }
    }
    chunks.retain(|chunk| !chunk.is_empty());

    match chunks.split_first() {
        None => quote!(()),
        Some((head, [])) => quote!(#head),
        Some((head, tail)) => {
            quote!(::cabin::view::AnyView::new(#head) #(.appended(#tail))*)
        }
    }
}
