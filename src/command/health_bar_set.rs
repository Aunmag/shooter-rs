use crate::{component::HealthBar, data::PIXELS_PER_METER, material::HealthBarMaterial, util};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    prelude::{shape::Cube, BuildWorldChildren, Color, Entity, Transform, Vec3, World},
    render::{mesh::Mesh, texture::Image},
    sprite::MaterialMesh2dBundle,
};

pub struct HealthBarSet(pub Entity);

impl Command for HealthBarSet {
    fn write(self, world: &mut World) {
        let image = world
            .resource_mut::<Assets<Image>>()
            .add(util::create_empty_image(1, 1));

        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(Cube::default()));

        let material = world
            .resource_mut::<Assets<HealthBarMaterial>>()
            .add(HealthBarMaterial {
                value: 1.0,
                color: Color::RED,
                thickness: 0.05,
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
            .insert(HealthBar::default())
            .set_parent(self.0);
    }
}
