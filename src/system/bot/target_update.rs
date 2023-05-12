use crate::component::Actor;
use crate::component::Bot;
use crate::component::Inertia;
use crate::model::geometry::GeometryProjection;
use crate::model::geometry::Line;
use crate::util;
use crate::util::ext::Vec2Ext;
use crate::util::Timer;
use bevy::ecs::system::Res;
use bevy::ecs::system::Resource;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Entity;
use bevy::prelude::Query;
use bevy::prelude::ResMut;
use bevy::prelude::Transform;
use bevy::prelude::Vec2;
use bevy::prelude::With;
use bevy::time::Time;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::time::Duration;

const RUN_INTERVAL: Duration = Duration::from_millis(200);
const DETOUR_DISTANCE_MIN: f32 = 2.0;

#[derive(Resource)]
pub struct TargetUpdateData {
    timer: Timer,
}

impl Default for TargetUpdateData {
    fn default() -> Self {
        return Self {
            timer: Timer::new(RUN_INTERVAL),
        };
    }
}

pub fn target_update(
    mut bots: Query<(Entity, &mut Bot, &Transform, &Inertia)>,
    actors: Query<(&Transform, &Inertia), With<Actor>>,
    mut data: ResMut<TargetUpdateData>,
    time: Res<Time>,
) {
    if !data.timer.next_if_done(time.elapsed()) {
        return;
    }

    for (entity, mut bot, bot_transform, bot_inertia) in bots.iter_mut() {
        if let Some((target_position, target_velocity)) = bot
            .target_actor
            .and_then(|e| actors.get(e).ok())
            .map(|a| (a.0.translation.xy(), a.1.velocity))
        {
            let bot_position = bot_transform.translation.xy();
            let mut rng = Pcg32::seed_from_u64(u64::from(entity.index()));
            let mut target_final = util::math::find_meet_point(
                bot_position,
                bot_inertia.velocity,
                target_position,
                target_velocity,
            );

            if rng.gen::<bool>() {
                let detour_point = generate_detour_point(bot_position, target_final, &mut rng);

                if (detour_point - bot_position).is_longer_than(DETOUR_DISTANCE_MIN) {
                    target_final = detour_point;
                }
            }

            bot.target_final = Some(target_final);
        } else {
            bot.target_final = None;
        }
    }
}

fn generate_detour_point(origin: Vec2, target: Vec2, rng: &mut Pcg32) -> Vec2 {
    let detour_line = Line::new(
        target,
        Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize(),
    );

    return origin.project_on(&detour_line);
}
