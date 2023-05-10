use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::{mesh::MeshVertexAttribute, render_resource::VertexFormat},
};
use bevy_rapier3d::prelude::*;

use self::{
    custom_vertex_attribute::CustomMaterial,
    portal_camera::{rotate_portal_cams, translate_portal_cams},
    portal_pair::{create_portals, PortalPair, PortalPairs},
    texture_binding_array::BindlessMaterial,
};

mod custom_vertex_attribute;
mod portal_camera;
mod portal_pair;
mod portal_screen;
mod texture_binding_array;
pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PortalPairs {
            portals: vec![PortalPair {
                a_pos: vec3(-1.0, -1.0, 0.0),
                b_pos: vec3(1.0, -1.0, 0.0),
                a_res: [512, 1024],
                b_res: [512, 1024],
                a_size: vec2(1.0, 2.0),
                b_size: vec2(1.0, 2.0),
                a_quat: Quat::IDENTITY,
                b_quat: Quat::IDENTITY,
            }],
        })
        .add_plugin(MaterialPlugin::<BindlessMaterial>::default())
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        // .add_startup_system(setup_scene)
        .add_startup_system(create_portals)
        // .add_startup_system(spawn_stuff)
        // .add_system(control_screen_cam)
        .add_system(rotate_portal_cams)
        .add_system(translate_portal_cams);
    }
}

fn spawn_stuff(
    mut commands: Commands,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // cube
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });
    mesh.insert_attribute(
        MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4),
        // The cube mesh has 24 vertices (6 faces, 4 vertices per face), so we insert one BlendColor for each
        vec![[1.0, 0.0, 0.0, 1.0]; 24],
    );

    commands
        .spawn(Name::new("Box"))
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(CustomMaterial {
                color: Color::WHITE,
            }),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 5.0, 0.0)));
}

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
