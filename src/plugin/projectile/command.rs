use crate::{
    data::LAYER_PROJECTILE,
    plugin::{projectile::material::ProjectileMaterial, Projectile, ProjectileConfig},
    resource::AssetStorage,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::Assets,
    ecs::world::Command,
    math::Vec3,
    prelude::{Entity, Time, Transform, Vec2, World},
    sprite::MaterialMesh2dBundle,
};

pub struct ProjectileSpawn {
    pub config: &'static ProjectileConfig,
    // TODO: store spawn time here for better accuracy?
    pub position: Vec2,
    pub velocity: Vec2,
    pub shooter: Option<Entity>,
}

impl Command for ProjectileSpawn {
    fn apply(self, world: &mut World) {
        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();

        let projectile = Projectile::new(
            self.config,
            world.resource::<Time>().elapsed(),
            self.position,
            self.velocity,
            self.shooter,
        );

        let material = world
            .resource_mut::<Assets<ProjectileMaterial>>()
            .add(ProjectileMaterial { image });

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: self.position.extend(LAYER_PROJECTILE),
                    rotation: self.velocity.as_quat(),
                    scale: Vec3::new(0.0, 0.0, 1.0),
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .insert(projectile);
    }
}
