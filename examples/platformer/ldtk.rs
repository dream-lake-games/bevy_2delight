use bevy::prelude::*;
use bevy_2delight::prelude::*;

#[derive(
    Clone, Copy, Debug, Default, strum_macros::EnumIter, Reflect, PartialEq, Eq, std::hash::Hash,
)]
pub(super) enum LdtkRoot {
    #[default]
    CatchAll,
    Dirt,
    Player,
    Platforms,
}
impl LdtkRootKind for LdtkRoot {}
pub(super) type LdtkSettings = LdtkSettingsGeneric<LdtkRoot>;
pub(super) type LdtkEntityPlugin<B> = LdtkEntityPluginGeneric<LdtkRoot, B>;
pub(super) type LdtkIntCellValuePlugin<B> = LdtkIntCellValuePluginGeneric<LdtkRoot, B>;
pub(super) type LdtkRootRes = LdtkRootResGeneric<LdtkRoot>;

#[derive(Bundle)]
struct DirtBundle {
    name: Name,
    pos: Pos,
    stx: StaticTx,
}
impl LdtkIntCellValue<LdtkRoot> for DirtBundle {
    const ROOT: LdtkRoot = LdtkRoot::Dirt;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("Dirt"),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, HBox::new(8, 8)),
        }
    }
}

fn startup(mut commands: Commands) {
    commands.trigger(LoadLdtk::new(
        "platformer/world/world.ldtk",
        "289cb4b0-e920-11ef-8ebd-c3d98294065b",
    ));
}

pub(super) fn register_ldtk(app: &mut App) {
    app.register_ldtk_int_cell_layer("DirtStatic", Layer::Static);
    app.register_ldtk_int_cell_layer("DirtAmbience", Layer::AmbientPixels);
    app.register_ldtk_int_cell_layer("DirtDetail", Layer::DetailPixels);

    app.add_plugins(LdtkIntCellValuePlugin::<DirtBundle>::single(
        "DirtStatic",
        1,
    ));

    app.add_systems(Startup, startup);
}
