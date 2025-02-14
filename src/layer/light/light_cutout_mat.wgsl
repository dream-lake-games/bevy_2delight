#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var light_texture: texture_2d<f32>;
@group(2) @binding(2)
var light_splr: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let original = textureSample(light_texture, light_splr, in.uv);
    let avg = (original.x + original.y + original.z) / 3.0;
    return vec4<f32>(
        1.0, 1.0, 1.0,
        avg * avg 
    );
}
