use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataStruct, DeriveInput, Error, Fields};

use crate::derive_element::{extract_field_options, extract_inner_type, Kind};

pub fn derive_attributes(input: DeriveInput) -> syn::Result<TokenStream> {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident: struct_ident,
        generics,
        data,
    } = input;

    let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = data
    else {
        return Err(Error::new(struct_ident.span(), "Attributes can only be derived from a named struct"));
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut builder_methods = Vec::with_capacity(fields.named.len());
    let mut render_statements = Vec::with_capacity(fields.named.len());
    let parent_ident = format_ident!("{}", struct_ident.to_string().to_lowercase());

    for f in fields.named.iter() {
        let ident = f.ident.as_ref().unwrap();
        let (ty, kind) = extract_inner_type(&f.ty)?;

        // Forward only certain args
        let attrs = f
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("doc") || a.path().is_ident("cfg"))
            .collect::<Vec<_>>();

        let opts = extract_field_options("attributes", &f.attrs)?;
        let method_name = opts
            .method_name
            .map(|name| format_ident!("{name}"))
            .unwrap_or_else(|| ident.clone());

        match kind {
            Kind::Option => {
                if !opts.skip {
                    builder_methods.push(quote! {
                        #(#attrs)*
                        pub fn #method_name(mut self, #ident: impl Into<#ty>) -> Self {
                            // TODO: use get_or_insert_default() once stabel
                            self.#parent_ident = match self.#parent_ident.take() {
                                Some(mut o) => {
                                    o.#ident = Some(#ident.into());
                                    Some(o)
                                },
                                None => {
                                    let mut o = Box::<#struct_ident>::default();
                                    o.#ident = Some(#ident.into());
                                    Some(o)
                                }
                            };
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
                            // TODO: use get_or_insert_default() once stabel
                            self.#parent_ident = match self.#parent_ident.take() {
                                Some(mut o) => {
                                    o.#ident = #ident;
                                    Some(o)
                                },
                                None => {
                                    let mut o = Box::<#struct_ident>::default();
                                    o.#ident = #ident;
                                    Some(o)
                                }
                            };
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
                            // TODO: use get_or_insert_default() once stabel
                            self.#parent_ident = match self.#parent_ident.take() {
                                Some(mut o) => {
                                    o.#ident = #ident.into();
                                    Some(o)
                                },
                                None => {
                                    let mut o = Box::<#struct_ident>::default();
                                    o.#ident = #ident.into();
                                    Some(o)
                                }
                            };
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

    Ok(quote! {
        #[automatically_derived]
        impl<V, K> ::cabin::html::Html<V, K> {
            #(#builder_methods)*
        }

        #[automatically_derived]
        impl #impl_generics ::cabin::html::ElementExt for #struct_ident #ty_generics #where_clause {
            fn render(self, r: &mut ::cabin::render::ElementRenderer) -> Result<(), ::cabin::Error>
            {
                #(#render_statements)*

                Ok(())
            }
        }
    })
}
