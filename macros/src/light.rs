use syn::{DeriveInput, Ident};

use crate::parse_helpers::*;

#[derive(Clone)]
struct VariantInfo {
    ident: Ident,
    radius: Option<u32>,
}

pub(super) fn produce_light_derive(ast: DeriveInput) -> proc_macro::TokenStream {
    let enum_ident = &ast.ident;

    let mut variant_infos = vec![];
    let variants = match &ast.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => panic!("Expected an enum with named variants"),
    };
    for variant in variants {
        let on = find_optional_attr!(variant, "on").map(|a| get_single_lit_int("on", a));

        let info = VariantInfo {
            ident: variant.ident.clone(),
            radius: on,
        };
        variant_infos.push(info);
    }

    if variant_infos.len() == 0 {
        panic!("The LightStateMachine must have at least one state");
    }

    let light_radius_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        match variant_info.radius {
            Some(ru32) => quote::quote! { Self::#ident => Some(Frac::const_whole(#ru32)) },
            None => quote::quote! { Self::#ident => None },
        }
    });

    quote::quote! {
        impl bevy_2delight::prelude::LightStateMachine for #enum_ident {
            fn light_radius(&self) -> Option<bevy_2delight::prelude::Frac> {
                match self {
                    #(#light_radius_tokens)*
                }
            }
        }
    }
    .into()
}
