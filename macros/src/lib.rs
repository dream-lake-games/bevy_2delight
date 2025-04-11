use anim::produce_anim_derive;
use light::produce_light_derive;
use parse_helpers::{
    find_optional_attr, find_required_attr, get_pair_lit_int, get_single_ident, get_single_lit_str,
};
use syn::parse_macro_input;

mod anim;
mod light;
mod parse_helpers;

#[proc_macro_derive(
    AnimStateMachine,
    attributes(folder, layer, time_class, rep, tag, fps, offset, zix, next)
)]
pub fn anim_state_machine_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    produce_anim_derive(ast)
}

#[proc_macro_derive(LightStateMachine, attributes(on))]
pub fn light_state_machine_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    produce_light_derive(ast)
}

#[proc_macro_derive(TriggerKind)]
pub fn trigger_kind_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let enum_ident = &ast.ident;

    quote::quote! {
        impl bevy_2delight::prelude::TriggerKindTrait for #enum_ident {}
    }
    .into()
}

#[proc_macro_derive(ShaderSpec, attributes(filepath, size, reps, layer))]
pub fn derive_shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let filepath = get_single_lit_str("filepath", find_required_attr!(input, "filepath"));
    let (size_x, size_y) = get_pair_lit_int::<u32>("size", find_required_attr!(input, "size"));
    let layer_toks = match find_optional_attr!(input, "layer")
        .map(|attr| get_single_ident("layer", attr))
    {
        Some(thing) => quote::quote! {
            const DEFAULT_LAYER: bevy_2delight::prelude::Layer = bevy_2delight::prelude::Layer::#thing;
        },
        None => quote::quote! {},
    };
    let reps_toks = match find_optional_attr!(input, "reps")
        .map(|attr| get_pair_lit_int::<u32>("reps", attr))
    {
        Some((x, y)) => quote::quote! {
            const DEFAULT_REPS: bevy::prelude::UVec2 = bevy::prelude::UVec2::new(#x, #y);
        },
        None => quote::quote! {},
    };
    quote::quote! {
        impl ShaderSpec for #name {
            const SHADER_PATH: &'static str = #filepath;
            const DEFAULT_SIZE: bevy::prelude::UVec2 = bevy::prelude::UVec2::new(#size_x, #size_y);
            #layer_toks
            #reps_toks
        }
    }
    .into()
}
