use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{token, Attribute, Error, Expr, ExprLit, FnArg, ItemTrait, Lit, TraitItem, TraitItemFn};

use crate::OptionExpr;

pub(crate) fn element_attribute(
    args: Punctuated<OptionExpr, Comma>,
    input: ItemTrait,
) -> syn::Result<TokenStream> {
    let ItemTrait {
        attrs,
        vis,
        unsafety: _,
        auto_token: _,
        restriction: _,
        trait_token: _,
        ident,
        generics,
        colon_token: _,
        supertraits: _,
        brace_token: _,
        items,
    } = input;

    if !generics.params.is_empty() {
        return Err(Error::new(ident.span(), "Element can not have generics"));
    }

    let opts = extract_options(args)?;
    let mut attr_factories = Vec::with_capacity(items.len());
    for item in items {
        let TraitItem::Fn(TraitItemFn {
            attrs,
            mut sig,
            default: _,
            semi_token: _,
        }) = item
        else {
            continue;
        };

        let opts = extract_field_options(&attrs)?;
        if opts.skip {
            continue;
        }

        let method_name = &sig.ident;
        let arg_names = sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Receiver(_) => None,
                FnArg::Typed(ty) => Some(ty.pat.clone()),
            })
            .collect::<Vec<_>>();

        sig.inputs = Punctuated::from_iter(
            sig.inputs
                .into_iter()
                .filter(|input| matches!(input, FnArg::Typed(_))),
        );

        // Forward only certain args
        let attrs = attrs
            .iter()
            .filter(|a| a.path().is_ident("doc") || a.path().is_ident("cfg"))
            .collect::<Vec<_>>();

        attr_factories.push(quote! {
            #(#attrs)*
            #vis #sig {
                #ident::#method_name((), #(#arg_names,)*)
            }
        });
    }

    let tag_name = opts
        .tag_name
        .unwrap_or_else(|| ident.to_string().to_lowercase());
    let fn_ident = format_ident!("{tag_name}");

    let factory = if opts.is_void {
        quote! {
            #(#attrs)*
            pub fn #fn_ident<A: #ident>(attrs: A) -> ::cabin::html::Html<A, ()> {
                ::cabin::html::Html::new(#tag_name, attrs, ()).into_void_element()
            }
        }
    } else {
        quote! {
            #[cfg(debug_assertions)]
            #(#attrs)*
            pub fn #fn_ident<A: #ident, V: ::cabin::View>(attrs: A, content: V)
                -> ::cabin::html::Html<A, ::cabin::view::BoxedView>
            {
                ::cabin::html::Html::new(#tag_name, attrs, content.boxed())
            }

            #[cfg(not(debug_assertions))]
            #(#attrs)*
            pub fn #fn_ident<A: #ident, V: ::cabin::View>(attrs: A, content: V)
                -> ::cabin::html::Html<A, V>
            {
                ::cabin::html::Html::new(#tag_name, attrs, content)
            }

        }
    };

    Ok(quote! {
        #factory

        #(#attr_factories)*

        impl Anchor for () {}

        impl<L, R> #ident for Pair<L, R>
        where
            L: ::cabin::html::attributes::Attributes,
            R: ::cabin::html::attributes::Attributes,
        {
        }
    })
}

#[derive(Debug, Default)]
struct Opts {
    tag_name: Option<String>,
    is_void: bool,
}

fn extract_options(attrs: Punctuated<OptionExpr, Comma>) -> syn::Result<Opts> {
    let mut opts = Opts::default();

    for opt in attrs {
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
struct FieldOpts {
    skip: bool,
}

fn extract_field_options(attrs: &[Attribute]) -> syn::Result<FieldOpts> {
    let mut opts = FieldOpts::default();

    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("element")) else {
        return Ok(opts);
    };

    for opt in attr.parse_args_with(Punctuated::<OptionExpr, token::Comma>::parse_terminated)? {
        if opt.value.is_none() && opt.key == format_ident!("skip") {
            opts.skip = true;
            continue;
        }

        return Err(Error::new(opt.key.span(), "unknown element option"));
    }

    Ok(opts)
}