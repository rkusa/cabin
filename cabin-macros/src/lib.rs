mod derive_attributes;
mod derive_element;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Dot, Paren};
use syn::{parse_macro_input, DeriveInput, ExprLit, Ident, Path};

#[proc_macro_derive(Element, attributes(element))]
pub fn derive_element(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match derive_element::derive_element(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(Attributes, attributes(attributes))]
pub fn derive_attributes(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    match derive_attributes::derive_attributes(input) {
        Ok(ts) => ts.into(),
        Err(err) => err.into_compile_error().into(),
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
}

#[derive(Debug, Hash)]
struct StyleMethodCalls {
    method_calls: Option<Vec<StyleMethodCall>>,
}

#[derive(Debug, Hash)]
struct StyleMethodCall {
    dot_token: Dot,
    method: Option<Ident>,      // optional to allow incomplete inputs
    paren_token: Option<Paren>, // optional to allow incomplete inputs
}

impl Parse for StyleExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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

#[proc_macro]
pub fn css(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Styles);
    // dbg!(&input);

    // Partition into ones without any modifier, and the ones with modifieres
    let styles = input.styles.into_iter();

    quote! {
        {
            static NAME: ::cabin::private::OnceCell<String> = ::cabin::private::OnceCell::new();

            #[::cabin::private::linkme::distributed_slice(::cabin_css::registry::STYLES)]
            #[linkme(crate = ::cabin::private::linkme)]
            fn __register(r: &mut ::cabin_css::registry::StyleRegistry) {
                let name = r.add(&[#(&#styles,)*]);
                NAME.set(name).ok();
            }

            ::cabin_css::ClassName(Some(::std::borrow::Cow::Borrowed(
                NAME.get().map(|s| s.as_str()).unwrap_or_default()
            )))
        }
    }
    .into()
}
