use crate::{
    component::{Projectile, ProjectileConfig},
    data::LAYER_PROJECTILE,
    material::ProjectileMaterial,
    model::TransformLite,
    resource::{Message, NetResource},
    util,
    util::ext::{Vec2Ext, WorldExt},
};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    math::Vec3,
    prelude::{shape::Cube, Entity, Image, Mesh, Time, Transform, Vec2, World},
    sprite::MaterialMesh2dBundle,
};

pub struct ProjectileSpawn {
    pub config: &'static ProjectileConfig,
    pub transform: TransformLite,
    pub velocity: f32,
    pub shooter: Option<Entity>,
}

impl Command for ProjectileSpawn {
    fn write(self, world: &mut World) {
        if world.is_server() {
            world
                .resource_mut::<NetResource>()
                .send_to_all(Message::ProjectileSpawn {
                    id: 0,
                    transform: self.transform,
                    velocity: self.velocity,
                    acceleration_factor: self.config.acceleration_factor,
                    shooter_id: self.shooter.map(Entity::index),
                });
        }

        let projectile = Projectile::new(
            self.config,
            world.resource::<Time>().elapsed(),
            self.transform.translation,
            Vec2::from_length(self.velocity, self.transform.direction),
            self.shooter,
        );

        let image = world
            .resource_mut::<Assets<Image>>()
            .add(util::create_empty_image(1, 1));

        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(Cube::default()));

        let material = world
            .resource_mut::<Assets<ProjectileMaterial>>()
            .add(ProjectileMaterial { image });

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: self.transform.translation.extend(LAYER_PROJECTILE),
                    scale: Vec3::new(0.0, 0.0, 1.0),
                    ..Transform::default()
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .insert(projectile);
    }
}
