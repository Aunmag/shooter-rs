use crate::{data::PIXELS_PER_METER, material::StatusBarMaterial, resource::AssetStorage};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    prelude::{BuildWorldChildren, Entity, Transform, Vec3, World},
    sprite::MaterialMesh2dBundle,
};

pub struct StatusBarSet(pub Entity);

impl Command for StatusBarSet {
    fn apply(self, world: &mut World) {
        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();
        let material = world
            .resource_mut::<Assets<StatusBarMaterial>>()
            .add(StatusBarMaterial {
                health: 0.0,
                health_alpha: 0.0,
                ammo: 1.0,
                ammo_alpha: 0.0,
                stamina: 0.0,
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
