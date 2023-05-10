use std::default;

use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalResources {
    pub cam_rot: Quat,
    pub y_rot: Quat,
}
impl Default for GlobalResources {
    fn default() -> Self {
        Self {
            cam_rot: Quat::IDENTITY,
            y_rot: Quat::IDENTITY,
        }
    }
}
