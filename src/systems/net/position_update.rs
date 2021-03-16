use crate::components::Interpolation;
use crate::components::Player;
use crate::resources::EntityMap;
use crate::resources::PositionUpdateResource;
use crate::utils;
use crate::utils::Position;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::Read;
use amethyst::ecs::ReadExpect;
use amethyst::ecs::Write;

const MAX_PLAYER_OFFSET: f32 = 0.25;

#[derive(SystemDesc)]
pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadExpect<'a, EntityMap>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Write<'a, PositionUpdateResource>,
        WriteStorage<'a, Interpolation>,
    );

    fn run(
        &mut self,
        (entities, time, entity_map, players, transforms, mut updates, mut interpolations): Self::SystemData,
    ) {
        if updates.is_empty() {
            return;
        }

        let mut ghost_update: Option<(Entity, &Position)> = None;
        let now = time.absolute_time();

        for (entity, transform, interpolation, player) in (
            &entities,
            &transforms,
            &mut interpolations,
            (&players).maybe(),
        )
            .join()
        {
            let update = entity_map
                .get_external_id(entity)
                .and_then(|id| updates.get(&id))
                .or_else(|| {
                    if let Some((ghost, update)) = ghost_update {
                        if ghost == entity {
                            return Some(update);
                        }
                    }

                    return None;
                });

            if let Some(update) = update {
                let fix_position;

                if let Some(player) = player {
                    fix_position = is_offset_noticeable(transform, update);

                    if let Some(ghost) = player.ghost {
                        ghost_update.replace((ghost, &update));
                    }
                } else {
                    fix_position = true;
                }

                if fix_position {
                    interpolation.next(*update, now);
                }
            }
        }

        updates.clear();
    }
}

fn is_offset_noticeable(transform: &Transform, update: &Position) -> bool {
    let offset_x = update.x - transform.translation().x;
    let offset_y = update.y - transform.translation().y;
    return !utils::math::are_closer_than(offset_x, offset_y, 0.0, 0.0, MAX_PLAYER_OFFSET);
}
