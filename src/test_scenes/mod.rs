use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::prelude::*;

pub fn mirrored_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // LET THERE BE LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn(Box::new(
        "Ground".into(),
        vec3(40.0, 1.0, 40.0),
        vec3(0.0, -0.5, 0.0),
        Color::WHITE,
        Some("textures/checker_board.png".into()),
        &mut meshes,
        &mut materials,
        &asset_server,
    ));

    commands.spawn(Box::new(
        "Green Box 0".into(),
        vec3(1.5, 2.0, 1.5),
        vec3(2.5, 1.0, 10.0),
        Color::GREEN,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Green Box 1".into(),
        vec3(0.5, 6.0, 0.5),
        vec3(4.0, 3.0, 9.0),
        Color::GREEN,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Green Box 2".into(),
        vec3(1.5, 1.0, 1.5),
        vec3(-1.5, 0.5, 7.0),
        Color::GREEN,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));

    commands.spawn(Box::new(
        "Blue Box 0".into(),
        vec3(1.5, 2.0, 1.5),
        vec3(-2.5, 1.0, -10.0),
        Color::BLUE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Blue Box 1".into(),
        vec3(0.5, 6.0, 0.5),
        vec3(-4.0, 3.0, -9.0),
        Color::BLUE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Blue Box 2".into(),
        vec3(1.5, 1.0, 1.5),
        vec3(1.5, 0.5, -7.0),
        Color::BLUE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));

    commands.spawn(Box::new(
        "Top Wall".into(),
        vec3(6.0, 4.0, 1.0),
        vec3(0.0, 4.0, 0.0),
        Color::WHITE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Left Wall".into(),
        vec3(2.0, 2.0, 1.0),
        vec3(-2.0, 1.0, 0.0),
        Color::WHITE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Right Wall".into(),
        vec3(2.0, 2.0, 1.0),
        vec3(2.0, 1.0, 0.0),
        Color::WHITE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
}

pub fn facing_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // LET THERE BE LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

    commands.spawn(Box::new(
        "Ground".into(),
        vec3(40.0, 1.0, 40.0),
        vec3(0.0, -0.5, 0.0),
        Color::WHITE,
        Some("textures/checker_board.png".into()),
        &mut meshes,
        &mut materials,
        &asset_server,
    ));

    commands.spawn(Box::new(
        "Green Box 0".into(),
        vec3(1.5, 2.0, 1.5),
        vec3(2.5, 1.0, 10.0),
        Color::GREEN,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Green Box 1".into(),
        vec3(0.5, 6.0, 0.5),
        vec3(4.0, 3.0, 9.0),
        Color::GREEN,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Green Box 2".into(),
        vec3(1.5, 1.0, 1.5),
        vec3(-1.5, 0.5, 7.0),
        Color::GREEN,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));

    commands.spawn(Box::new(
        "Blue Box 0".into(),
        vec3(1.5, 2.0, 1.5),
        vec3(-2.5, 1.0, -10.0),
        Color::BLUE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Blue Box 1".into(),
        vec3(0.5, 6.0, 0.5),
        vec3(-4.0, 3.0, -9.0),
        Color::BLUE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
    commands.spawn(Box::new(
        "Blue Box 2".into(),
        vec3(1.5, 1.0, 1.5),
        vec3(1.5, 0.5, -7.0),
        Color::BLUE,
        None,
        &mut meshes,
        &mut materials,
        &asset_server,
    ));
}

#[derive(Bundle)]
struct Box {
    name: Name,
    col: Collider,
    material_mesh_bundle: MaterialMeshBundle<StandardMaterial>,
}
impl Box {
    fn new(
        name: String,
        size: Vec3,
        pos: Vec3,
        color: Color,
        texture_path: Option<String>,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let texture;
        if let Some(path) = texture_path {
            texture = Some(asset_server.load(path));
        } else {
            texture = None;
        }

        return Self {
            name: Name::new(name),
            col: Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            material_mesh_bundle: MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z))),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    base_color_texture: texture,
                    ..default()
                }),
                transform: Transform::from_translation(pos),
                ..default()
            },
        };
    }
}
