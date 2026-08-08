#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const COLOR = vec4<f32>(1.0, 1.0, 0.1, 1.0);

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let y = 0.01 / abs(0.5 - in.uv.y);
    let x0 = saturate((1.0 - in.uv.x) * 100.0);
    let x1 = in.uv.x;
    return vec4<f32>(COLOR.rgb, COLOR.a * y * x0 * x1);
}
