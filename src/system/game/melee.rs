use crate::command::ActorMeleeReset;
use crate::command::AudioPlay;
use crate::component::Actor;
use crate::component::ActorConfig;
use crate::component::Health;
use crate::component::Inertia;
use crate::component::Weapon;
use crate::model::ActorActionsExt;
use crate::model::TransformLite;
use crate::util::ext::Vec2Ext;
use crate::util::math;
use bevy::ecs::entity::Entity;
use bevy::math::Vec2Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::Vec2;
use bevy::prelude::Without;
use bevy::time::Time;

pub fn melee(
    attackers: Query<(Entity, &Actor, &Transform), Without<Weapon>>,
    mut targets: Query<(Entity, &Actor, &Transform, &mut Inertia, &mut Health)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (attacker_entity, attacker_actor, attacker_transform) in attackers.iter() {
        let attacker_transform = TransformLite::from(attacker_transform);

        if !attacker_actor.actions.is_attacking() {
            continue;
        }

        if attacker_actor.melee_next > time {
            continue;
        }

        let mut victim: Option<TargetData> = None;

        for (target_entity, target_actor, target_transform, _, _) in targets.iter() {
            if attacker_actor.config.actor_type == target_actor.config.actor_type {
                continue;
            }

            if let Some(target_data) = calc_target_data(
                attacker_actor.config,
                &attacker_transform,
                target_actor.config,
                &TransformLite::from(target_transform),
                target_entity,
            ) {
                if victim
                    .as_ref()
                    .map_or(true, |v| v.distance > target_data.distance)
                {
                    victim = Some(target_data);
                }
            }
        }

        if let Some(victim) = victim {
            if let Ok((_, _, _, mut victim_inertia, mut victim_health)) =
                targets.get_mut(victim.entity)
            {
                let momentum = attacker_actor.config.melee_damage;
                let force = Vec2::from_length(momentum, victim.angle_objective);

                victim_inertia.push(
                    force,
                    momentum * -victim.angle_subjective,
                    true,
                    false,
                    true,
                );

                commands.add(AudioPlay {
                    path: "sounds/melee_{n}.ogg",
                    volume: 0.6,
                    source: Some(attacker_transform.translation.xy()),
                    ..AudioPlay::DEFAULT
                });

                victim_health.damage(momentum, time);
            }
        }

        commands.add(ActorMeleeReset(attacker_entity));
    }
}

struct TargetData {
    entity: Entity,
    distance: f32,
    angle_objective: f32,
    angle_subjective: f32,
}

fn calc_target_data(
    attacker: &ActorConfig,
    attacker_transform: &TransformLite,
    target: &ActorConfig,
    target_transform: &TransformLite,
    target_entity: Entity,
) -> Option<TargetData> {
    let relative = target_transform.translation - attacker_transform.translation;
    let distance = (relative.length() - target.radius) / attacker.melee_distance;

    if distance > 1.0 {
        return None;
    }

    let angle_objective = relative.angle();
    let angle_subjective = math::angle_difference(angle_objective, attacker_transform.direction);
    let distance_angular = angle_subjective.abs() / (attacker.melee_distance_angular / 2.0);

    if distance_angular > 1.0 {
        return None;
    }

    return Some(TargetData {
        entity: target_entity,
        distance: distance * distance_angular,
        angle_objective,
        angle_subjective,
    });
}
