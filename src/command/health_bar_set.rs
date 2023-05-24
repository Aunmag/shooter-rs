use crate::component::HealthBar;
use crate::data::PIXELS_PER_METER;
use crate::material::HealthBarMaterial;
use crate::util;
use bevy::asset::Assets;
use bevy::ecs::system::Command;
use bevy::prelude::shape::Cube;
use bevy::prelude::BuildWorldChildren;
use bevy::prelude::Color;
use bevy::prelude::Entity;
use bevy::prelude::Transform;
use bevy::prelude::Vec3;
use bevy::prelude::World;
use bevy::render::mesh::Mesh;
use bevy::render::texture::Image;
use bevy::sprite::MaterialMesh2dBundle;

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
