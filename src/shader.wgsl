struct Globals {
    mvp: mat4x4<f32>,
    size: vec2<f32>,
    _pad0: u32,
    _pad1: u32,
};

struct Locals {
    position: vec3<f32>,
    velocity: vec2<f32>,
    color: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0)
@binding(0)
var<uniform> globals: Globals;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
) -> VertexOutput {
    let tc = vec2<f32>(0.0, 0.0);
    //96	192	219
    let color = vec4<f32>(96.0, 148., 255., 255.0) / 255.0;
    let pos = vec4<f32>(globals.mvp * position);

    return VertexOutput(pos, tc, color);
}


@group(0)
@binding(1)
var sam: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    //return vertex.color * textureSampleLevel(tex, sam, vertex.tex_coords, 0.0);
    return vertex.color;
}
