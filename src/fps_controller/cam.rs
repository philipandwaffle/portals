use super::controls::ControlState;
use crate::global_resources::GlobalResources;
use bevy::prelude::*;

#[derive(Component)]
pub struct FPSCam {
    pub sen: f32,
}
impl Default for FPSCam {
    fn default() -> Self {
        Self { sen: 0.001 }
    }
}

pub fn rotate_player_camera(
    mut cam: Query<(&FPSCam, &mut Transform, &GlobalTransform)>,
    cs: Res<ControlState>,
    mut gr: ResMut<GlobalResources>,
) {
    match cam.get_single_mut() {
        Ok((fps_cam, mut transform, gt)) => {
            let pitch = Quat::from_axis_angle(Vec3::X, cs.look_delta.y * -fps_cam.sen);
            transform.rotate_local(pitch);

            let yaw = Quat::from_axis_angle(Vec3::Y, cs.look_delta.x * -fps_cam.sen);
            transform.rotate(yaw);

            gr.y_rot *= yaw;
            gr.cam_rot = transform.rotation;
            gr.global_player_pos = gt.translation();
        }
        Err(_) => warn!("There is no player camera in the scene"),
    }
}
