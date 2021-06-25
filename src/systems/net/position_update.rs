use crate::components::Interpolation;
use crate::components::Player;
use crate::resources::PositionUpdateResource;
use crate::utils;
use crate::utils::Position;
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::Entities;
use amethyst::ecs::Entity;
use amethyst::ecs::Join;
use amethyst::ecs::Read;
use amethyst::ecs::ReadStorage;
use amethyst::ecs::System;
use amethyst::ecs::Write;
use amethyst::ecs::WriteStorage;

const MAX_PLAYER_OFFSET: f32 = 0.25;

pub struct PositionUpdateSystem;

impl<'a> System<'a> for PositionUpdateSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        Write<'a, PositionUpdateResource>,
        WriteStorage<'a, Interpolation>,
    );

    fn run(
        &mut self,
        (entities, time, players, transforms, mut updates, mut interpolations): Self::SystemData,
    ) {
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
            let update = updates.get(&entity.id()).or_else(|| {
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
                        ghost_update.replace((ghost, update));
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
