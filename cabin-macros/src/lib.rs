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

#[derive(Debug, Hash)]
enum StyleExpr {
    Path {
        path: Path,
    },
    Call {
        func: Path,
        paren_token: Paren,
        args: Punctuated<ExprLit, Comma>,
    },
    MethodCalls {
        path: Path,
        method_calls: Vec<StyleMethodCall>,
    },
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
            })
        } else if input.peek(Dot) {
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

            Ok(StyleExpr::MethodCalls { path, method_calls })
        } else {
            Ok(StyleExpr::Path { path })
        }
    }
}

impl ToTokens for StyleExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            StyleExpr::Path { path } => {
                path.to_tokens(tokens);
            }
            StyleExpr::Call {
                func,
                paren_token,
                args,
            } => {
                func.to_tokens(tokens);
                paren_token.surround(tokens, |tokens| {
                    args.to_tokens(tokens);
                });
            }
            StyleExpr::MethodCalls { path, method_calls } => {
                path.to_tokens(tokens);
                for method_call in method_calls {
                    method_call.to_tokens(tokens);
                }
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
