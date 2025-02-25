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
        ParallaxX::wrapped(Frac::cent(20), Frac::cent(100)),
    ));
    commands.spawn((
        Name::new("Grass"),
        AnimMan::new(GrassAnim::Grass),
        ParallaxX::wrapped(Frac::whole(2), Frac::cent(100)),
    ));
}

pub(super) fn register_bgfg(app: &mut App) {
    app.add_systems(Startup, startup);
}
