use std::vec;

use super::{portal_pair::PortalPair, portal_screen::PortalScreen};
use crate::global_resources::GlobalResources;
use bevy::{
    ecs::query::QueryEntityError, prelude::*, reflect::erased_serde::__private::serde::ser::Error,
    render::camera::RenderTarget,
};
use bevy_inspector_egui::egui::vec2;

#[derive(Bundle)]
pub struct PortalCameraBundle {
    pub name: Name,
    pub portal_cam: PortalCamera,
    pub cam_bundle: Camera3dBundle,
}
impl PortalCameraBundle {
    pub fn new(name: String, target_img: Handle<Image>) -> Self {
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
    pairs: Query<(&Name, &Children), With<PortalPair>>,
    screens: Query<(&GlobalTransform, &Children), With<PortalScreen>>,
    mut cams: Query<&mut Transform, With<PortalCamera>>,
    gr: Res<GlobalResources>,
) {
    let player_pos = gr.global_player_pos;
    for (name, pair_children) in pairs.iter() {
        let mut screen_pos = vec![];
        let mut cam_entities = vec![];

        for &screen in pair_children.iter() {
            let (screen_gt, screen_children) = screens.get(screen).unwrap();

            screen_pos.push(screen_gt.translation().clone());
            cam_entities.push(screen_children[0]);
        }

        let i = screen_pos[0];
        let j = screen_pos[1];

        cams.get_mut(cam_entities[0]).unwrap().translation = gr.global_player_pos;
        cams.get_mut(cam_entities[0]).unwrap().translation = i + j - gr.global_player_pos;

        // println!("{},{}", screen_pos.len(), cam_entities.len());
    }
}

fn from_pair_get_screen_cam(
    entity: Entity,
    screen_children: &Query<&Children, With<PortalScreen>>,
    cam_children: &Query<&Children, With<PortalCamera>>,
) -> Result<([Entity; 2], [Entity; 2]), QueryEntityError> {
    let mut screens = vec![];
    let mut cams = vec![];

    for screen in screen_children.iter_descendants(entity) {
        screens.push(screen);
        for cam in cam_children.iter_descendants(screen) {
            cams.push(cam);
        }
    }
    println!("{},{}", screens.len(), cams.len());
    if screens.len() != 2 || cams.len() != 2 {
        return Err(QueryEntityError::QueryDoesNotMatch(entity));
    }

    return Ok(([screens[0], screens[1]], [cams[0], cams[1]]));
}
