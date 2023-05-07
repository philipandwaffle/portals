use bevy::{math::vec3, prelude::*, render::camera::RenderTarget, transform::commands};

pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Screens::default())
            .add_startup_system(setup_scene);
    }
}

#[derive(Resource)]
pub struct Screens {
    images: Vec<Handle<Image>>,
}
impl Default for Screens {
    fn default() -> Self {
        Self { images: vec![] }
    }
}

#[derive(Component)]
pub struct PortalPair;

#[derive(Component)]
struct Screen;

#[derive(Component)]
struct ScreenCamera;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut screens: ResMut<Screens>,
) {
    let screen = asset_server.load("textures/screen.png");
    screens.images.push(screen.clone());

    commands
        .spawn(Name::new("Screen_a"))
        .insert(Camera3dBundle {
            camera: Camera {
                target: RenderTarget::Image(screen.clone()),
                ..default()
            },
            ..default()
        })
        .insert(ScreenCamera);

    // commands
    //     .spawn(Name::new("Screen"))
    //     .insert(MaterialMeshBundle {
    //         mesh: meshes.add(Mesh::from(shape::Quad {
    //             // size: Vec2::new(8.0, 4.5),
    //             size: Vec2::new(1.0, 2.0),
    //             flip: false,
    //         })),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::WHITE,
    //             base_color_texture: Some(screen.clone()),
    //             ..default()
    //         }),
    //         transform: Transform {
    //             translation: vec3(0.0, 0.0, 0.0),
    //             ..default()
    //         },
    //         ..default()
    //     });
}
