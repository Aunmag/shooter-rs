use crate::command::ActorSet;
use crate::command::HealthBarSet;
use crate::component::Actor;
use crate::component::ActorConfig;
use crate::component::Inertia;
use crate::component::Player;
use crate::data::LAYER_ACTOR_PLAYER;
use crate::model::TransformLiteU8;
use crate::util::ext::WorldExt;
use bevy::ecs::system::Command;
use bevy::prelude::Entity;
use bevy::prelude::Transform;
use bevy::prelude::World;

pub struct ActorPlayerSet(pub Entity);

impl Command for ActorPlayerSet {
    fn write(self, world: &mut World) {
        let is_client = world.is_client();
        let mut ghost_transform = TransformLiteU8::default();

        if let Some(mut transform) = world.get_mut::<Transform>(self.0) {
            ghost_transform = TransformLiteU8::from(&*transform);
            transform.translation.z = LAYER_ACTOR_PLAYER;
        }

        let config = world
            .get::<Actor>(self.0)
            .map(|a| a.config)
            .unwrap_or_else(|| {
                log::warn!("Couldn't find actor component");
                return ActorConfig::HUMAN;
            });

        let mut ghost = None;

        if is_client && world.config().misc.show_ghost {
            let entity = world.spawn_empty().id();

            ActorSet {
                entity,
                config,
                transform: ghost_transform,
                is_ghost: true,
            }
            .write(world);

            ghost = Some(entity);
        }

        let mut entity_mut = world.entity_mut(self.0);
        entity_mut.insert(Player::new(ghost));
        entity_mut.insert(Inertia::new(config.mass));

        HealthBarSet(self.0).write(world);
    }
}
