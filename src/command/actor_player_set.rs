use super::LaserSightSet;
use crate::{
    command::StatusBarSet,
    component::Player,
    data::{LAYER_ACTOR_PLAYER, LAYER_CROSSHAIR},
    material::CrosshairMaterial,
    resource::{AssetStorage, Config, GameMode},
};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    math::Vec3,
    prelude::{Entity, Transform, World},
    sprite::MaterialMesh2dBundle,
};

pub struct ActorPlayerSet {
    pub entity: Entity,
    pub is_controllable: bool,
}

impl Command for ActorPlayerSet {
    fn apply(self, world: &mut World) {
        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        let crosshair = spawn_crosshair(world);

        world
            .entity_mut(self.entity)
            .insert(Player::new(self.is_controllable, crosshair));

        StatusBarSet(self.entity).apply(world);

        if world
            .resource::<Config>()
            .game
            .modes
            .contains(&GameMode::LaserSight)
        {
            LaserSightSet(self.entity).apply(world);
        }
    }
}

fn spawn_crosshair(world: &mut World) -> Entity {
    let assets = world.resource::<AssetStorage>();
    let image = assets.dummy_image().clone();
    let mesh = assets.dummy_mesh().clone();
    let material = world
        .resource_mut::<Assets<CrosshairMaterial>>()
        .add(CrosshairMaterial { image });

    return world
        .spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, LAYER_CROSSHAIR),
                ..Transform::default()
            },
            mesh: mesh.into(),
            material,
            ..Default::default()
        })
        .id();
}
