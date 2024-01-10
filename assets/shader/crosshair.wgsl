#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const ALPHA = 0.9;
const THICKNESS = 0.125;

@group(1) @binding(1)
var texture: texture_2d<f32>;

@group(1) @binding(2)
var oputput: sampler;

fn ddxy(v: f32, c: f32) -> f32 {
    if v < c {
        return 0.0;
    }

    return 1.0;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var center = 1.0 - abs(in.uv.xy - 0.5) * 2.0;
    var line_x = ddxy(center.y, 1.0 - THICKNESS);
    var line_y = ddxy(center.x, 1.0 - THICKNESS);
    var gap = ddxy(min(center.x, center.y), 1.0 - THICKNESS * 2.0);
    var alpha = clamp(line_x + line_y, 0.0, 1.0) - gap;
    return vec4<f32>(0.8, 0.8, 0.8, ALPHA * alpha);
}
