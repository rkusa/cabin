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

    let inner_ident = format_ident!("__{}", ident);

    let wrapped_fn = quote! {
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs #variadic) -> ::rust_html_over_wire::Component<#state_type, impl View<#state_type>> {

            #(#attrs)*
            #constness #asyncness #unsafety #abi fn #inner_ident #generics(#inputs #variadic) #output {
                #block
            }

            // TODO: component id
            ::rust_html_over_wire::Component::new("counter::counter", #state_ident, #inner_ident)
        }
    };

    wrapped_fn.into()
}
