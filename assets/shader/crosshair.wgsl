#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const COLOR = vec4<f32>(0.8, 0.8, 0.8, 0.9);
const THICKNESS = 0.125;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let center = 1.0 - abs(in.uv.xy - 0.5) * 2.0;
    let line_x = step(1.0 - THICKNESS, center.y);
    let line_y = step(1.0 - THICKNESS, center.x);
    let gap = step(1.0 - THICKNESS * 2.0, min(center.x, center.y));
    let alpha = clamp(line_x + line_y, 0.0, 1.0) - gap;
    return vec4<f32>(COLOR.rgb, COLOR.a * alpha);
}
