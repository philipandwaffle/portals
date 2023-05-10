use bevy::{
    math::vec3,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

use super::texture_binding_array::BindlessMaterial;

#[derive(Resource)]
pub struct PortalPairs {
    pub portals: Vec<PortalPair>,
}
impl PortalPairs {
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
        self.portals.push(PortalPair {
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
pub struct PortalPair {
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
pub struct Portal;
#[derive(Component)]
pub struct Screen;

#[derive(Component)]
pub struct ScreenCamera;

pub fn create_portals(
    spawnlist: Res<PortalPairs>,
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

        let foo = BindlessMaterial {
            textures: vec![image_handle_a.clone()],
        };
        commands
            .spawn(Name::new(format!("{}_screen_a", count)))
            .insert(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: portal_pair.a_size,
                    flip: false,
                })),
                // material: bindless_materials.add(foo),
                // transform: Transform {
                //     translation: portal.a_pos,
                //     ..default()
                // },
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(image_handle_a.clone()),
                    ..default()
                }),
                transform: Transform {
                    translation: portal_pair.a_pos,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(Name::new("Camera"))
                    .insert(Camera3dBundle {
                        camera: Camera {
                            target: RenderTarget::Image(image_handle_b.clone()),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ScreenCamera)
                    .insert(MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::RED,
                            ..default()
                        }),
                        ..default()
                    });
            });

        commands
            .spawn(Name::new(format!("{}_screen_b", count)))
            .insert(MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: portal_pair.b_size,
                    flip: false,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    base_color_texture: Some(image_handle_b.clone()),
                    ..default()
                }),
                transform: Transform {
                    translation: portal_pair.b_pos,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn(Name::new("Camera"))
                    .insert(Camera3dBundle {
                        camera: Camera {
                            target: RenderTarget::Image(image_handle_a.clone()),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(ScreenCamera)
                    .insert(MaterialMeshBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::RED,
                            ..default()
                        }),
                        ..default()
                    });
            });

        count += 1;
    }
}
