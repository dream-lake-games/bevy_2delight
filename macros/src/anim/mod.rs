use syn::{DeriveInput, Ident};

mod tag_info;

use crate::parse_helpers::*;
use tag_info::*;

struct EnumInfo {
    folder: String,
    layer: Option<Ident>,
    zix: Option<f32>,
    time_class: Option<Ident>,
    rep: Option<(u32, u32)>,
}

#[derive(Clone)]
struct VariantInfo {
    ident: Ident,
    tag: String,
    tag_info: TagInfo,
    fps: Option<u32>,
    offset: Option<(i32, i32)>,
    next: Option<Ident>,
}

pub(super) fn produce_anim_derive(ast: DeriveInput) -> proc_macro::TokenStream {
    let enum_ident = &ast.ident;
    let enum_info = EnumInfo {
        folder: get_single_lit_str("folder", find_required_attr!(ast, "folder")),
        layer: find_optional_attr!(ast, "layer").map(|attr| get_single_ident("layer", attr)),
        zix: find_optional_attr!(ast, "zix").map(|attr| get_single_lit_float("zix", attr)),
        time_class: find_optional_attr!(ast, "time_class")
            .map(|attr| get_single_ident("time_class", attr)),
        rep: find_optional_attr!(ast, "rep").map(|attr| get_pair_lit_int::<u32>("rep", attr)),
    };

    let mut variant_infos = vec![];
    let variants = match &ast.data {
        syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
        _ => panic!("Expected an enum with named variants"),
    };
    for variant in variants {
        let tag = get_single_lit_str("tag", find_required_attr!(variant, "tag"));
        let mut path = std::path::Path::new("assets")
            .join(&enum_info.folder)
            .join(&tag);
        path.set_extension("json");
        if !path.exists() {
            panic!("Anim tag JSON not found: {}", path.display());
        }
        let tag_info = TagInfo::from_path(&path).unwrap();

        let info = VariantInfo {
            ident: variant.ident.clone(),
            tag,
            tag_info,
            fps: find_optional_attr!(variant, "fps").map(|a| get_single_lit_int("fps", a)),
            offset: find_optional_attr!(variant, "offset").map(|a| get_pair_lit_int("offset", a)),
            next: find_optional_attr!(variant, "next").map(|a| get_single_ident("next", a)),
        };
        variant_infos.push(info);
    }

    if variant_infos.len() == 0 {
        panic!("The AnimStateMachine must have at least one state");
    }
    let fishing_info = variant_infos.iter().next().unwrap();
    let (w, h) = (fishing_info.tag_info.w, fishing_info.tag_info.h);
    let (rep_x, rep_y) = enum_info.rep.unwrap_or((1, 1));

    let zix = enum_info.zix.unwrap_or(0.0);
    let time_class = match enum_info.time_class {
        Some(class) => quote::quote! { Some(AnimTimeClass::#class) },
        None => quote::quote!(None),
    };

    let handle_map_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        let path = format!("{}/{}.png", enum_info.folder, variant_info.tag);
        quote::quote! { map.insert(Self::#ident, ass.load(#path)); }
    });

    let get_filepath_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        let path = format!("{}/{}.png", enum_info.folder, variant_info.tag);
        quote::quote! { Self::#ident => #path, }
    });

    let get_length_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        let length = variant_info.tag_info.length;
        quote::quote! { Self::#ident => #length, }
    });

    let get_fps_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        let fps = variant_info.fps.unwrap_or(30);
        quote::quote! { Self::#ident => #fps, }
    });

    let get_offset_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        let (off_x, off_y) = variant_info.offset.unwrap_or((0, 0));
        quote::quote! { Self::#ident => IVec2::new(#off_x, #off_y), }
    });

    let get_next_tokens = variant_infos.clone().into_iter().map(|variant_info| {
        let ident = variant_info.ident;
        let next = variant_info.next;
        match next {
            Some(next) => {
                if next.to_string().as_str() == "Despawn" || next.to_string().as_str() == "Remove" {
                    quote::quote! { Self::#ident => bevy_2delight::prelude::AnimNextState::#next, }
                } else {
                    quote::quote! { Self::#ident => bevy_2delight::prelude::AnimNextState::Some(Self::#next), }
                }
            }
            None => quote::quote! { Self::#ident => bevy_2delight::prelude::AnimNextState::Stay, },
        }
    });

    quote::quote! {
        impl bevy_2delight::prelude::AnimStateMachine for #enum_ident {
            const SIZE: UVec2 = UVec2::new(#w, #h);
            const ZIX: f32 = #zix;
            const TIME_CLASS: Option<bevy_2delight::prelude::AnimTimeClass> = #time_class;
            const REP: UVec2 = UVec2::new(#rep_x, #rep_y);

            fn make_handle_map(
                ass: &bevy::prelude::Res<bevy::prelude::AssetServer>
            ) -> bevy::utils::HashMap<Self, Handle<bevy::prelude::Image>> {
                let mut map = bevy::utils::HashMap::new();
                #(#handle_map_tokens)*
                map
            }

            fn get_filepath(&self) -> &'static str {
                match self {
                    #(#get_filepath_tokens)*
                }
            }

            fn get_length(&self) -> u32 {
                match self {
                    #(#get_length_tokens)*
                }
            }

            fn get_fps(&self) -> u32 {
                match self {
                    #(#get_fps_tokens)*
                }
            }

            fn get_offset(&self) -> IVec2 {
                match self {
                    #(#get_offset_tokens)*
                }
            }

            fn get_next(&self) -> bevy_2delight::prelude::AnimNextState<Self> {
                match self {
                    #(#get_next_tokens)*
                }
            }
        }
    }
    .into()
}
