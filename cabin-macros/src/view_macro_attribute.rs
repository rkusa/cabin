use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, FnArg, ItemFn, Path, Signature};

pub struct ModulePathAttribute {
    pub path: Path,
}

impl Parse for ModulePathAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let Ok(path) = input.parse::<Path>() else {
            return Err(input
                .error("this macro expects a module path (e.g., #[view_macro(crate::foo::bar)])"));
        };
        if !input.is_empty() {
            return Err(input.error(
                "this macro accepts only a single module path as its \
                attribute argument (e.g., #[view_macro(crate::foo::bar)])",
            ));
        }

        Ok(ModulePathAttribute { path })
    }
}

pub fn view_macro_attribute(attr: ModulePathAttribute, item: ItemFn) -> syn::Result<TokenStream> {
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

    let Some(input) = inputs.first() else {
        return Err(Error::new(inputs.span(), "function must have one argument"));
    };
    if let FnArg::Receiver(_) = input {
        return Err(Error::new(input.span(), "cannot have self argument"));
    }

    let module = if let Some(p) = attr.path.segments.first()
        && p.ident == "crate"
    {
        let path = attr.path;
        quote!($#path)
    } else {
        let path = attr.path;
        quote!(#path)
    };
    let module_ident = format_ident!("__view_macro_{}", ident);

    Ok(quote! {
        #(#attrs)*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs #variadic) #output {
            #block
        }

        mod #module_ident {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! #ident {
                ($($x:tt)*) => {
                    #module::#ident(cabin::view![$($x)*])
                }
            }

            pub use #ident;
        }

        #[doc(inline)]
        pub use #module_ident::#ident;
    })
}
