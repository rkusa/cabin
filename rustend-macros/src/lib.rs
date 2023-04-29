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

    let state_type = match &inputs[0] {
        arg @ FnArg::Receiver(_) => {
            return Error::new(arg.span(), "State cannot be a self argument")
                .into_compile_error()
                .into()
        }
        FnArg::Typed(pat_type) => &pat_type.ty,
    };

    // find actions (`fn`s inside of the components content)
    let mut action_registrations = Vec::new();
    let (actions, other): (Vec<_>, Vec<_>) = block.stmts.into_iter().partition(|stmt| {
        if let Stmt::Item(Item::Fn(f)) = stmt {
            if f.sig.inputs.len() != 2 {
                return false;
                // return Error::new(
                //     f.sig.inputs.span(),
                //     "Exactly two function arguments expected for actions \
                //             (the component state and the action payload)",
                // )
                // .into_compile_error()
                // .into();
            }

            match &f.sig.inputs[0] {
                FnArg::Receiver(_) => {
                    return false;
                    // return Error::new(arg.span(), "State cannot be a self argument")
                    //     .into_compile_error()
                    //     .into()
                }
                FnArg::Typed(pat_type) => {
                    if &pat_type.ty != state_type {
                        return false;
                        // return Error::new(arg.span(), "Action and component state type mismatch")
                        //     .into_compile_error()
                        //     .into();
                    }
                }
            }

            let payload_type = match &f.sig.inputs[1] {
                FnArg::Receiver(_) => {
                    return false;
                    // return Error::new(arg.span(), "Payload cannot be a self argument")
                    //     .into_compile_error()
                    //     .into()
                }
                FnArg::Typed(pat_type) => &pat_type.ty,
            };

            let action_ident = &f.sig.ident;
            let name = action_ident.to_string();
            action_registrations.push(quote! {
                r.register::<#state_type, #payload_type, _, _, _, _>(ID, #name, #action_ident, #ident);
            });
            true
        } else {
            false
        }
    });

    let name = ident.to_string();

    let wrapped_fn = quote! {
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(__state: impl rustend::previous::FromPrevious<#state_type> + 'static) #output {
            static ID: ::rustend::component::ComponentId = ::rustend::component::ComponentId::new(module_path!(), #name);

            #(#actions)*

            #[::rustend::private::linkme::distributed_slice(::rustend::component::registry::COMPONENT_FACTORIES)]
            #[linkme(crate = ::rustend::private::linkme)]
            fn __register(r: &mut ::rustend::component::registry::ComponentRegistry) {
                #(#action_registrations)*
            }

            #(#attrs)*
            #constness async #unsafety #abi fn #ident #generics(#inputs #variadic) #output {
                #(#other)*
            }

            Ok(::rustend::component::ServerComponent::new(ID, __state, #ident))
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
                args: content.parse_terminated(ExprLit::parse)?,
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
            static NAME: ::rustend::private::OnceCell<String> = ::rustend::private::OnceCell::new();

            #[::rustend::private::linkme::distributed_slice(::rustend_css::registry::STYLES)]
            #[linkme(crate = ::rustend::private::linkme)]
            fn __register(r: &mut ::rustend_css::registry::StyleRegistry) {
                let name = r.add(&[#(&#styles,)*]);
                NAME.set(name).ok();
            }

            ::rustend_css::ClassName(Some(::std::borrow::Cow::Borrowed(
                NAME.get().map(|s| s.as_str()).unwrap_or_default()
            )))
        }
    }
    .into()
}
