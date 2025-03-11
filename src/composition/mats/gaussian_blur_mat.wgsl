#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var input_texture: texture_2d<f32>;
@group(2) @binding(2)
var input_splr: sampler;

@group(2) @binding(3)
var<uniform> horizontal_ksize_unused_unused: vec4<i32>;

const SQ_TWO_PI: f32 = 2.5066282;
const E: f32 = 2.71828;

const sampling_distance_factor: f32 = 1.0;
// Based
const sigma: f32 = 1.5;
const sigma_square: f32 = sigma * sigma;

fn gaussian_weight(v: i32) -> f32 {
    return (1.0 / (SQ_TWO_PI * sigma)) * pow(E, -f32(v * v) / (2.0 * sigma_square));
}

fn blur(in_uv: vec2<f32>, horizontal: bool) -> vec4<f32> {
    let upper = (horizontal_ksize_unused_unused[1] - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    var total_weight = 0.0;
    
    let texture_size = vec2<f32>(textureDimensions(input_texture));
    let texel_size = 1.0 / texture_size;
    
    for (var v = lower; v <= upper; v++) {
        var uv = in_uv;
        if (horizontal) {
            uv += vec2<f32>(f32(v) * sampling_distance_factor * texel_size.x, 0.);
        } else {
            uv += vec2<f32>(0., f32(v) * sampling_distance_factor * texel_size.y);
        }

        let weight = gaussian_weight(v);
        total_weight += weight;
        color += weight * vec4(textureSample(input_texture, input_splr, uv).xyz, 1.0);
    }
    
    // Normalize by total weight to maintain consistent brightness
    return color;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return blur(in.uv, horizontal_ksize_unused_unused[0] == 1);
}