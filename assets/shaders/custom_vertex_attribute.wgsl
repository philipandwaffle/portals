#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

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
    let foo = view.view_proj * mesh.model * vec4<f32>(vertex.position, 1.0);
    // out.clip_position = foo;
    // out.blend_color = vertex.blend_color;
    // out.blend_color = foo;

    // out.position = vec4<f32>(vertex.position, 10.0);
    out.position = foo;
    out.blend_color = vec4<f32>(vertex.position, 1.0);
    return out;
}

// fn mesh_position_local_to_clip(model: mat4x4<f32>, vertex_position: vec4<f32>) -> vec4<f32> {
//     let world_position = mesh_position_local_to_world(model, vertex_position);
//     return mesh_position_world_to_clip(world_position);
// }
// fn mesh_position_local_to_world(model: mat4x4<f32>, vertex_position: vec4<f32>) -> vec4<f32> {
//     return model * vertex_position;
// }
// fn mesh_position_world_to_clip(world_position: vec4<f32>) -> vec4<f32> {
//     return view.view_proj * world_position;
// }

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.blend_color;
    // return material.color * input.blend_color;
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
