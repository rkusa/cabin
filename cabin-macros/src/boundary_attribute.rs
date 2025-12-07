use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, FnArg, ItemFn, Pat, PatType, Signature, Type};

pub fn boundary_attribute(
    item: ItemFn,
    events: Punctuated<Type, Comma>,
    wasm_enabled: bool,
) -> syn::Result<TokenStream> {
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
        // first argument must be &Context
        .skip(1)
        .map(|input| match input {
            FnArg::Receiver(_) => Err(Error::new(
                input.span(),
                "boundary cannot have self argument",
            )),
            FnArg::Typed(PatType { pat, ty, .. }) => match pat.as_ref() {
                Pat::Ident(ident) => Ok((ident.ident.clone(), ty.clone())),
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
            if let FnArg::Typed(PatType { pat, .. }) = &mut input
                && let Pat::Ident(ident) = pat.as_mut()
            {
                ident.mutability = None;
            }
            input
        })
        .collect::<Punctuated<_, Comma>>();

    let to_async = if asyncness.is_some() {
        quote! {
            async move {
                c.any(::cabin::view::boundary::internal::Boundary::upgrade(
                    #inner_ident(c, #args_idents).await,
                    &BOUNDARY
                ))
            }
        }
    } else {
        quote! {
            ::std::future::ready(c.any(::cabin::view::boundary::internal::Boundary::upgrade(
                #inner_ident(c, #args_idents),
                &BOUNDARY
            )))
        }
    };
    let async_await = if asyncness.is_some() {
        quote! { .await }
    } else {
        quote! {}
    };
    let events = events.into_iter().collect::<Vec<Type>>();

    let wasm = wasm_enabled.then(|| {
        quote! {
            #[cfg(target_arch = "wasm32")]
            #[unsafe(export_name = concat!(module_path!(), "::", #name))]
            unsafe extern "C" fn __wasm(event: *const u8, event_len: usize, out: *mut *const u8) -> usize {
                BOUNDARY.wasm(event, event_len, out)
            }
        }
    });

    Ok(quote! {
        #(#attrs)*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs_no_mut #variadic) #output {
            #vis #constness #asyncness #unsafety #abi fn #inner_ident #generics(#inputs #variadic) #output {
                #block
            }

            const ID: &str = concat!(module_path!(), "::", #name);
            static EVENTS: &'static [&'static str] = &[#(::cabin::event::event_id::<#events>(),)*];
            static BOUNDARY: ::cabin::view::boundary::BoundaryRef<(#args_types)> =
                ::cabin::view::boundary::BoundaryRef::new(
                    ID,
                    &EVENTS,
                    &(move |c, (#args_idents)| Box::pin(#to_async)),
                );

            #[cfg(not(target_arch = "wasm32"))]
            #[::cabin::private::linkme::distributed_slice(crate::BOUNDARIES)]
            #[linkme(crate = ::cabin::private::linkme)]
            fn __register(r: &mut ::cabin::boundary_registry::BoundaryRegistry) {
                r.register(&BOUNDARY)
            }

            #wasm

            ::cabin::view::boundary::internal::Boundary::upgrade(
                #inner_ident(c, #args_idents) #async_await,
                &BOUNDARY
            )
        }
    })
}
