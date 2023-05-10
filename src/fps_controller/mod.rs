use bevy::{math::vec3, prelude::*};
use bevy_rapier3d::prelude::*;

use self::{
    cam::{rotate_player_camera, FPSCam},
    controls::ControlPlugin,
    movement::{move_player, move_player_down},
};
mod cam;
mod controls;
mod movement;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub max_speed: f32,
    pub jump_force: f32,
    pub sen: f32,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 0.5,
            max_speed: 3.0,
            jump_force: 3.0,
            sen: 0.001,
        }
    }
}

pub struct FPSPlugin;
impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ControlPlugin);
        app.add_startup_system(spawn_player);

        // Tried to schedule system before to avoid having to do checks but didn't work
        // app.add_system(move_player_down.after(move_player));
        app.add_system(move_player_down);
        app.add_system(rotate_player_camera);
        app.add_system(move_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let spawn = TransformBundle::from(Transform::from_xyz(0.0, 3.0, 0.0));
    commands
        .spawn(Name::new("Player"))
        .insert(Player::default())
        .insert(
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Z
                | LockedAxes::ROTATION_LOCKED_Y,
        )
        .insert(Collider::capsule_y(0.5, 0.25))
        .insert(KinematicCharacterController {
            offset: CharacterLength::Relative(0.01),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale::default())
        .insert(Velocity::default())
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(MaterialMeshBundle {
            mesh: meshes.add(
                shape::Capsule {
                    radius: 0.25,
                    rings: 0,
                    depth: 1.0,
                    latitudes: 16,
                    longitudes: 32,
                    uv_profile: shape::CapsuleUvProfile::Aspect,
                }
                .into(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            ..default()
        })
        .insert(spawn)
        .with_children(|parent| {
            parent
                .spawn(Name::new("FPSCamera"))
                .insert(Camera3dBundle {
                    transform: Transform {
                        translation: vec3(0.0, 0.5, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(FPSCam::default());
        });
}
