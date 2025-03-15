use bevy::prelude::*;
use bevy_2delight::prelude::*;

#[derive(
    Clone, Copy, Debug, Default, strum_macros::EnumIter, Reflect, PartialEq, Eq, std::hash::Hash,
)]
pub(super) enum LdtkRoot {
    #[default]
    Detail,
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
    occlude: OccludeLight,
}
impl LdtkIntCellValue<LdtkRoot> for DirtBundle {
    const ROOT: LdtkRoot = LdtkRoot::Dirt;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("Dirt"),
            pos,
            stx: StaticTx::single(StaticTxKind::Solid, HBox::new(8, 8)),
            occlude: OccludeLight::StaticTx,
        }
    }
}

defn_anim!(
    TorchAnim,
    #[folder("platformer/world/detail/torch")]
    pub enum TorchAnim {
        #[default]
        #[tag("burn")]
        #[fps(8)]
        Burn,
    }
);
#[derive(Bundle)]
struct TorchBundle {
    name: Name,
    pos: Pos,
    anim: AnimMan<TorchAnim>,
    light: CircleLight,
    flicker: LightFlicker,
}
impl LdtkEntity<LdtkRoot> for TorchBundle {
    const ROOT: LdtkRoot = LdtkRoot::Detail;
    fn from_ldtk(
        pos: Pos,
        _fields: &bevy::utils::HashMap<String, bevy_ecs_ldtk::prelude::FieldValue>,
        _iid: String,
    ) -> Self {
        Self {
            name: Name::new("Torch"),
            pos: pos.with_z(Fx::from_num(90)),
            anim: default(),
            light: CircleLight::strength(24.0).with_color(Color::linear_rgb(1.0, 1.0, 0.0)),
            flicker: LightFlicker::new(24.0, 4.0, 4.0, 3.0, 0.1, 0.05),
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
    app.register_ldtk_int_cell_layer("DirtStatic", Layer::StaticPixels);
    app.register_ldtk_int_cell_layer("DirtAmbience", Layer::AmbientPixels);
    app.register_ldtk_int_cell_layer("DirtDetail", Layer::DetailPixels);

    app.add_plugins(
        LdtkIntCellValuePlugin::<DirtBundle>::single("DirtStatic", 1).with_consolidate(8),
    );

    app.add_plugins(LdtkEntityPlugin::<TorchBundle>::new("Entities", "Torch"));

    app.add_systems(Startup, startup);
}
