use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_macro_input, Error, ExprCall, FnArg, Item, ItemFn, Signature, Stmt};

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
                    #[::linkme::distributed_slice(rustend::component::registry::COMPONENT_FACTORIES)]
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
struct Styles {
    styles: Punctuated<ExprCall, Comma>,
}

impl Parse for Styles {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Styles {
            styles: Punctuated::<ExprCall, Comma>::parse_terminated(input)?,
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

    let styles = input
        .styles
        .into_iter()
        .map(|expr| quote!(s.append(#expr);));

    quote! {
        {
            #[::linkme::distributed_slice(rustend::style::registry::STYLES)]
            fn __register(r: &mut ::rustend::style::registry::StyleRegistry) {
                r.add(#name, |mut s| {
                    #(#styles)*
                });
            }

            #name
        }
    }
    .into()
}
