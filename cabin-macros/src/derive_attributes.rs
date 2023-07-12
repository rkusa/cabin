use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    token, Attribute, Data, DataStruct, DeriveInput, Error, Expr, ExprLit, Fields, Ident, Lit, Type,
};

pub fn derive_attributes(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    if !generics.params.is_empty() {
        return Err(Error::new(ident.span(), "Attributes can not have generics"));
    }

    let Data::Struct(DataStruct {
        fields: Fields::Unnamed(fields),
        ..
    }) = data
    else {
        return Err(Error::new(
            ident.span(),
            "Attributes can only be derived from an unnamed struct",
        ));
    };

    if fields.unnamed.len() != 1 {
        return Err(Error::new(
            ident.span(),
            "Attributes can only be derived from an unnamed struct with exactly one field",
        ));
    }

    let opts = extract_options(&attrs)?;

    let attr_name = opts
        .name
        .unwrap_or_else(|| ident.to_string().to_lowercase());

    let field = fields.unnamed.first().unwrap();
    let (_, kind) = extract_inner_type(&field.ty)?;

    match kind {
        Kind::Bool => Ok(quote! {
            #[automatically_derived]
            impl ::cabin::html::attributes::Attributes2 for #ident {
                fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error> {
                    if self.0 {
                        r.attribute(#attr_name, self.0).map_err(::cabin::error::InternalError::from)?;
                    }
                    Ok(())
                }
            }
        }),
        Kind::Other => Ok(quote! {
            #[automatically_derived]
            impl ::cabin::html::attributes::Attributes2 for #ident {
                fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error> {
                    r.attribute(#attr_name, self.0).map_err(::cabin::error::InternalError::from)?;
                    Ok(())
                }
            }
        }),
    }
}

pub(crate) enum Kind {
    Bool,
    Other,
}

pub(crate) fn extract_inner_type(ty: &Type) -> syn::Result<(&Type, Kind)> {
    if let Type::Path(p) = ty {
        if p.path.is_ident("bool") {
            return Ok((ty, Kind::Bool));
        }
    }

    Ok((ty, Kind::Other))
}

#[derive(Debug, Default)]
struct Opts {
    name: Option<String>,
}

fn extract_options(attrs: &[Attribute]) -> syn::Result<Opts> {
    let mut opts = Opts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("attributes")) else {
        return Ok(opts);
    };

    for opt in attr.parse_args_with(Punctuated::<OptionExpr, token::Comma>::parse_terminated)? {
        if let Some(value) = opt.value {
            if opt.key == format_ident!("name") {
                let Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) = value
                else {
                    return Err(Error::new(value.span(), "name must be a str"));
                };

                opts.name = Some(s.value());
                continue;
            }
        }
        return Err(Error::new(opt.key.span(), "unknown element option"));
    }

    Ok(opts)
}

#[derive(Debug, Hash)]
struct OptionExpr {
    key: Ident,
    value: Option<Expr>,
}

impl Parse for OptionExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = Ident::parse(input)?;
        let value = if Option::<token::Eq>::parse(input)?.is_some() {
            Some(Expr::parse(input)?)
        } else {
            None
        };
        Ok(OptionExpr { key, value })
    }
}
