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
    rain_length: f32,
    rain_speed: f32,
    rain_color: vec4<f32>,
}
@group(2) @binding(1)
var<uniform> rain_params: RainShader;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2(fract(in.uv.x * input.rep_x), fract(in.uv.y * input.rep_y));
    let time = input.bullet_time;
    
    let remainder = fract(uv.x * rain_params.rain_amount) / rain_params.rain_amount;
    let grid_x = uv.x - remainder;
    let rn = fract(sin(grid_x * rain_params.rain_amount * rain_params.rain_length * rain_params.rain_speed));
    
    let width = 0.3;
    let speed = rain_params.rain_speed / input.loop_time;
    
    let y_pos = fract(uv.y + rn * input.loop_time - time * speed);
    let is_raindrop = step(1.0 - rain_params.rain_length, y_pos) * step(remainder * rain_params.rain_amount, width);
    
    return vec4(rain_params.rain_color.xyz, rain_params.rain_color.w * is_raindrop);
}
