use bevy::prelude::*;

#[derive(Bundle)]
pub struct PortalScreenBundle {
    pub name: Name,
    pub portal_screen: PortalScreen,
    pub material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}

impl PortalScreenBundle {
    pub fn new(
        name: String,
        mesh_handle: Handle<Mesh>,
        material_handle: Handle<StandardMaterial>,
        translation: Vec3,
    ) -> Self {
        return Self {
            name: Name::new(name),
            portal_screen: PortalScreen,
            material_mesh_bundle: MaterialMeshBundle {
                mesh: mesh_handle,
                material: material_handle,
                transform: Transform::from_translation(translation),
                ..default()
            },
        };
    }
}

#[derive(Component)]
pub struct PortalScreen;

// .spawn(Name::new(format!("{}_screen_a", count)))
// .insert(PortalScreen)
// .insert(MaterialMeshBundle {
//     mesh: meshes.add(Mesh::from(shape::Quad {
//         size: portal_pair.a_size,
//         flip: false,
//     })),
//     // material: bindless_materials.add(foo),
//     // transform: Transform {
//     //     translation: portal.a_pos,
//     //     ..default()
//     // },
//     material: materials.add(StandardMaterial {
//         base_color: Color::WHITE,
//         base_color_texture: Some(image_handle_a.clone()),
//         ..default()
//     }),
//     ..default()
// })
// .insert(SpatialBundle {
//     transform: Transform::from_translation(portal_pair.a_pos),
//     ..Default::default()
// })
