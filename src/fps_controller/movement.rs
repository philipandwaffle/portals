use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;

use super::{controls::ControlState, Player};

pub fn move_player_down(mut controllers: Query<&mut KinematicCharacterController, With<Player>>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(0.0, -0.0001, 0.0));
    }
}

pub fn move_player(
    mut player: Query<(
        &Player,
        &mut Velocity,
        &Transform,
        &KinematicCharacterControllerOutput,
    )>,
    mut cs: ResMut<ControlState>,
) {
    match player.get_single_mut() {
        Ok((player, mut vel, trans, output)) => {
            if output.grounded && cs.jump {
                vel.linvel.y += player.jump_force;
                cs.jump = false;
            }

            let delta_v = trans.rotation.mul_vec3(cs.move_dir.extend(0.0).xzy()) * player.speed;
            if vel.linvel.length_squared() < player.max_speed * player.max_speed {
                vel.linvel += delta_v;
            }
            cs.move_dir = Vec2::new(0.0, 0.0);
        }
        Err(err) => println!("Error getting player, {:?}", err),
    }
}
