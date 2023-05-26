use crate::{data::PIXELS_PER_METER, material::StatusBarMaterial, util};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    prelude::{shape::Cube, BuildWorldChildren, Entity, Transform, Vec3, World},
    render::{mesh::Mesh, texture::Image},
    sprite::MaterialMesh2dBundle,
};

pub struct StatusBarSet(pub Entity);

impl Command for StatusBarSet {
    fn write(self, world: &mut World) {
        let image = world
            .resource_mut::<Assets<Image>>()
            .add(util::create_empty_image(1, 1));

        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(Cube::default()));

        let material = world
            .resource_mut::<Assets<StatusBarMaterial>>()
            .add(StatusBarMaterial {
                health: 0.0,
                health_alpha: 0.0,
                ammo: 1.0,
                ammo_alpha: 0.0,
                image,
            });

        let transform = Transform::default().with_scale(Vec3::splat(PIXELS_PER_METER * 1.2));

        world
            .spawn(MaterialMesh2dBundle {
                transform,
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .set_parent(self.0);
    }
}