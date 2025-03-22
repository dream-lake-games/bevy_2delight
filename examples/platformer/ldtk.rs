use bevy::{color::palettes::tailwind, prelude::*};
use bevy_2delight::prelude::*;
use rand::{thread_rng, Rng};

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
    #[layer(FrontDetailPixels)]
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
            pos,
            anim: default(),
            light: CircleLight::strength(24.0).with_color(Color::linear_rgb(1.0, 1.0, 0.4)),
            flicker: LightFlicker::new(24.0, 4.0, 4.0, 3.0, 0.1, 0.05),
        }
    }
}
fn juice_torch(torches: Query<&Pos, With<AnimMan<TorchAnim>>>, mut commands: Commands) {
    for pos in &torches {
        if thread_rng().gen_bool(0.05) {
            commands.spawn(
                Particle::new(*pos, Fx::from_num(2.0))
                    .with_pos_fuzz(0.5, 0.5)
                    .with_lifetime_fuzz(0.5)
                    .with_vel(FVec2::new(Fx::from_num(0), Fx::from_num(5)))
                    .with_vel_fuzz(1.0, 2.5)
                    .with_color_terp(
                        Color::srgb_u8(238, 191, 245),
                        Color::BLACK,
                        TerpMode::Linear,
                    )
                    .with_color_fuzz(Color::srgb(0.1, 0.1, 0.1))
                    .with_brightness_terp(
                        tailwind::YELLOW_300.into(),
                        Color::BLACK,
                        TerpMode::Linear,
                    )
                    .with_size_terp(Fx::from_num(2), Fx::from_num(1), TerpMode::Linear)
                    .with_size_fuzz(0.5)
                    .with_layer(Layer::FrontDetailPixels),
            );
        }
    }
}

defn_anim!(
    ShinyRockAnim,
    #[folder("platformer/world/detail/shiny_rock")]
    #[layer(BackDetailPixels)]
    pub enum ShinyRockAnim {
        #[default]
        #[tag("rock")]
        Rock,
    }
);
#[derive(Bundle)]
struct ShinyRockBundle {
    name: Name,
    pos: Pos,
    anim: AnimMan<ShinyRockAnim>,
}
impl LdtkEntity<LdtkRoot> for ShinyRockBundle {
    const ROOT: LdtkRoot = LdtkRoot::Detail;
    fn from_ldtk(
        pos: Pos,
        _fields: &bevy::utils::HashMap<String, bevy_ecs_ldtk::prelude::FieldValue>,
        _iid: String,
    ) -> Self {
        Self {
            name: Name::new("ShinyRock"),
            pos,
            anim: default(),
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
    app.register_ldtk_int_cell_layer("DirtDetail", Layer::BackDetailPixels);

    app.add_plugins(
        LdtkIntCellValuePlugin::<DirtBundle>::single("DirtStatic", 1).with_consolidate(8),
    );

    app.add_plugins(LdtkEntityPlugin::<TorchBundle>::new("Entities", "Torch"));
    app.add_plugins(LdtkEntityPlugin::<ShinyRockBundle>::new(
        "Entities",
        "ShinyRock",
    ));

    app.add_systems(Startup, startup);

    app.add_systems(Update, juice_torch);
}
