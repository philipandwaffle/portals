use crate::global_resources::GlobalResources;
use bevy::{prelude::*, render::camera::RenderTarget};

use super::portal_pair::PortalScreen;

#[derive(Bundle)]
pub struct PortalCameraBundle {
    pub name: Name,
    pub portal_cam: PortalCamera,
    pub cam_bundle: Camera3dBundle,
}
impl PortalCameraBundle {
    fn new(name: String, target_img: Handle<Image>) -> Self {
        Self {
            name: Name::new(name),
            portal_cam: PortalCamera,
            cam_bundle: Camera3dBundle {
                camera: Camera {
                    target: RenderTarget::Image(target_img.clone()),
                    ..default()
                },
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct PortalCamera;

pub fn rotate_portal_cams(
    mut portal_cams: Query<&mut Transform, With<PortalCamera>>,
    gr: Res<GlobalResources>,
) {
    for mut transform in portal_cams.iter_mut() {
        transform.rotation = gr.cam_rot;
    }
}

pub fn translate_portal_cams(
    screens: Query<&GlobalTransform, (With<PortalScreen>, Without<PortalCamera>)>,
    mut cams: Query<(&Parent, &mut Transform), (With<PortalCamera>, Without<PortalScreen>)>,
    gr: Res<GlobalResources>,
) {
    for (cam_parent, mut cam_transform) in cams.iter_mut() {
        // `parent` contains the Entity ID we can use
        // to query components from the parent:
        match screens.get(cam_parent.get()) {
            Ok(screen_gt) => {
                cam_transform.translation = screen_gt.translation() - gr.global_player_pos;
            }
            Err(err) => warn!("One of the portal screens doesn't have a camera, {:?}", err),
        };
        // if let Ok(screen_gt) = screens.get(cam_parent.get()) {
        //     cam_transform.translation = screen_gt.translation() - gr.global_player_pos;
        // }
    }
}
