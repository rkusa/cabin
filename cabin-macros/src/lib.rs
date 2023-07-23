mod derive_attribute;
mod element_attribute;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Dot, Eq, Paren};
use syn::{
    parse_macro_input, DeriveInput, Expr, ExprLit, Ident, ItemTrait, Path, TraitItem, TraitItemFn,
};

#[proc_macro_attribute]
pub fn element(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<OptionExpr, Comma>::parse_terminated);
    let input = parse_macro_input!(item as ItemTrait);
    let mut original = input.clone();
    let addition: proc_macro2::TokenStream =
        match element_attribute::element_attribute(args, input, false) {
            Ok(ts) => ts,
            Err(err) => return err.into_compile_error().into(),
        };
    for item in &mut original.items {
        if let TraitItem::Fn(TraitItemFn { attrs, .. }) = item {
            attrs.retain(|attr| !attr.meta.path().is_ident("element"));
        }
    }
    quote! {
        #original
        #addition
    }
    .into()
}

#[proc_macro_attribute]
pub fn attributes(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr with Punctuated::<OptionExpr, Comma>::parse_terminated);
    let input = parse_macro_input!(item as ItemTrait);
    let mut original = input.clone();
    let addition: proc_macro2::TokenStream =
        match element_attribute::element_attribute(args, input, true) {
            Ok(ts) => ts,
            Err(err) => return err.into_compile_error().into(),
        };
    for item in &mut original.items {
        if let TraitItem::Fn(TraitItemFn { attrs, .. }) = item {
            attrs.retain(|attr| !attr.meta.path().is_ident("element"));
        }
    }
    quote! {
        #original
        #addition
    }
    .into()
}

#[proc_macro_derive(Attribute, attributes(attribute))]
pub fn derive_attribute(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match derive_attribute::derive_attribute(input) {
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

#[derive(Debug, Hash)]
enum StyleExpr {
    // e.g.: css::bg::BLACK
    Path {
        path: Path,
        // e.g.:css::bg::BLACK.hover()
        method_calls: StyleMethodCalls,
    },
    // e.g.: css::w::px(46)
    Call {
        func: Path,
        paren_token: Paren,
        args: Punctuated<ExprLit, Comma>,
        // e.g.: css::w::px(46).hover()
        method_calls: StyleMethodCalls,
    },
    // e.g.: (css::bg::BLACK, css::w::px(46))
    Tuple {
        paren_token: Paren,
        args: Punctuated<StyleExpr, Comma>,
        method_calls: StyleMethodCalls,
    },
}

#[derive(Debug, Hash, Clone)]
struct StyleMethodCalls {
    method_calls: Option<Vec<StyleMethodCall>>,
}

#[derive(Debug, Hash, Clone)]
struct StyleMethodCall {
    dot_token: Dot,
    method: Option<Ident>,      // optional to allow incomplete inputs
    paren_token: Option<Paren>, // optional to allow incomplete inputs
}

impl Parse for StyleExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            let content;
            return Ok(StyleExpr::Tuple {
                paren_token: syn::parenthesized!(content in input),
                args: content.parse_terminated(StyleExpr::parse, Comma)?,
                method_calls: input.parse()?,
            });
        }

        let path = Path::parse(input)?;

        if input.peek(Paren) {
            let content;
            Ok(StyleExpr::Call {
                func: path,
                paren_token: syn::parenthesized!(content in input),
                args: content.parse_terminated(ExprLit::parse, Comma)?,
                method_calls: input.parse()?,
            })
        } else {
            Ok(StyleExpr::Path {
                path,
                method_calls: input.parse()?,
            })
        }
    }
}

impl Parse for StyleMethodCalls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut method_calls = Vec::with_capacity(1);
        while input.peek(Dot) {
            method_calls.push(StyleMethodCall {
                dot_token: input.parse()?,
                method: if input.peek(Ident) {
                    Some(input.parse()?)
                } else {
                    None
                },
                paren_token: if input.peek(Paren) {
                    #[allow(unused)]
                    let content;
                    Some(syn::parenthesized!(content in input))
                } else {
                    None
                },
            });
        }

        Ok(Self {
            method_calls: if method_calls.is_empty() {
                None
            } else {
                Some(method_calls)
            },
        })
    }
}

impl ToTokens for StyleExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            StyleExpr::Path { path, method_calls } => {
                path.to_tokens(tokens);
                method_calls.to_tokens(tokens);
            }
            StyleExpr::Call {
                func,
                paren_token,
                args,
                method_calls,
            } => {
                func.to_tokens(tokens);
                paren_token.surround(tokens, |tokens| {
                    args.to_tokens(tokens);
                });
                method_calls.to_tokens(tokens);
            }
            StyleExpr::Tuple {
                paren_token,
                args,
                method_calls,
            } => {
                paren_token.surround(tokens, |tokens| {
                    args.to_tokens(tokens);
                });
                method_calls.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for StyleMethodCall {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let StyleMethodCall {
            dot_token,
            method,
            paren_token,
        } = self;
        dot_token.to_tokens(tokens);
        method.to_tokens(tokens);
        if let Some(paren_token) = paren_token {
            paren_token.surround(tokens, |_| {});
        }
    }
}

impl ToTokens for StyleMethodCalls {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(method_calls) = &self.method_calls {
            for method_call in method_calls {
                method_call.to_tokens(tokens);
            }
        }
    }
}

#[derive(Debug, Hash)]
struct Styles {
    styles: Punctuated<StyleExpr, Comma>,
}

impl Parse for Styles {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Styles {
            styles: Punctuated::<StyleExpr, Comma>::parse_terminated(input)?,
        })
    }
}

fn flatten_recursively(iter: impl Iterator<Item = StyleExpr>) -> impl Iterator<Item = StyleExpr> {
    iter.flat_map(|style| match style {
        s @ StyleExpr::Path { .. } => Box::new(std::iter::once(s)),
        s @ StyleExpr::Call { .. } => Box::new(std::iter::once(s)),
        StyleExpr::Tuple {
            args,
            method_calls: parent_method_calls,
            ..
        } => Box::new(flatten_recursively(args.into_iter()).map(move |mut s| {
            if parent_method_calls.method_calls.is_some() {
                match &mut s {
                    StyleExpr::Call { method_calls, .. } | StyleExpr::Path { method_calls, .. } => {
                        if method_calls.method_calls.is_none() {
                            method_calls.method_calls = parent_method_calls.method_calls.clone();
                        } else {
                            method_calls
                                .method_calls
                                .as_mut()
                                .unwrap()
                                .extend(parent_method_calls.method_calls.clone().unwrap())
                        }
                    }
                    StyleExpr::Tuple { .. } => unreachable!(),
                }
            }
            s
        })) as Box<dyn Iterator<Item = StyleExpr>>,
    })
}

#[proc_macro]
pub fn tw(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Styles);
    // dbg!(&input);

    // TODO: Partition here into ones without any modifier, and the ones with modifieres?
    // Flatten tuples out
    let styles = flatten_recursively(input.styles.into_iter());

    quote! {
        {
            static NAME: ::cabin::private::OnceCell<String> = ::cabin::private::OnceCell::new();

            #[::cabin::private::linkme::distributed_slice(::cabin_tailwind::registry::STYLES)]
            #[linkme(crate = ::cabin::private::linkme)]
            fn __register(r: &mut ::cabin_tailwind::registry::StyleRegistry) {
                let name = r.add(&[#(&#styles,)*]);
                NAME.set(name).ok();
            }

            ::cabin_tailwind::Class(::std::borrow::Cow::Borrowed(
                NAME.get().map(|s| s.as_str()).unwrap_or_default()
            ))
        }
    }
    .into()
}
