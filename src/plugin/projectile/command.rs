use crate::{
    data::LAYER_PROJECTILE,
    model::TransformLite,
    plugin::{projectile::material::ProjectileMaterial, Projectile, ProjectileConfig},
    resource::AssetStorage,
    util::ext::Vec2Ext,
};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    math::Vec3,
    prelude::{Entity, Time, Transform, Vec2, World},
    sprite::MaterialMesh2dBundle,
};

pub struct ProjectileSpawn {
    pub config: &'static ProjectileConfig,
    pub transform: TransformLite,
    pub velocity: f32,
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
            self.transform.translation,
            Vec2::from_length(self.velocity, self.transform.direction),
            self.shooter,
        );

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
