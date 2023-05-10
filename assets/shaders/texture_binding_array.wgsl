#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};


@group(1) @binding(0)
var textures: binding_array<texture_2d<f32>>;
@group(1) @binding(1)
var nearest_sampler: sampler;
// We can also have array of samplers
// var samplers: binding_array<sampler>;

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return  input.blend_color;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // let foo = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    let foo = mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
    // let foo = mesh_position_world_to_clip(vec4<f32>(vertex.position, 1.0));
    out.clip_position = vec4<f32>(vertex.position, 0.5);
    // out.blend_color = vertex.blend_color;
    out.blend_color = vec4<f32>(1.0,0.5,0.5,1.0);
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