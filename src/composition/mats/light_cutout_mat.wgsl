#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var input_texture: texture_2d<f32>;
@group(2) @binding(2)
var input_splr: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var val = textureSample(input_texture, input_splr, in.uv);
    if (val.x < 0.00001 && val.y < 0.00001 && val.z < 0.00001 && val.w > 0.99999) {
        return vec4<f32>(0);
    }
    return val;
}
