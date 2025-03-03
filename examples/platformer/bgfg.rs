use bevy::prelude::*;
use bevy_2delight::prelude::*;

defn_anim!(
    MountainAnim,
    #[folder("platformer/bg")]
    #[layer(Bg)]
    #[rep(3, 1)]
    pub enum MountainAnim {
        #[default]
        #[tag("mountain")]
        Mountain,
    }
);

defn_anim!(
    GrassAnim,
    #[folder("platformer/fg")]
    #[layer(Fg)]
    #[rep(3, 1)]
    pub enum GrassAnim {
        #[default]
        #[tag("grass")]
        Grass,
    }
);

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Mountain"),
        AnimMan::new(MountainAnim::Mountain),
        ParallaxX::wrapped(Fx::from_num(0.2), Fx::from_num(1)),
    ));
    commands.spawn((
        Name::new("Grass"),
        AnimMan::new(GrassAnim::Grass),
        ParallaxX::wrapped(Fx::from_num(1.3), Fx::from_num(1)),
    ));
}

pub(super) fn register_bgfg(app: &mut App) {
    app.add_systems(Startup, startup);
}
