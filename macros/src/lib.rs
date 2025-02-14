use anim::produce_anim_derive;
use light::produce_light_derive;

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

#[proc_macro_derive(TriggerKind, attributes(on))]
pub fn trigger_kind_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let enum_ident = &ast.ident;

    quote::quote! {
        impl bevy_2delight::prelude::TriggerKindTrait for #enum_ident {}
    }
    .into()
}
