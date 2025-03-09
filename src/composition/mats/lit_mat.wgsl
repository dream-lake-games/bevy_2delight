#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var pixels_texture: texture_2d<f32>;
@group(2) @binding(2)
var pixels_splr: sampler;

@group(2) @binding(3)
var light_texture: texture_2d<f32>;
@group(2) @binding(4)
var light_splr: sampler;

@group(2) @binding(5)
var<uniform> base_light: vec4<f32>;

fn first_play(in: VertexOutput) -> vec4<f32> {
    var raw_pixel = textureSample(pixels_texture, pixels_splr, in.uv);

    var sampled_light = textureSample(light_texture, light_splr, in.uv);
    var total_light = sampled_light + base_light;

    return raw_pixel * total_light;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return first_play(in);
}
