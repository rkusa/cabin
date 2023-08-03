use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, FnArg, ItemFn, Pat, PatType, Signature};

pub fn boundary_attribute(item: ItemFn) -> syn::Result<TokenStream> {
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

    let inner_ident = format_ident!("__{ident}");
    let name = ident.to_string();
    let (args_idents, args_types) = inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => Err(Error::new(
                input.span(),
                "boundary cannot have self argument",
            )),
            FnArg::Typed(PatType { pat, ty, .. }) => match pat.as_ref() {
                Pat::Ident(ref ident) => Ok((ident.ident.clone(), ty.clone())),
                pat => Err(Error::new(pat.span(), "boundary arguments must be idents")),
            },
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .unzip::<_, _, Punctuated<_, Comma>, Punctuated<_, Comma>>();

    // remove `mut` keywords from outer inputs
    let inputs_no_mut = inputs
        .iter()
        .cloned()
        .map(|mut input| {
            if let FnArg::Typed(PatType { ref mut pat, .. }) = &mut input {
                if let Pat::Ident(ref mut ident) = pat.as_mut() {
                    ident.mutability = None;
                }
            }
            input
        })
        .collect::<Punctuated<_, Comma>>();

    let to_async = if asyncness.is_some() {
        quote! {
            async move { ::cabin::view::boundary::Boundary::from(#inner_ident(#args_idents).await) }
        }
    } else {
        quote! {
            ::std::future::ready(::cabin::view::boundary::Boundary::from(#inner_ident(#args_idents)))
        }
    };
    let async_await = if asyncness.is_some() {
        quote! { .await }
    } else {
        quote! {}
    };

    Ok(quote! {
        #(#attrs)*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs_no_mut #variadic) #output {
            #vis #constness #asyncness #unsafety #abi fn #inner_ident #generics(#inputs #variadic) #output {
                #block
            }

            const ID: &str = concat!(module_path!(), "::", #name);
            const BOUNDARY: ::cabin::view::boundary::BoundaryRef<(#args_types)> = ::cabin::view::boundary::BoundaryRef::new(
                ID,
                &(move |(#args_idents)| Box::pin(#to_async)),
            );

            #[::cabin::private::linkme::distributed_slice(::cabin::view::boundary::BOUNDARIES)]
            #[linkme(crate = ::cabin::private::linkme)]
            fn __register(r: &mut ::cabin::view::boundary::BoundaryRegistry) {
                r.register(&BOUNDARY)
            }

            ::cabin::view::boundary::internal::Boundary::with_id(
                #inner_ident(#args_idents) #async_await,
                ID
            )
        }
    })
}
