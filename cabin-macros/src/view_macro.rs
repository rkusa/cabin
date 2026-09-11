use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

/// Expand a `view![a, b, c]` invocation.
pub fn view_macro(input: TokenStream) -> TokenStream {
    // Prefer a real parse: it correctly handles turbofish generics, qualified paths, comparison
    // operators etc. Only fall back to a lenient split (e.g. while the invocation is mid-edit in
    // an editor) so rust-analyzer still gets a usable expansion to run completion against.
    let chunks = match Punctuated::<Expr, Token![,]>::parse_terminated.parse2(input.clone()) {
        Ok(exprs) => exprs.into_iter().map(|expr| quote!(#expr)).collect(),
        Err(_) => split_lenient(input),
    };
    combine(chunks)
}

fn combine(chunks: Vec<TokenStream>) -> TokenStream {
    match chunks.split_first() {
        None => quote!(()),
        Some((head, [])) => quote!(#head),
        Some((head, tail)) => {
            quote!(::cabin::view::AnyView::new(#head) #(.appended(#tail))*)
        }
    }
}

/// Best-effort split on top-level commas that tolerates unparsable/incomplete input.
fn split_lenient(input: TokenStream) -> Vec<TokenStream> {
    let mut chunks: Vec<TokenStream> = vec![TokenStream::new()];
    // `proc_macro2` already groups `()`/`[]`/`{}`, but `<...>` isn't a real delimiter, so track
    // it ourselves: only treat `<` as opening a level after `::` (turbofish) or while already
    // inside one, closing on `>`. This misparses a bare `a < b, c`, but that's fine here since
    // this path only ever runs on already-invalid input.
    let mut angle_depth: u32 = 0;
    let mut colon_run = 0u8;

    for tt in input {
        let is_colon = matches!(&tt, TokenTree::Punct(p) if p.as_char() == ':');
        match &tt {
            TokenTree::Punct(p) if p.as_char() == ',' && angle_depth == 0 => {
                chunks.push(TokenStream::new());
            }
            TokenTree::Punct(p) if p.as_char() == '<' && (colon_run == 2 || angle_depth > 0) => {
                angle_depth += 1;
                chunks.last_mut().unwrap().extend(std::iter::once(tt.clone()));
            }
            TokenTree::Punct(p) if p.as_char() == '>' && angle_depth > 0 => {
                angle_depth -= 1;
                chunks.last_mut().unwrap().extend(std::iter::once(tt.clone()));
            }
            _ => chunks.last_mut().unwrap().extend(std::iter::once(tt.clone())),
        }
        // track whether the last one or two tokens were `:`, to detect `::`
        colon_run = if is_colon { (colon_run + 1).min(2) } else { 0 };
    }

    chunks.retain(|chunk| !chunk.is_empty());
    chunks
}
