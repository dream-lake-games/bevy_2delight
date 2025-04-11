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

defn_shader!(
    RainShader,
    #[filepath("platformer/shaders/rain.wgsl")]
    #[size(240, 240)]
    #[reps(3, 3)]
    pub struct RainShader {
        rain_amount: f32,
        rain_length: f32,
        rain_speed: f32,
        rain_color: Vec4,
    }
);

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Mountain"),
        AnimMan::new(MountainAnim::Mountain),
        ParallaxX::wrapped(0.2, 1),
    ));
    commands.spawn((
        Name::new("Grass"),
        AnimMan::new(GrassAnim::Grass),
        ParallaxX::wrapped(1.3, 1),
    ));
    commands.spawn((
        Name::new("RainShader"),
        ShaderMan::new(RainShader {
            rain_amount: 60.0,
            rain_length: 0.02,
            rain_speed: 70.0,
            rain_color: color_as_vec4(Color::WHITE),
        })
        .with_layer(Layer::Bg),
        Pos::default().with_z(10.0),
        ParallaxX::wrapped(0.95, 1),
        ParallaxY::wrapped(0.95, 1),
    ));
    commands.spawn((
        Name::new("RainShader"),
        ShaderMan::new(RainShader {
            rain_amount: 60.0,
            rain_length: 0.06,
            rain_speed: 80.0,
            rain_color: color_as_vec4(Color::WHITE),
        })
        .with_layer(Layer::Fg),
        Pos::default().with_z(10.0),
        ParallaxX::wrapped(1.05, 1),
        ParallaxY::wrapped(1.05, 1),
    ));
}

pub(super) fn register_bgfg(app: &mut App) {
    app.add_systems(Startup, startup);
}
