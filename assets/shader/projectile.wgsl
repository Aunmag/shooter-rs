#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var oputput: sampler;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    var x = 0.010 / abs(0.5 - uv.x);
    var y0 = saturate((1.0 - uv.y) * 100.0);
    var y1 = uv.y;
    return vec4<f32>(vec3<f32>(1.0, 1.0, 0.1), x * y0 * y1);
}
