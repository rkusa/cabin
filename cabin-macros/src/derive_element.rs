use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_quote, token, Attribute, Data, DataStruct, DeriveInput, Error, Expr, ExprLit, ExprPath,
    Fields, Ident, Lit, Path, PathArguments, Type,
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
        return Err(Error::new(
            ident.span(),
            "Element can only be derived from a named struct",
        ));
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
        let (ty, kind) = extract_inner_type(&f.ty)?;

        // Forward only certain args
        let attrs = f
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("doc") || a.path().is_ident("cfg"))
            .collect::<Vec<_>>();

        let opts = extract_field_options("element", &f.attrs)?;
        let method_name = opts
            .method_name
            .map(|name| format_ident!("{name}"))
            .unwrap_or_else(|| ident.clone());

        match kind {
            Kind::Event => {
                let Some(event) = opts.event else {
                    return Err(Error::new(
                        ident.span(),
                        "event attribute requires event type via #[element(event = ...)]",
                    ));
                };

                if !opts.skip {
                    let what = format!("{method_name} event");
                    builder_methods.push(quote! {
                        #(#attrs)*
                        pub fn #method_name<E>(mut self, event: impl FnOnce(#event) -> E) -> Self
                        where
                            E: ::serde::Serialize + 'static,
                        {
                            let event = event(#event::default());
                            self.base.#ident = Some(Box::new(move || {
                                use std::hash::{Hash, Hasher};
                                let mut hasher = ::twox_hash::XxHash32::default();
                                ::std::any::TypeId::of::<E>().hash(&mut hasher);
                                let hash = hasher.finish() as u32;
                                ::serde_json::to_string(&event)
                                    .map_err(|err| ::cabin::error::InternalError::Serialize {
                                        what: #what,
                                        err,
                                    })
                                    .map(|json| (hash, json))
                            }));

                            self
                        }
                    });
                }

                let attr_name = opts.attribute_name.unwrap_or_else(|| {
                    let name = ident.to_string();
                    name.strip_prefix("on_")
                        .map(|s| s.to_string())
                        .unwrap_or(name)
                });
                let attr_name_id = format!("cabin-{attr_name}");
                let attr_name_payload = format!("cabin-{attr_name}-payload");
                render_statements.push(quote! {
                    if let Some(event) = self.#ident {
                        // TODO: directly write into r?
                        let (id, payload) = &(event)()?;
                        r.attribute(#attr_name_id, id)
                            .map_err(crate::error::InternalError::from)?;
                        r.attribute(#attr_name_payload, payload)
                            .map_err(crate::error::InternalError::from)?;
                    }
                });
            }
            Kind::Option => {
                if !opts.skip {
                    builder_methods.push(quote! {
                        #(#attrs)*
                        pub fn #method_name(mut self, #ident: impl Into<#ty>) -> Self {
                            self.base.#ident = Some(#ident.into());
                            self
                        }
                    });
                }

                let attr_name = opts.attribute_name.unwrap_or_else(|| ident.to_string());
                render_statements.push(quote! {
                    if let Some(#ident) = &self.#ident {
                        r.attribute(#attr_name, #ident).map_err(::cabin::error::InternalError::from)?;
                    }
                });
            }
            Kind::Bool => {
                if !opts.skip {
                    builder_methods.push(quote! {
                        #(#attrs)*
                        pub fn #method_name(mut self, #ident: #ty) -> Self {
                            self.base.#ident = #ident;
                            self
                        }
                    });
                }

                let attr_name = opts.attribute_name.unwrap_or_else(|| ident.to_string());
                render_statements.push(quote! {
                    if self.#ident {
                        r.empty_attribute(#attr_name).map_err(::cabin::error::InternalError::from)?;
                    }
                });
            }
            Kind::Other => {
                if !opts.skip {
                    builder_methods.push(quote! {
                        #(#attrs)*
                        pub fn #method_name(mut self, #ident: impl Into<#ty>) -> Self {
                            self.base.#ident = #ident.into();
                            self
                        }
                    });
                }

                let attr_name = opts.attribute_name.unwrap_or_else(|| ident.to_string());
                render_statements.push(quote! {
                    if self.#ident != Default::default() {
                        r.attribute(#attr_name, self.#ident).map_err(::cabin::error::InternalError::from)?;
                    }
                });
            }
        }
    }

    let alias_ident = format_ident!("{}Element", ident);
    let tag_name = opts
        .tag_name
        .unwrap_or_else(|| ident.to_string().to_lowercase());
    let fn_ident = format_ident!("{tag_name}");
    let is_void = opts.is_void;

    let factory = if opts.is_void {
        quote! {
            #(#attrs)*
            pub fn #fn_ident(
                attributes: impl Into<::cabin::html::attributes::Attributes<#ident, ()>>,
            ) -> ::cabin::html::Html<(), #ident, ()> {
                ::cabin::html::Html::new(attributes, ())
            }
        }
    } else {
        quote! {
            #[cfg(debug_assertions)]
            #(#attrs)*
            pub fn #fn_ident<V: ::cabin::View>(
                attributes: impl Into<::cabin::html::attributes::Attributes<#ident, ()>>,
                content: V
            ) -> ::cabin::html::Html<::cabin::view::BoxedView, #ident, ()> {
                ::cabin::html::Html::new(attributes, content.boxed())
            }

            #[cfg(not(debug_assertions))]
            #(#attrs)*
            pub fn #fn_ident<V: ::cabin::View>(
                attributes: impl Into<::cabin::html::attributes::Attributes<#ident, ()>>,
                content: V
            ) -> ::cabin::html::Html<V, #ident, ()> {
                ::cabin::html::Html::new(attributes, content)
            }
        }
    };

    Ok(quote! {
        #(#attrs)*
        pub type #alias_ident<Ext> = ::cabin::html::attributes::Attributes<#ident, Ext>;

        #factory

        pub mod #fn_ident {
            #[inline]
            pub fn default<Ext: Default>() -> ::cabin::html::attributes::Attributes<super::#ident, Ext> {
                ::cabin::html::attributes::Attributes::default()
            }
        }

        #[automatically_derived]
        impl<Ext> #alias_ident<Ext> {
            #(#builder_methods)*
        }

        #[automatically_derived]
        impl ::cabin::html::elements::ElementExt for #ident {
            fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error>
            {
                #(#render_statements)*

                Ok(())
            }
        }

        #[automatically_derived]
        impl ::cabin::html::elements::Element for #ident {
            const TAG: &'static str = #tag_name;

            fn is_void_element() -> bool {
                #is_void
            }
        }
    })
}

pub(crate) enum Kind {
    Event,
    Option,
    Bool,
    Other,
}

pub(crate) fn extract_inner_type(ty: &Type) -> syn::Result<(&Type, Kind)> {
    if let Type::Path(p) = ty {
        if p.clone() == parse_quote!(Option<Box<SerializeEventFn>>) {
            return Ok((ty, Kind::Event));
        }

        if p.path.segments.len() != 1 {
            return Ok((ty, Kind::Other));
        }

        let segment = &p.path.segments[0];
        if segment.ident == "bool" {
            return Ok((ty, Kind::Bool));
        }

        if segment.ident != "Option" {
            return Ok((ty, Kind::Other));
        }

        if let PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(syn::GenericArgument::Type(t)) = args.args.first() {
                return Ok((t, Kind::Option));
            }
        }
    }

    Ok((ty, Kind::Other))
}

#[derive(Debug, Default)]
struct Opts {
    tag_name: Option<String>,
    is_void: bool,
}

fn extract_options(attrs: &[Attribute]) -> syn::Result<Opts> {
    let mut opts = Opts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("element")) else {
        return Ok(opts);
    };

    for opt in attr.parse_args_with(Punctuated::<OptionExpr, token::Comma>::parse_terminated)? {
        if let Some(value) = opt.value {
            if opt.key == format_ident!("tag_name") {
                let Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) = value
                else {
                    return Err(Error::new(value.span(), "tag_name must be a str"));
                };

                opts.tag_name = Some(s.value());
            } else {
                return Err(Error::new(opt.key.span(), "unknown element option"));
            }
        } else if opt.key == format_ident!("void") {
            opts.is_void = true;
        } else {
            return Err(Error::new(opt.key.span(), "unknown element option"));
        }
    }

    Ok(opts)
}

#[derive(Debug, Default)]
pub(crate) struct FieldOpts {
    pub(crate) method_name: Option<String>,
    pub(crate) attribute_name: Option<String>,
    pub(crate) skip: bool,
    pub(crate) event: Option<Path>,
}

pub(crate) fn extract_field_options(tag: &str, attrs: &[Attribute]) -> syn::Result<FieldOpts> {
    let mut opts = FieldOpts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident(tag)) else {
        return Ok(opts);
    };

    for opt in attr.parse_args_with(Punctuated::<OptionExpr, token::Comma>::parse_terminated)? {
        if let Some(value) = opt.value {
            if opt.key == format_ident!("method_name") {
                let Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) = value
                else {
                    return Err(Error::new(value.span(), "method_name must be a str"));
                };

                opts.method_name = Some(s.value());
            } else if opt.key == format_ident!("attribute_name") {
                let Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) = value
                else {
                    return Err(Error::new(value.span(), "attribute_name must be a str"));
                };

                opts.attribute_name = Some(s.value());
            } else if opt.key == format_ident!("event") {
                let Expr::Path(ExprPath { path, .. }) = value else {
                    return Err(Error::new(value.span(), "event must be a path"));
                };

                opts.event = Some(path);
            } else {
                return Err(Error::new(opt.key.span(), "unknown element option"));
            }
        } else if opt.key == format_ident!("skip") {
            opts.skip = true;
        } else {
            return Err(Error::new(opt.key.span(), "unknown element option"));
        }
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
