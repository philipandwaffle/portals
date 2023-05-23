#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::utils

struct CustomMaterial {
    color: vec4<f32>,
};
@group(1) @binding(0)
var<uniform> material: CustomMaterial;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) blend_color: vec4<f32>,
// };
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var x = vertex.position.x;
    var y = vertex.position.y;
    var z = vertex.position.z;
    // x += rand(globals.time) / 200.0;
    // y += rand(globals.time) / 200.0;
    // z += rand(globals.time) / 200.0;

    let foo = view.view_proj * mesh.model * vec4<f32>(x, y, z, 1.0);
    // out.clip_position = foo;
    // out.blend_color = vertex.blend_color;
    // out.blend_color = foo;

    // out.position = vec4<f32>(vertex.position, 10.0);
    out.position = foo;
    out.blend_color = vec4<f32>(vertex.position, 1.0);
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(@builtin(position) position: vec4<f32>, input: FragmentInput) -> @location(0) vec4<f32> {
    let uv = coords_to_viewport_uv(position.xy, view.viewport);
    return vec4<f32>(uv.xy, 0.0, 0.0);
    // return material.color * input.blend_color;
}
// @fragment
// fn fragment(
//     @builtin(position) position: vec4<f32>,
//     #import bevy_pbr::mesh_vertex_output
// ) -> @location(0) vec4<f32> {
//     let uv = coords_to_viewport_uv(position.xy, view.viewport);
//     let color = textureSample(texture, texture_sampler, uv);
//     return color;
// }

fn rand(n: f32) -> f32 { return fract(sin(n) * 43758.5453123); }
fn noise(p: f32) -> f32 {
    let fl = floor(p);
    let fc = fract(p);
    return mix(rand(fl), rand(fl + 1.), fc);
}

// #import bevy_pbr::mesh_view_bindings
// #import bevy_pbr::mesh_bindings

// struct CustomMaterial {
//     color: vec4<f32>,
// };
// @group(1) @binding(0)
// var<uniform> material: CustomMaterial;

// // NOTE: Bindings must come before functions that use them!
// #import bevy_pbr::mesh_functions

// struct Vertex {
//     @location(0) position: vec3<f32>,
//     @location(1) blend_color: vec4<f32>,
// };

// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) blend_color: vec4<f32>,
// };

// @vertex
// fn vertex(vertex: Vertex) -> VertexOutput {
//     var out: VertexOutput;
//     out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
//     out.blend_color = vertex.blend_color;
//     return out;
// }

// struct FragmentInput {
//     @location(0) blend_color: vec4<f32>,
// };

// @fragment
// fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
//     return material.color * input.blend_color;
// }
