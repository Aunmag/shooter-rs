#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0)
var texture: texture_2d<f32>;

@group(2) @binding(1)
var oputput: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var y = 0.01 / abs(0.5 - in.uv.y);
    var x0 = saturate((1.0 - in.uv.x) * 100.0);
    var x1 = in.uv.x;
    return vec4<f32>(1.0, 1.0, 0.1, y * x0 * x1);
}
