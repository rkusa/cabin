use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error};

pub fn derive_event(input: DeriveInput) -> syn::Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new(
            input.ident.span(),
            "Events cannot have generics",
        ));
    }

    // Events always come from the client, so derive `Exposed` too, unless opted out.
    let exposed = if is_manual_exposed(&input.attrs)? {
        None
    } else {
        Some(crate::derive_exposed::derive_exposed(input.clone())?)
    };

    let DeriveInput { ident, .. } = input;

    let name = ident.to_string();
    Ok(quote! {
        #exposed

        #[automatically_derived]
        impl ::cabin::event::Event for #ident {
            const ID: &'static str =  concat!(module_path!(), "::", #name);
        }

        #[cfg(not(target_arch = "wasm32"))]
        const _: () = {
            #[::cabin::private::linkme::distributed_slice(::cabin::private::EVENTS)]
            #[linkme(crate = ::cabin::private::linkme)]
            static VALIDATOR: ::cabin::private::EventValidator = ::cabin::private::EventValidator {
                id: <#ident as ::cabin::event::Event>::ID,
                validate: {
                    fn validate(
                        payload: ::cabin::private::EventPayload<'_>,
                    ) -> ::std::option::Option<::cabin::private::ValidateFuture> {
                        #[allow(unused_imports)]
                        use ::cabin::private::{ValidateDeserializable as _, ValidateOther as _};
                        (&&::cabin::private::Probe::<#ident>::new()).validate(payload)
                    }
                    validate
                },
            };
        };
    })
}

fn is_manual_exposed(attrs: &[Attribute]) -> syn::Result<bool> {
    let mut manual = false;
    for attr in attrs {
        if !attr.path().is_ident("event") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("manual_exposed") {
                manual = true;
                Ok(())
            } else {
                Err(meta.error("unsupported event attribute, expected `manual_exposed`"))
            }
        })?;
    }
    Ok(manual)
}
