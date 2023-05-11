use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

use super::{
    portal_camera::{PortalCamera, PortalCameraBundle},
    portal_screen::PortalScreenBundle,
    texture_binding_array::BindlessMaterial,
};

#[derive(Resource)]
pub struct PortalPairSpawns {
    pub portals: Vec<PortalPairSpawn>,
}
impl PortalPairSpawns {
    fn add_portal(
        &mut self,
        a_pos: Vec3,
        b_pos: Vec3,
        a_res: [u32; 2],
        b_res: [u32; 2],
        a_size: Vec2,
        b_size: Vec2,
        a_quat: Quat,
        b_quat: Quat,
    ) {
        self.portals.push(PortalPairSpawn {
            a_pos,
            b_pos,
            a_res,
            b_res,
            a_size,
            b_size,
            a_quat,
            b_quat,
        });
    }
}
pub struct PortalPairSpawn {
    pub a_pos: Vec3,
    pub b_pos: Vec3,
    pub a_res: [u32; 2],
    pub b_res: [u32; 2],
    pub a_size: Vec2,
    pub b_size: Vec2,
    pub a_quat: Quat,
    pub b_quat: Quat,
}

#[derive(Component)]
pub struct PortalPair;

pub fn create_portals(
    spawnlist: Res<PortalPairSpawns>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bindless_materials: ResMut<Assets<BindlessMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut count = 0;
    for portal_pair in spawnlist.portals.iter() {
        let a_image_size = Extent3d {
            width: portal_pair.a_res[0],
            height: portal_pair.a_res[1],
            ..default()
        };
        let mut a_image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: a_image_size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        a_image.resize(a_image_size);

        let b_image_size = Extent3d {
            width: portal_pair.b_res[0],
            height: portal_pair.b_res[1],
            ..default()
        };
        let mut b_image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: b_image_size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        b_image.resize(b_image_size);

        let image_handle_a = images.add(a_image.clone());
        let image_handle_b = images.add(b_image.clone());

        // let foo = BindlessMaterial {
        //     textures: vec![image_handle_a.clone()],
        // };

        let pair = commands
            .spawn(Name::new(format!("portal_pair_{}", count)))
            .insert(PortalPair)
            .insert(SpatialBundle::default())
            .id();

        let screen_a = commands
            .spawn(PortalScreenBundle::new(
                format!("{}_screen_a", count),
                meshes.add(Mesh::from(shape::Quad {
                    size: portal_pair.a_size,
                    flip: false,
                })),
                materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(image_handle_a.clone()),
                    ..default()
                }),
                portal_pair.a_pos,
            ))
            .id();

        let screen_b = commands
            .spawn(PortalScreenBundle::new(
                "screen_b".into(),
                meshes.add(Mesh::from(shape::Quad {
                    size: portal_pair.b_size,
                    flip: false,
                })),
                materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(image_handle_b.clone()),
                    ..default()
                }),
                portal_pair.b_pos,
            ))
            .id();

        let cam_a = commands
            .spawn(PortalCameraBundle::new(
                "cam_a".into(),
                image_handle_b.clone(),
            ))
            .insert(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                ..default()
            })
            .id();

        let cam_b = commands
            .spawn(PortalCameraBundle::new(
                "cam_b".into(),
                image_handle_a.clone(),
            ))
            .insert(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                ..default()
            })
            .id();

        commands.entity(pair).push_children(&[screen_a, screen_b]);
        commands.entity(screen_a).push_children(&[cam_a]);
        commands.entity(screen_b).push_children(&[cam_b]);

        count += 1;
    }
}
