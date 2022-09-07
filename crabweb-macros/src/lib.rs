use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, FnArg, ItemFn, Signature};

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

    let factory_ident = format_ident!("__register_{}", ident);
    let name = ident.to_string();

    let wrapped_fn = quote! {
        #(#attrs)*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs #variadic) #output {
            ::crabweb::Component::<#state_type, _, _>::new(module_path!(), #name, #state_ident, |#inputs #variadic| #block)
        }

        #[::linkme::distributed_slice(::crabweb::component::registry::COMPONENT_FACTORIES)]
        fn #factory_ident(r: &mut ::crabweb::component::registry::ComponentRegistry) {
            r.register::<#state_type, _, _>(module_path!(), #name, #ident);
        }
    };

    wrapped_fn.into()
}

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
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

    let original_ident = format_ident!("__{}", ident);
    let factory_ident = format_ident!("__register_{}", ident);
    let name = ident.to_string();

    let wrapped_fn = quote! {
        #(#attrs)*
        #constness #asyncness #unsafety #abi fn #original_ident #generics(#inputs #variadic) #output {
            #block
        }

        #[allow(non_upper_case_globals)]
        #vis const #ident: ::crabweb::action::Action<#state_type> =
            ::crabweb::action::Action::new(module_path!(), #name, #original_ident);

        #[::linkme::distributed_slice(::crabweb::action::registry::ACTION_FACTORIES)]
        fn #factory_ident(r: &mut ::crabweb::action::registry::ActionRegistry) {
            r.register(module_path!(), #name, #ident);
        }
    };

    wrapped_fn.into()
}

#[proc_macro_attribute]
pub fn event(_attr: TokenStream, item: TokenStream) -> TokenStream {
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

    if inputs.len() != 2 {
        return Error::new(
            inputs.span(),
            "Exactly two function argument expected (the component state and the event)",
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
    let event_type = match &inputs[1] {
        arg @ FnArg::Receiver(_) => {
            return Error::new(arg.span(), "Event cannot be a self argument")
                .into_compile_error()
                .into()
        }
        FnArg::Typed(pat_type) => &pat_type.ty,
    };

    let original_ident = format_ident!("__{}", ident);
    let factory_ident = format_ident!("__register_{}", ident);
    let name = ident.to_string();

    let wrapped_fn = quote! {
        #(#attrs)*
        #constness #asyncness #unsafety #abi fn #original_ident #generics(#inputs #variadic) #output {
            #block
        }

        #[allow(non_upper_case_globals)]
        #vis const #ident: ::crabweb::action::EventAction<#state_type, #event_type> =
            ::crabweb::action::EventAction::new(module_path!(), #name, #original_ident);

        #[::linkme::distributed_slice(::crabweb::action::registry::ACTION_FACTORIES)]
        fn #factory_ident(r: &mut ::crabweb::action::registry::ActionRegistry) {
            r.register_event(module_path!(), #name, #ident);
        }
    };

    wrapped_fn.into()
}
