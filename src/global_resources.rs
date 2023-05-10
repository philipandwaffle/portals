use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalResources {
    pub cam_rot: Quat,
    pub y_rot: Quat,
    pub global_player_pos: Vec3,
}
impl Default for GlobalResources {
    fn default() -> Self {
        Self {
            cam_rot: Quat::IDENTITY,
            y_rot: Quat::IDENTITY,
            global_player_pos: Vec3::ZERO,
        }
    }
}
