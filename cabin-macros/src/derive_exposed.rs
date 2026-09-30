use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DataUnion, DeriveInput, Error, Fields, parse_quote,
};

pub fn derive_exposed(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        mut generics,
        data,
    } = input;

    let mut boxed = is_boxed(&attrs)?;
    let arms = match data {
        Data::Struct(DataStruct { fields, .. }) => {
            vec![arm(quote! { Self }, &fields, &ident, &mut boxed)?]
        }
        Data::Enum(DataEnum { variants, .. }) => variants
            .iter()
            .map(|variant| {
                let variant_ident = &variant.ident;
                arm(
                    quote! { Self::#variant_ident },
                    &variant.fields,
                    &ident,
                    &mut boxed,
                )
            })
            .collect::<syn::Result<_>>()?,
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(Error::new(
                union_token.span,
                "Exposed cannot be derived for unions",
            ));
        }
    };

    for param in generics.type_params_mut() {
        param.bounds.push(parse_quote!(::cabin::Exposed));
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let body = if arms.is_empty() {
        // enum without variants
        quote! { match *self {} }
    } else {
        quote! {
            match self {
                #(#arms)*
            }
            Ok(())
        }
    };

    if !boxed {
        return Ok(quote! {
            #[automatically_derived]
            impl #impl_generics ::cabin::Exposed for #ident #ty_generics #where_clause {
                async fn validate(&self) -> ::std::result::Result<(), ::cabin::Error> {
                    #body
                }
            }
        });
    }

    // For recursive types (e.g. `struct Node { children: Vec<Node> }`), the future is boxed so
    // that its type is known without inferring it from the body. Otherwise, they'd fail to
    // compile, because inferring the future type (and whether it is `Send`) would require itself.
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::cabin::Exposed for #ident #ty_generics #where_clause {
            #[allow(refining_impl_trait)]
            fn validate(
                &self,
            ) -> ::std::pin::Pin<::std::boxed::Box<
                dyn ::std::future::Future<Output = ::std::result::Result<(), ::cabin::Error>>
                    + ::std::marker::Send
                    + '_,
            >> {
                ::std::boxed::Box::pin(async move {
                    #body
                })
            }
        }
    })
}

/// Creates a match arm that destructures `fields` and validates each non-skipped one. Sets `boxed`
/// if a validated field's type refers to the type itself.
fn arm(
    path: TokenStream,
    fields: &Fields,
    ident: &Ident,
    boxed: &mut bool,
) -> syn::Result<TokenStream> {
    let mut bindings = Vec::new();
    let mut validations = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let binding = format_ident!("__field{i}");
        let skip = is_skipped(field)?;
        if !skip && refers_to(field.ty.to_token_stream(), ident) {
            *boxed = true;
        }
        let pat = if skip {
            quote! { _ }
        } else {
            validations.push(quote! { ::cabin::Exposed::validate(#binding).await?; });
            quote! { #binding }
        };
        bindings.push(match &field.ident {
            Some(ident) => quote! { #ident: #pat },
            None => pat,
        });
    }

    Ok(match fields {
        Fields::Named(_) => quote! { #path { #(#bindings),* } => { #(#validations)* } },
        Fields::Unnamed(_) => quote! { #path(#(#bindings),*) => { #(#validations)* } },
        Fields::Unit => quote! { #path => {} },
    })
}

/// Whether `tokens` contain `ident` or `Self`. Only catches direct recursion; types that are
/// recursive through another type need `#[exposed(boxed)]`.
fn refers_to(tokens: TokenStream, ident: &Ident) -> bool {
    tokens.into_iter().any(|tt| match tt {
        TokenTree::Ident(i) => i == *ident || i == "Self",
        TokenTree::Group(g) => refers_to(g.stream(), ident),
        _ => false,
    })
}

fn is_boxed(attrs: &[Attribute]) -> syn::Result<bool> {
    let mut boxed = false;
    for attr in attrs {
        if !attr.path().is_ident("exposed") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("boxed") {
                boxed = true;
                Ok(())
            } else {
                Err(meta.error("unsupported exposed attribute, expected `boxed`"))
            }
        })?;
    }
    Ok(boxed)
}

fn is_skipped(field: &syn::Field) -> syn::Result<bool> {
    let mut skip = false;
    for attr in &field.attrs {
        if !attr.path().is_ident("exposed") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                skip = true;
                Ok(())
            } else {
                Err(meta.error("unsupported exposed attribute, expected `skip`"))
            }
        })?;
    }
    Ok(skip)
}
