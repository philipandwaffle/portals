use bevy::prelude::*;

use super::{controls::ControlState, Player};

#[derive(Component)]
pub struct FPSCam {
    pub sen: f32,
}
impl Default for FPSCam {
    fn default() -> Self {
        Self { sen: 0.01 }
    }
}

pub fn pitch_camera(mut cams: Query<(&FPSCam, &mut Transform)>, mut cs: ResMut<ControlState>) {
    for (fps_cam, mut transform) in cams.iter_mut() {
        let z_rot = Quat::IDENTITY.slerp(
            Quat::from_axis_angle(Vec3::X, cs.look_dir.y * -fps_cam.sen),
            0.1,
        );
        transform.rotate(z_rot);
    }
    cs.look_dir.y = 0.0;
}

pub fn yaw_player(mut player: Query<(&Player, &mut Transform)>, mut cs: ResMut<ControlState>) {
    for (player, mut transform) in player.iter_mut() {
        let y_rot = Quat::IDENTITY.slerp(
            Quat::from_axis_angle(Vec3::Y, cs.look_dir.x * -player.sen),
            0.1,
        );
        transform.rotate(y_rot);
    }
    cs.look_dir.x = 0.0;
}
