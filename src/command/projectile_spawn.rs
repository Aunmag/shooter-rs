use crate::component::Projectile;
use crate::component::ProjectileConfig;
use crate::data::LAYER_PROJECTILE;
use crate::material::ProjectileMaterial;
use crate::model::Position;
use crate::resource::Message;
use crate::resource::NetResource;
use crate::util;
use crate::util::ext::WorldExt;
use bevy::asset::Assets;
use bevy::ecs::system::Command;
use bevy::math::Vec3;
use bevy::prelude::shape::Cube;
use bevy::prelude::Entity;
use bevy::prelude::Image;
use bevy::prelude::Mesh;
use bevy::prelude::Time;
use bevy::prelude::Transform;
use bevy::prelude::Vec2;
use bevy::prelude::World;
use bevy::sprite::MaterialMesh2dBundle;

pub struct ProjectileSpawn {
    pub position: Position,
    pub velocity: f32,
    pub acceleration_factor: f32,
    pub shooter: Option<Entity>,
}

impl Command for ProjectileSpawn {
    fn write(self, world: &mut World) {
        if world.is_server() {
            world
                .resource_mut::<NetResource>()
                .send_to_all(Message::ProjectileSpawn {
                    id: 0,
                    position: self.position,
                    velocity: self.velocity,
                    acceleration_factor: self.acceleration_factor,
                    shooter_id: self.shooter.map(Entity::id),
                });
        }

        let (sin, cos) = (-self.position.direction).sin_cos();
        let projectile = Projectile::new(
            ProjectileConfig {
                acceleration_factor: self.acceleration_factor,
            },
            world.resource::<Time>().time_since_startup(),
            Vec2::new(self.position.x, self.position.y),
            Vec2::new(self.velocity * sin, self.velocity * cos),
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
            .spawn()
            .insert_bundle(MaterialMesh2dBundle {
                transform: Transform {
                    translation: Vec3::new(self.position.x, self.position.y, LAYER_PROJECTILE),
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
