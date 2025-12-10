use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Expr, ExprLit, Fields, Ident, Lit,
    Type, token,
};

pub fn derive_attribute(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data,
    } = input;

    if !generics.params.is_empty() {
        return Err(Error::new(ident.span(), "Attribute cannot have generics"));
    }

    let opts = extract_options(&attrs)?;
    let attr_name = opts
        .name
        .unwrap_or_else(|| ident.to_string().to_lowercase());

    if !valid_attribute_name(&attr_name) {
        return Err(Error::new(
            ident.span(),
            format!("Invalid html attribute name `{attr_name}`"),
        ));
    }

    if opts.outer || matches!(data, Data::Enum(DataEnum { .. })) {
        return Ok(quote! {
            #[automatically_derived]
            impl ::cabin::html::attributes::Attributes for #ident {
                fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error> {
                    r.attribute(#attr_name, self);
                    Ok(())
                }
            }
        });
    }

    let Data::Struct(DataStruct {
        fields: Fields::Unnamed(fields),
        ..
    }) = data
    else {
        return Err(Error::new(
            ident.span(),
            "Attribute can only be derived from an unnamed struct",
        ));
    };

    if fields.unnamed.len() != 1 {
        return Err(Error::new(
            ident.span(),
            "Attribute can only be derived from an unnamed struct with exactly one field",
        ));
    }

    let field = fields.unnamed.first().unwrap();
    let (_, kind) = extract_inner_type(&field.ty)?;

    match kind {
        Kind::Bool => Ok(quote! {
            #[automatically_derived]
            impl ::cabin::html::attributes::Attributes for #ident {
                fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error> {
                    if self.0 {
                        r.empty_attribute(#attr_name);
                    }
                    Ok(())
                }
            }
        }),
        Kind::Other => Ok(quote! {
            #[automatically_derived]
            impl ::cabin::html::attributes::Attributes for #ident {
                fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error> {
                    r.attribute(#attr_name, self.0);
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
    if let Type::Path(p) = ty
        && p.path.is_ident("bool")
    {
        return Ok((ty, Kind::Bool));
    }

    Ok((ty, Kind::Other))
}

#[derive(Debug, Default)]
struct Opts {
    name: Option<String>,
    outer: bool,
}

fn extract_options(attrs: &[Attribute]) -> syn::Result<Opts> {
    let mut opts = Opts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("attribute")) else {
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
        } else if opt.key == format_ident!("outer") {
            opts.outer = true;
            continue;
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

fn valid_attribute_name(name: &str) -> bool {
    // https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
    !name.chars().any(|ch| {
        matches!(ch,
            ' ' | '"' | '\'' | '>' | '/' | '=' |
            /* controls */
            '\u{7F}'..='\u{9F}' |
            /* non character */
            '\u{FDD0}'..='\u{FDEF}' |  '\u{FFFE}' | '\u{FFFF}' | '\u{1FFFE}' | '\u{1FFFF}' |
            '\u{2FFFE}' | '\u{2FFFF}' | '\u{3FFFE}' | '\u{3FFFF}' | '\u{4FFFE}' | '\u{4FFFF}' |
            '\u{5FFFE}' | '\u{5FFFF}' | '\u{6FFFE}' | '\u{6FFFF}' | '\u{7FFFE}' | '\u{7FFFF}' |
            '\u{8FFFE}' | '\u{8FFFF}' | '\u{9FFFE}' | '\u{9FFFF}' | '\u{AFFFE}' | '\u{AFFFF}' |
            '\u{BFFFE}' | '\u{BFFFF}' | '\u{CFFFE}' | '\u{CFFFF}' | '\u{DFFFE}' | '\u{DFFFF}' |
            '\u{EFFFE}' | '\u{EFFFF}' | '\u{FFFFE}' | '\u{FFFFF}' | '\u{10FFFE}' |  '\u{10FFFF}'
        )
    })
}
