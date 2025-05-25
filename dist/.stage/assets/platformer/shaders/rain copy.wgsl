#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct ShaderInput {
    bullet_time: f32,
    real_time: f32,
    loop_time: f32,
    rep_x: f32,
    rep_y: f32,
}
@group(2) @binding(0)
var<uniform> input: ShaderInput;

struct RainShader {
    rain_amount: f32,
    near_rain_length: f32,
    far_rain_length: f32,
    near_rain_transparency: f32,
    far_rain_transparency: f32,
    base_rain_speed: f32,
    additional_rain_speed: f32,
    rain_color: vec3<f32>,
}

var<private> rain_params: RainShader = RainShader(
    150.0,    // rain_amount
    0.1,      // near_rain_length
    0.02,     // far_rain_length
    1.0,      // near_rain_transparency
    0.5,      // far_rain_transparency
    1.1,     // base_rain_speed
    0.6,     // additional_rain_speed
    vec3(0.4, 0.4, 0.9),  // rain_color
);

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2(fract(in.uv.x * input.rep_x), fract(in.uv.y * input.rep_y));
    let time = input.bullet_time;
    
    let remainder = fract(uv.x * rain_params.rain_amount) / rain_params.rain_amount;
    let grid_x = uv.x - remainder;
    
    let rn = fract(sin(grid_x * rain_params.rain_amount));
    
    let length = mix(rain_params.far_rain_length, rain_params.near_rain_length, rn);
    let width = 0.5;
    let transparency = mix(rain_params.far_rain_transparency, rain_params.near_rain_transparency, rn);
    let speed = 5.0 / input.loop_time;
    
    let y_pos = fract(uv.y + rn * input.loop_time - time * speed);
    let is_raindrop = step(1.0 - length, y_pos) * step(remainder * rain_params.rain_amount, width);
    
    return vec4(rain_params.rain_color.xyz, is_raindrop * transparency);
}
