use bevy::{input::common_conditions::input_toggle_active, prelude::*};

use super::{
    pos::Pos,
    prelude::{StaticRx, StaticTx, TriggerKindTrait, TriggerRxGeneric, TriggerTxGeneric},
    PhysicsSet,
};

fn draw_hitboxes<TriggerRx: TriggerKindTrait, TriggerTx: TriggerKindTrait>(
    srx_q: Query<(&Pos, &StaticRx)>,
    stx_q: Query<(&Pos, &StaticTx)>,
    trx_q: Query<(&Pos, &TriggerRxGeneric<TriggerRx>)>,
    ttx_q: Query<(&Pos, &TriggerTxGeneric<TriggerTx>)>,
    mut gz: Gizmos,
) {
    for hbox in srx_q
        .iter()
        .map(|pair| pair.1.get_thboxes(*pair.0))
        .chain(stx_q.iter().map(|pair| pair.1.get_thboxes(*pair.0)))
        .chain(trx_q.iter().map(|pair| pair.1.get_thboxes(*pair.0)))
        .chain(ttx_q.iter().map(|pair| pair.1.get_thboxes(*pair.0)))
        .flat_map(|v| v)
    {
        gz.rect_2d(
            Isometry2d::from_translation(hbox.get_offset().as_vec2()),
            hbox.get_size().as_vec2(),
            Color::WHITE,
        );
    }
}

pub(super) struct PhysicsDebugPluginGeneric<
    TriggerRxKind: TriggerKindTrait,
    TriggerTxKind: TriggerKindTrait,
> {
    _pd: std::marker::PhantomData<(TriggerRxKind, TriggerTxKind)>,
}
impl<TriggerRx: TriggerKindTrait, TriggerTx: TriggerKindTrait> Default
    for PhysicsDebugPluginGeneric<TriggerRx, TriggerTx>
{
    fn default() -> Self {
        Self { _pd: default() }
    }
}
impl<TriggerRx: TriggerKindTrait, TriggerTx: TriggerKindTrait> Plugin
    for PhysicsDebugPluginGeneric<TriggerRx, TriggerTx>
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_hitboxes::<TriggerRx, TriggerTx>
                .after(PhysicsSet)
                .run_if(input_toggle_active(false, KeyCode::KeyH)),
        );
    }
}
