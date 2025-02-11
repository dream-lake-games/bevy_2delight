use anim::produce_anim_derive;

mod anim;
mod parse_helpers;

#[proc_macro_derive(
    AnimStateMachine,
    attributes(folder, layer, time_class, rep, tag, fps, offset, zix, next)
)]
pub fn anim_state_machine_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    produce_anim_derive(ast)
}
