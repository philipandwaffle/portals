use crate::global_resources::GlobalResources;
use bevy::prelude::*;

use super::portal_pair::ScreenCamera;

pub fn rotate_portal_cams(
    mut portal_cams: Query<(&ScreenCamera, &mut Transform)>,
    gr: Res<GlobalResources>,
) {
    for (_, mut transform) in portal_cams.iter_mut() {
        transform.rotation = gr.cam_rot;
    }
}
