#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var oputput: sampler;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let v = abs(0.5 - uv.y);
  	var c = 0.005 / v;
  	var a = 0.02 / v;
    return vec4<f32>(1.0, c, c, a * uv.x * 0.5);
}
