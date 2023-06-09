use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::{mesh::MeshVertexAttribute, render_resource::VertexFormat},
};
use bevy_rapier3d::prelude::*;

use self::{
    custom_vertex_attribute::CustomMaterial,
    portal_camera::{rotate_portal_cams, translate_portal_cams},
    portal_pair::{create_portals, PortalPairSpawns},
    texture_binding_array::BindlessMaterial,
};

mod custom_vertex_attribute;
mod portal_camera;
mod portal_pair;
mod portal_screen;
mod texture_binding_array;
pub struct PortalTestingPlugin;
impl Plugin for PortalTestingPlugin {
    fn build(&self, app: &mut App) {
        let mut portal_spawns = PortalPairSpawns::new();
        // portal_spawns.add_portal(
        //     vec3(0.0, 1.0, -0.5),
        //     vec3(0.0, 1.0, 0.5),
        //     [1024, 1024],
        //     [1024, 1024],
        //     vec2(2.0, 2.0),
        //     vec2(2.0, 2.0),
        //     Quat::from_rotation_y(PI),
        //     Quat::IDENTITY,
        // );

        portal_spawns.add_portal(
            vec3(0.0, 1.0, 10.0),
            vec3(0.0, 1.0, -10.0),
            [1024, 1024],
            [1024, 1024],
            vec2(2.0, 2.0),
            vec2(2.0, 2.0),
            Quat::from_rotation_y(PI),
            Quat::IDENTITY,
        );

        app.insert_resource(portal_spawns)
            .add_plugin(MaterialPlugin::<BindlessMaterial>::default())
            .add_plugin(MaterialPlugin::<CustomMaterial>::default())
            // .add_startup_system(setup_scene)
            .add_startup_system(create_portals)
            .add_startup_system(spawn_stuff)
            // .add_system(control_screen_cam)
            .add_system(translate_portal_cams)
            .add_system(rotate_portal_cams);
    }
}

fn spawn_stuff(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let size = 1.0;
    // cube
    let mut mesh = Mesh::from(shape::Cube { size });
    mesh.insert_attribute(
        MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4),
        // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
        vec![[1.0, 0.0, 0.0, 1.0]; 24],
    );

    commands
        .spawn(Name::new("Box"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(size / 2.0, size / 2.0, size / 2.0))
        .insert(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(CustomMaterial {
                color: Color::WHITE,
            }),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(10.0, 5.0, 0.0)));
}

// fn shader_testing(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut bindless_materials: ResMut<Assets<BindlessMaterial>>,
// ) {
//     commands.spawn(bundle)
// }

// fn control_screen_cam(
//     mut cams: Query<&mut Transform, With<PortalCamera>>,
//     input: Res<Input<KeyCode>>,
// ) {
//     if cams.is_empty() {
//         return;
//     }

//     let mut delta_z = 0.0;
//     if input.pressed(KeyCode::Up) {
//         delta_z -= 1.0;
//     }
//     if input.pressed(KeyCode::Down) {
//         delta_z += 1.0;
//     }

//     let mut delta_x = 0.0;
//     if input.pressed(KeyCode::Left) {
//         delta_x -= 1.0;
//     }
//     if input.pressed(KeyCode::Right) {
//         delta_x += 1.0;
//     }

//     for mut trans in cams.iter_mut() {
//         trans.translation += vec3(delta_x, 0.0, delta_z) * 0.2;
//     }
// }
