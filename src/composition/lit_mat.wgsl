#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var pixels_texture: texture_2d<f32>;
@group(2) @binding(2)
var pixels_splr: sampler;

@group(2) @binding(3)
var brightness_texture: texture_2d<f32>;
@group(2) @binding(4)
var brightness_splr: sampler;

@group(2) @binding(5)
var reflexivity_texture: texture_2d<f32>;
@group(2) @binding(6)
var reflexivity_splr: sampler;

@group(2) @binding(7)
var light_texture: texture_2d<f32>;
@group(2) @binding(8)
var light_splr: sampler;

@group(2) @binding(9)
var<uniform> base_light: vec4<f32>;

@group(2) @binding(10)
var<uniform> brightness_threshold_unused_unused_unused: vec4<f32>;

fn first_play(in: VertexOutput) -> vec4<f32> {
    var raw_pixel = textureSample(pixels_texture, pixels_splr, in.uv);

    var sampled_light = textureSample(light_texture, light_splr, in.uv);
    var total_light = sampled_light + base_light;
    var m = max(total_light.x, max(total_light.y, total_light.z)) * 0.1;

    return raw_pixel * total_light;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return first_play(in);
}
