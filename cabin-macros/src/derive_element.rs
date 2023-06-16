use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    token, Attribute, Data, DataStruct, DeriveInput, Error, Fields, Ident, Lit, PathArguments, Type,
};

pub fn derive_element(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    if !generics.params.is_empty() {
        return Err(Error::new(ident.span(), "Element can not have generics"));
    }

    let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = data
    else {
        return Err(Error::new(ident.span(), "Element can only be derived from a named struct"));
    };

    let opts = extract_options(&attrs)?;

    // Do not forward `element` attributes
    let attrs = attrs
        .into_iter()
        .filter(|a| !a.path().is_ident("element"))
        .collect::<Vec<_>>();

    // let mut html_generics = generics.clone();
    // html_generics.params.push(parse_quote!(V));
    // html_generics.params.push(parse_quote!(A));
    // let (html_impl_generics, html_ty_generics, _) = html_generics.split_for_impl();
    // let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut builder_methods = Vec::with_capacity(fields.named.len());
    let mut render_statements = Vec::with_capacity(fields.named.len());

    for f in fields.named.iter() {
        let ident = f.ident.as_ref().unwrap();
        let ty = extract_inner_type(&f.ty)?;

        // Forward only certain args
        let attrs = f
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("doc") || a.path().is_ident("cfg"))
            .collect::<Vec<_>>();

        let opts = extract_field_options(&f.attrs)?;
        let method_name = opts
            .method_name
            .map(|name| format_ident!("{name}"))
            .unwrap_or_else(|| ident.clone());

        builder_methods.push(quote! {
            #(#attrs)*
            pub fn #method_name(mut self, #ident: impl Into<#ty>) -> Self {
                self.kind.#ident = Some(#ident.into());
                self
            }
        });

        let attr_name = opts.attribute_name.unwrap_or_else(|| ident.to_string());
        render_statements.push(quote! {
            if let Some(#ident) = &self.#ident {
                r.attribute(#attr_name, #ident).map_err(::cabin::error::InternalError::from)?;
            }
        });
    }

    let alias_ident = format_ident!("{}Element", ident);
    let tag_name = opts
        .tag_name
        .unwrap_or_else(|| ident.to_string().to_lowercase());
    let fn_ident = format_ident!("{tag_name}");

    Ok(quote! {
        #(#attrs)*
        pub type #alias_ident<V> = ::cabin::html::Html<V, #ident>;

        #(#attrs)*
        pub fn #fn_ident<V: ::cabin::View>(content: V) -> #alias_ident<V> {
            ::cabin::html::Html::new(#tag_name, content)
        }

        #[automatically_derived]
        impl<V> #alias_ident<V> {
            #(#builder_methods)*
        }

        #[automatically_derived]
        impl ::cabin::html::ElementExt for #ident {
            fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error>
            {
                #(#render_statements)*

                Ok(())
            }
        }
    })
}

fn extract_inner_type(ty: &Type) -> syn::Result<&Type> {
    if let Type::Path(p) = ty {
        if p.path.segments.len() != 1 {
            return Err(Error::new(
                ty.span(),
                "expected struct field to be an Option<T>",
            ));
        }

        let segment = &p.path.segments[0];
        if segment.ident != "Option" {
            return Err(Error::new(
                ty.span(),
                "expected struct field to be an Option<T>",
            ));
        }

        if let PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(syn::GenericArgument::Type(t)) = args.args.first() {
                return Ok(t);
            }
        }
    }

    Err(Error::new(
        ty.span(),
        "expected struct field to be an Option<T>",
    ))
}

#[derive(Debug, Default)]
struct Opts {
    tag_name: Option<String>,
}

fn extract_options(attrs: &[Attribute]) -> syn::Result<Opts> {
    let mut opts = Opts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("element"))
    else {
        return Ok(opts);
    };

    for opt in attr.parse_args_with(Punctuated::<OptionExpr, token::Comma>::parse_terminated)? {
        if opt.key == format_ident!("tag_name") {
            let Lit::Str(s) = opt.value
            else {
                return Err(Error::new(
                    opt.value.span(),
                    "tag_name must be a str"
                ));
            };

            opts.tag_name = Some(s.value());
        } else {
            return Err(Error::new(opt.key.span(), "unknown element option"));
        }
    }

    Ok(opts)
}

#[derive(Debug, Default)]
struct FieldOpts {
    method_name: Option<String>,
    attribute_name: Option<String>,
}

fn extract_field_options(attrs: &[Attribute]) -> syn::Result<FieldOpts> {
    let mut opts = FieldOpts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("element"))
    else {
        return Ok(opts);
    };

    for opt in attr.parse_args_with(Punctuated::<OptionExpr, token::Comma>::parse_terminated)? {
        if opt.key == format_ident!("method_name") {
            let Lit::Str(s) = opt.value
            else {
                return Err(Error::new(
                    opt.value.span(),
                    "method_name must be a str"
                ));
            };

            opts.method_name = Some(s.value());
        } else if opt.key == format_ident!("attribute_name") {
            let Lit::Str(s) = opt.value
            else {
                return Err(Error::new(
                    opt.value.span(),
                    "attribute_name must be a str"
                ));
            };

            opts.attribute_name = Some(s.value());
        } else {
            return Err(Error::new(opt.key.span(), "unknown element option"));
        }
    }

    Ok(opts)
}

#[derive(Debug, Hash)]
struct OptionExpr {
    key: Ident,
    eq_token: token::Eq,
    value: Lit,
}

impl Parse for OptionExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(OptionExpr {
            key: Ident::parse(input)?,
            eq_token: token::Eq::parse(input)?,
            value: Lit::parse(input)?,
        })
    }
}
