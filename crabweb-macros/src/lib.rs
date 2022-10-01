use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let ident = input.ident;
    let snake_name = ident
        .to_string()
        .from_case(Case::Pascal)
        .to_case(Case::Snake);
    let factory_ident = format_ident!("__register_{}", snake_name);

    quote! {
        impl #generics ::crabweb::component::Component for #ident #generics {
            fn id() -> ::std::borrow::Cow<'static, str> {
                format!("{}::{}", module_path!().replace("r#", ""), #snake_name).into()
            }
        }

        #[::linkme::distributed_slice(::crabweb::component::registry::COMPONENT_FACTORIES)]
        fn #factory_ident(r: &mut ::crabweb::component::registry::ComponentRegistry) {
            r.register::<#ident>();
        }
    }
    .into()
}
