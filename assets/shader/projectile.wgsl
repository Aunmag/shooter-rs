#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var oputput: sampler;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var y = 0.010 / abs(0.5 - uv.y);
    var x0 = saturate((1.0 - uv.x) * 100.0);
    var x1 = uv.x;
    return vec4<f32>(vec3<f32>(1.0, 1.0, 0.1), y * x0 * x1);
}
