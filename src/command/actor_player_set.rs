use crate::{
    command::{LaserSightSet, StatusBarSet},
    component::{Actor, Health, Player},
    data::{LAYER_ACTOR_PLAYER, LAYER_CROSSHAIR},
    material::CrosshairMaterial,
    plugin::CameraTarget,
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
        let health_multiplier = 1.0 / world.resource::<Config>().game.difficulty;

        if let Some(mut actor) = world.get_mut::<Actor>(self.entity) {
            actor.skill = 1.0; // to keep game balance well, player skill must always be 1.0
        }

        if let Some(mut health) = world.get_mut::<Health>(self.entity) {
            health.multiply(health_multiplier);
        }

        if let Some(mut transform) = world.get_mut::<Transform>(self.entity) {
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        let crosshair = spawn_crosshair(world);

        // TODO: don't insert player if it isn't controllable
        world
            .entity_mut(self.entity)
            .insert(Player::new(self.is_controllable, crosshair))
            .insert(CameraTarget::default());

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
