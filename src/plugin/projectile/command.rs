use crate::{
    data::LAYER_PROJECTILE,
    plugin::{
        projectile::material::ProjectileMaterial, Projectile, ProjectileConfig, ProjectilePhysics,
    },
    resource::AssetStorage,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    math::Vec3,
    mesh::Mesh2d,
    prelude::{Entity, Time, Transform, Vec2, World},
    sprite_render::MeshMaterial2d,
};

pub struct ProjectileSpawn {
    pub config: &'static ProjectileConfig,
    pub position: Vec2,
    pub velocity: Vec2,
    pub distance_limit: f32,
    pub shooter: Option<Entity>,
}

impl Command for ProjectileSpawn {
    type Out = ();

    fn apply(self, world: &mut World) {
        let projectile = Projectile::new(
            self.config,
            world.resource::<Time>().elapsed(),
            self.position,
            self.velocity,
            self.distance_limit,
            self.shooter,
        );

        let transform = Transform {
            translation: self.position.extend(LAYER_PROJECTILE),
            rotation: self.velocity.as_quat(),
            scale: Vec3::new(0.0, 0.0, 1.0),
        };

        if self.config.physics == ProjectilePhysics::Grenade {
            world.spawn((projectile, transform));
        } else {
            let mesh = world.resource::<AssetStorage>().dummy_mesh().clone();

            let material = world
                .resource_mut::<Assets<ProjectileMaterial>>()
                .add(ProjectileMaterial {});

            world.spawn((
                projectile,
                transform,
                Mesh2d(mesh),
                MeshMaterial2d(material),
            ));
        }
    }
}
