use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use fps_controller::FPSPlugin;
use global_resources::GlobalResources;
use portal::TestPlugin;

mod fps_controller;
mod global_resources;
mod portal;

fn main() {
    App::new()
        .insert_resource(GlobalResources::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "FPS Cam".into(),
                        resolution: (1900., 1280.).into(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::BorderlessFullscreen,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                // don't use linear sampling as image textures will be blurry
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(FPSPlugin)
        .add_plugin(TestPlugin)
        .add_startup_system(init_scene)
        .run();
}

fn init_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    // ground
    commands
        .spawn(Name::new("Ground"))
        .insert(Collider::cuboid(20.0, 0.1, 20.0))
        .insert(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(40.0, 0.2, 40.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                base_color_texture: Some(asset_server.load("textures/checker_board.png")),
                ..default()
            }),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    // LET THERE BE LIGHT
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });
}
