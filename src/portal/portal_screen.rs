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
