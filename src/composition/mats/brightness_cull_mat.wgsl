#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var brightness_texture: texture_2d<f32>;
@group(2) @binding(2)
var brightness_splr: sampler;

@group(2) @binding(3)
var reflexivity_texture: texture_2d<f32>;
@group(2) @binding(4)
var reflexivity_splr: sampler;

@group(2) @binding(5)
var light_texture: texture_2d<f32>;
@group(2) @binding(6)
var light_splr: sampler;

@group(2) @binding(7)
var input_pixels_texture: texture_2d<f32>;
@group(2) @binding(8)
var input_pixels_splr: sampler;

@group(2) @binding(9)
var<uniform> bthreshold_unused_unused_unused: vec4<f32>;

fn get_brightness(uv: vec2<f32>) -> vec4<f32> {
    let measured_brightness = textureSample(brightness_texture, brightness_splr, uv);
    let measured_reflexivity = textureSample(reflexivity_texture, reflexivity_splr, uv);
    let measured_light = textureSample(light_texture, light_splr, uv);
    return measured_brightness + measured_reflexivity * measured_light;
}

fn threshold_brightness(raw_brightness: vec4<f32>) -> vec4<f32> {
    if (raw_brightness.x + raw_brightness.y + raw_brightness.z > bthreshold_unused_unused_unused[0]) {
        return vec4<f32>(raw_brightness.x, raw_brightness.y, raw_brightness.z, 1.0);
    } else {
        return vec4<f32>(0.0);
    }
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let raw_brightness = get_brightness(in.uv);
    let pixel_value = textureSample(input_pixels_texture, input_pixels_splr, in.uv);
    let pixel_luminance = 0.2126 * pixel_value.x + 0.7152 * pixel_value.y + 0.0722 * pixel_value.z;
    let total_brightness = raw_brightness + pixel_value * pixel_luminance;
    var brightness = threshold_brightness(total_brightness);
    return brightness;
}
