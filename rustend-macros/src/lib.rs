use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Dot, Paren};
use syn::{parse_macro_input, Error, ExprLit, FnArg, Ident, Item, ItemFn, Path, Signature, Stmt};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item;
    let Signature {
        constness,
        asyncness,
        unsafety,
        abi,
        fn_token: _,
        ident,
        generics,
        paren_token: _,
        inputs,
        variadic,
        output,
    } = sig;

    if inputs.len() != 1 {
        return Error::new(
            inputs.span(),
            "Exactly one function argument expected (the component state)",
        )
        .into_compile_error()
        .into();
    }

    let (state_ident, state_type) = match &inputs[0] {
        arg @ FnArg::Receiver(_) => {
            return Error::new(arg.span(), "State cannot be a self argument")
                .into_compile_error()
                .into()
        }
        FnArg::Typed(pat_type) => (&pat_type.pat, &pat_type.ty),
    };

    // find actions (`fn`s inside of the components content)
    let mut actions = Vec::new();
    for stmt in &block.stmts {
        if let Stmt::Item(Item::Fn(f)) = stmt {
            if f.sig.inputs.len() != 2 {
                return Error::new(
                    f.sig.inputs.span(),
                    "Exactly two function arguments expected for actions \
                            (the component state and the action payload)",
                )
                .into_compile_error()
                .into();
            }

            match &f.sig.inputs[0] {
                arg @ FnArg::Receiver(_) => {
                    return Error::new(arg.span(), "State cannot be a self argument")
                        .into_compile_error()
                        .into()
                }
                arg @ FnArg::Typed(pat_type) => {
                    if &pat_type.ty != state_type {
                        return Error::new(arg.span(), "Action and component state type mismatch")
                            .into_compile_error()
                            .into();
                    }
                }
            }

            let payload_type = match &f.sig.inputs[1] {
                arg @ FnArg::Receiver(_) => {
                    return Error::new(arg.span(), "Payload cannot be a self argument")
                        .into_compile_error()
                        .into()
                }
                FnArg::Typed(pat_type) => &pat_type.ty,
            };

            let action_ident = &f.sig.ident;
            let name = action_ident.to_string();
            actions.push(quote! {
                r.register::<#state_type, #payload_type, _, _, _>(ID, #name, #action_ident, __inner);
            })
        }
    }

    let name = ident.to_string();
    let block = block.stmts;

    let wrapped_fn = quote! {
        #(#attrs)*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#state_ident: impl rustend::previous::FromPrevious<#state_type> + 'static) #output {
            static ID: ::rustend::component::ComponentId = ::rustend::component::ComponentId::new(module_path!(), #name);

            #constness async #unsafety #abi fn __inner #generics(#inputs #variadic) #output {
                // TODO: Get rid into_view()
                ::rustend::IntoView::into_view({
                    #[::linkme::distributed_slice(::rustend::component::registry::COMPONENT_FACTORIES)]
                    fn __register(r: &mut ::rustend::component::registry::ComponentRegistry) {
                        #(#actions)*
                    }

                    #(#block)*
                })
            }

            ::rustend::component::ServerComponent::new(ID, #state_ident, __inner)
        }
    };

    wrapped_fn.into()
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
    method: Ident,
    paren_token: Paren,
}

impl Parse for StyleExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = Path::parse(input)?;

        if input.peek(Paren) {
            let content;
            Ok(StyleExpr::Call {
                func: path,
                paren_token: syn::parenthesized!(content in input),
                args: content.parse_terminated(ExprLit::parse)?,
            })
        } else if input.peek(Dot) {
            let mut method_calls = Vec::with_capacity(1);
            while input.peek(Dot) {
                #[allow(unused)]
                let content;
                method_calls.push(StyleMethodCall {
                    dot_token: input.parse()?,
                    method: input.parse()?,
                    paren_token: syn::parenthesized!(content in input),
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
        paren_token.surround(tokens, |_| {});
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

    // TODO: sort before creating hash
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let name = format!("c{:x}", hasher.finish());

    // Partition into ones without any modifier, and the ones with modifieres
    let styles = input.styles.into_iter();

    quote! {
        {
            #[::linkme::distributed_slice(::rustend_css::registry::STYLES)]
            fn __register(r: &mut ::rustend_css::registry::StyleRegistry) {
                r.add(#name, &[#(&#styles,)*]);
            }

            ::rustend_css::ClassName(Some(::std::borrow::Cow::Borrowed(#name)))
        }
    }
    .into()
}
