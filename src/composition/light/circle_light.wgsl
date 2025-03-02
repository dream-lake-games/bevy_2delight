#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(1)
var<uniform> color: vec4<f32>;
@group(2) @binding(2)
var<uniform> sx_sy_rings_unused: vec4<f32>;

const ring_width: f32 = 0.1;

fn ellip_distance(in: vec2<f32>) -> f32 {
    var x_comp = (in.x - 0.5) * (in.x - 0.5) / (sx_sy_rings_unused[0]  * sx_sy_rings_unused[0]);
    var y_comp = (in.y - 0.5) * (in.y - 0.5) / (sx_sy_rings_unused[1]  * sx_sy_rings_unused[1]);
    return sqrt(x_comp + y_comp);
}

// Returns the alpha given the ellipse_distance
fn apply_ring(ed: f32) -> f32 {
    if (ed > 1.0) {
        return 0.0;
    }
    var val_tran = (1.0 - ed * ed);
    var rings = sx_sy_rings_unused[2];
    var min_val = 0.1;
    return min_val + ceil(val_tran * rings) / rings * (1.0 - min_val);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var ed = ellip_distance(in.uv);
    var alpha = apply_ring(ed);
    return vec4<f32>(color.x, color.y, color.z, alpha);
}
