use crate::components::Interpolation;
use crate::components::Player;
use crate::resources::EntityMap;
use crate::resources::PositionUpdate;
use crate::resources::PositionUpdateResource;
use crate::utils;
use amethyst::core::transform::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::Join;
use amethyst::ecs::prelude::ReadStorage;
use amethyst::ecs::prelude::System;
use amethyst::ecs::prelude::SystemData;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::ReadExpect;
use amethyst::ecs::Write;

const MAX_PLAYER_OFFSET: f32 = 0.25;

#[derive(SystemDesc)]
pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, EntityMap>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Write<'a, PositionUpdateResource>,
        WriteStorage<'a, Interpolation>,
    );

    fn run(
        &mut self,
        (entities, entity_map, players, transforms, mut updates, mut interpolations): Self::SystemData,
    ) {
        if updates.is_empty() {
            return;
        }

        let query = (&entities, &transforms, &mut interpolations, (&players).maybe()).join();
        let mut ghost_update: Option<(Entity, &PositionUpdate)> = None;

        for (entity, transform, interpolation, player) in query {
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
                let offset_x = update.x - transform.translation().x;
                let offset_y = update.y - transform.translation().y;

                if let Some(player) = player {
                    fix_position = is_offset_noticeable(offset_x, offset_y);

                    if let Some(ghost) = player.ghost {
                        ghost_update.replace((ghost, &update));
                    }
                } else {
                    fix_position = true;

                    interpolation.offset_direction = utils::math::angle_difference(
                        update.direction,
                        transform.euler_angles().2,
                    );
                }

                if fix_position {
                    interpolation.offset_x = offset_x;
                    interpolation.offset_y = offset_y;
                }
            }
        }

        updates.clear();
    }
}

fn is_offset_noticeable(offset_x: f32, offset_y: f32) -> bool {
    return !utils::math::are_closer_than(offset_x, offset_y, 0.0, 0.0, MAX_PLAYER_OFFSET);
}
