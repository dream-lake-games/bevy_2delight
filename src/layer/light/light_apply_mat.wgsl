#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var image_texture: texture_2d<f32>;
@group(2) @binding(2)
var image_splr: sampler;

@group(2) @binding(3)
var light_texture: texture_2d<f32>;
@group(2) @binding(4)
var light_splr: sampler;

@group(2) @binding(5)
var<uniform> base_light: vec4<f32>;

fn vec_min(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    var result = a;
    if (b.x < a.x) {
        result.x = b.x;
    }
    if (b.y < a.y) {
        result.y = b.y;
    }
    if (b.z < a.z) {
        result.z = b.z;
    }
    if (b.w < a.w) {
        result.w = b.w;
    }
    return result;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(image_texture, image_splr, in.uv);
    let active_light = textureSample(light_texture, light_splr, in.uv);
    let total_light = vec_min(base_light + active_light, vec4<f32>(1.0));
    return vec4<f32>(
        original[0] * total_light[0],
        original[1] * total_light[1],
        original[2] * total_light[2],
        original[3] * total_light[3]
    );
}
