use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, ExprLit, FnArg, GenericArgument, ItemTrait, Lit, PathArguments, Signature, TraitItem,
    TraitItemFn, Type, TypeParamBound,
};

pub fn length_aliases_attribute(mut item: ItemTrait) -> syn::Result<TokenStream> {
    let mut items = Vec::new();

    for item in item.items.iter_mut() {
        let TraitItem::Fn(f) = item else {
            continue;
        };

        let TraitItemFn {
            attrs,
            sig,
            default: _,
            semi_token: _,
        } = f;

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

        if inputs.len() != 2 {
            continue;
        }
        let self_input = {
            let mut self_input = inputs[0].clone();
            let FnArg::Receiver(receiver) = &mut self_input else {
                continue;
            };
            receiver.mutability = None;
            self_input
        };

        let input = &inputs[1];

        let FnArg::Typed(ty) = input else {
            continue;
        };
        let input_pat = &ty.pat;

        let Type::ImplTrait(impl_trait) = ty.ty.as_ref() else {
            continue;
        };

        if impl_trait.bounds.len() != 1 {
            continue;
        }
        let bound = &impl_trait.bounds[0];

        let TypeParamBound::Trait(trait_bound) = &bound else {
            continue;
        };

        if trait_bound.path.segments.len() != 1 {
            continue;
        }
        let segment = &trait_bound.path.segments[0];
        if segment.ident != "Into" {
            continue;
        }

        let PathArguments::AngleBracketed(args) = &segment.arguments else {
            continue;
        };

        if args.args.len() != 1 {
            continue;
        }
        let arg = &args.args[0];

        let GenericArgument::Type(Type::Path(path)) = &arg else {
            continue;
        };

        if path.path.segments.len() != 1 {
            continue;
        }
        let segment = &path.path.segments[0];

        if segment.ident != "Length" {
            continue;
        }

        let mut with_horizontal_viewport_units = false;
        let mut with_vertical_viewport_units = false;
        let mut with_full = false;
        let mut with_screen_horizontal = false;
        let mut with_screen_vertical = false;
        let mut with_auto = false;
        let mut with_content = false;
        let mut without_zero = false;
        attrs.retain(|attr| match &attr.meta {
            syn::Meta::Path(path) => {
                let Some(ident) = path.get_ident() else {
                    return true;
                };
                if ident == "with_horizontal_viewport_units" {
                    with_horizontal_viewport_units = true;
                    false
                } else if ident == "with_vertical_viewport_units" {
                    with_vertical_viewport_units = true;
                    false
                } else if ident == "with_full" {
                    with_full = true;
                    false
                } else if ident == "with_screen_horizontal" {
                    with_screen_horizontal = true;
                    false
                } else if ident == "with_screen_vertical" {
                    with_screen_vertical = true;
                    false
                } else if ident == "with_auto" {
                    with_auto = true;
                    false
                } else if ident == "with_content" {
                    with_content = true;
                    false
                } else if ident == "without_zero" {
                    without_zero = true;
                    false
                } else {
                    true
                }
            }
            _ => true,
        });

        let doc_lines = attrs
            .iter()
            .filter_map(|attr| {
                // panic!("{:#?}", attr);
                let name_value = attr.meta.require_name_value().ok()?;
                if name_value.path.get_ident()? != "doc" {
                    return None;
                }
                let Expr::Lit(ExprLit {
                    lit: Lit::Str(str), ..
                }) = &name_value.value
                else {
                    return None;
                };
                Some(str.value())
            })
            .skip_while(|doc| doc != " ```css")
            .take_while(|doc| doc != " ```");
        let doc = doc_lines.collect::<Vec<_>>().join("\n") + "\n ```";

        if !without_zero {
            items.push({
                let alias_ident = format_ident!("{ident}_zero");
                let doc = doc.replace("{x}", "0");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Px(0.0))
                    }
                }
            });
        }
        if with_auto {
            items.push({
                let alias_ident = format_ident!("{ident}_auto");
                let doc = doc.replace("{x}", "auto");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Auto)
                    }
                }
            });
        }
        if with_full {
            items.push({
                let alias_ident = format_ident!("{ident}_full");
                let doc = doc.replace("{x}", "100%");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Percent(100.0))
                    }
                }
            });
        }
        if with_screen_vertical {
            items.push({
                let alias_ident = format_ident!("{ident}_screen");
                let doc = doc.replace("{x}", "100vh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Vh(100))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_screen_svh");
                let doc = doc.replace("{x}", "100svh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Svh(100))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_screen_lvh");
                let doc = doc.replace("{x}", "100lvh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Lvh(100))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_screen_dvh");
                let doc = doc.replace("{x}", "100dvh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Dvh(100))
                    }
                }
            });
        }
        if with_screen_horizontal {
            items.push({
                let alias_ident = format_ident!("{ident}_screen");
                let doc = doc.replace("{x}", "100vw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Vw(100))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_screen_svw");
                let doc = doc.replace("{x}", "100svw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Svw(100))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_screen_lvw");
                let doc = doc.replace("{x}", "100lvw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Lvw(100))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_screen_dvw");
                let doc = doc.replace("{x}", "100dvw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::Dvw(100))
                    }
                }
            });
        }
        if with_content {
            items.push({
                let alias_ident = format_ident!("{ident}_min");
                let doc = doc.replace("{x}", "min-content");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::MinContent)
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_max");
                let doc = doc.replace("{x}", "max-content");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::MaxContent)
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_fit");
                let doc = doc.replace("{x}", "fit-content");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input) #variadic #output {
                        self.#ident(Length::FitContent)
                    }
                }
            });
        }
        items.push({
            let alias_ident = format_ident!("{ident}_unit");
            let doc = doc.replace("{x}", "{x * 0.25}rem");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: i16) #variadic #output {
                    self.#ident(Length::Unit(f32::from(#input_pat)))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_unitf");
            let doc = doc.replace("{x}", "{x * 0.25}rem");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Unit(#input_pat))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_rem");
            let doc = doc.replace("{x}", "{x}rem");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: i16) #variadic #output {
                    self.#ident(Length::Rem(f32::from(#input_pat)))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_remf");
            let doc = doc.replace("{x}", "{x}rem");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Rem(#input_pat))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_em");
            let doc = doc.replace("{x}", "{x}em");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: i16) #variadic #output {
                    self.#ident(Length::Em(f32::from(#input_pat)))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_emf");
            let doc = doc.replace("{x}", "{x}em");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Em(#input_pat))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_px");
            let doc = doc.replace("{x}", "{x}px");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: i16) #variadic #output {
                    self.#ident(Length::Px(f32::from(#input_pat)))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_pxf");
            let doc = doc.replace("{x}", "{x}px");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Px(#input_pat))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_percent");
            let doc = doc.replace("{x}", "{x}%");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: i16) #variadic #output {
                    self.#ident(Length::Percent(f32::from(#input_pat)))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_percentf");
            let doc = doc.replace("{x}", "{x}%");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Percent(#input_pat))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_mm");
            let doc = doc.replace("{x}", "{x}mm");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Mm(#input_pat))
                }
            }
        });
        items.push({
            let alias_ident = format_ident!("{ident}_cm");
            let doc = doc.replace("{x}", "{x}cm");
            quote! {
                #[doc = #doc]
                #constness #asyncness #unsafety #abi
                fn #alias_ident #generics (#self_input, #input_pat: f32) #variadic #output {
                    self.#ident(Length::Cm(#input_pat))
                }
            }
        });
        if with_horizontal_viewport_units {
            items.push({
                let alias_ident = format_ident!("{ident}_vw");
                let doc = doc.replace("{x}", "{x}vw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Vw(#input_pat))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_svw");
                let doc = doc.replace("{x}", "{x}svw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Svw(#input_pat))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_lvw");
                let doc = doc.replace("{x}", "{x}lvw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Lvw(#input_pat))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_dvw");
                let doc = doc.replace("{x}", "{x}dvw");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Dvw(#input_pat))
                    }
                }
            });
        }
        if with_vertical_viewport_units {
            items.push({
                let alias_ident = format_ident!("{ident}_vh");
                let doc = doc.replace("{x}", "{x}vh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Vh(#input_pat))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_svh");
                let doc = doc.replace("{x}", "{x}svh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Svh(#input_pat))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_lvh");
                let doc = doc.replace("{x}", "{x}lvh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Lvh(#input_pat))
                    }
                }
            });
            items.push({
                let alias_ident = format_ident!("{ident}_dvh");
                let doc = doc.replace("{x}", "{x}dvh");
                quote! {
                    #[doc = #doc]
                    #constness #asyncness #unsafety #abi
                    fn #alias_ident #generics (#self_input, #input_pat: u16) #variadic #output {
                        self.#ident(Length::Dvw(#input_pat))
                    }
                }
            });
        }
    }

    let vis = &item.vis;
    let generics = &item.generics;
    let ident = &item.ident;
    let trait_ext_ident = format_ident!("{ident}Ext");

    Ok(quote! {
        #item

        #vis trait #trait_ext_ident #generics: #ident #generics {
            #(#items)*
        }
    })
}
