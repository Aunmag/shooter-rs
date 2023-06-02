use crate::{
    command::{ActorMeleeReset, AudioPlay},
    component::{Actor, ActorConfig, Health, Inertia, Player, Weapon},
    model::{ActorActionsExt, TransformLite},
    util::{ext::Vec2Ext, math},
};
use bevy::{
    ecs::entity::Entity,
    math::Vec2Swizzles,
    prelude::{Commands, Query, Res, Transform, Vec2, Without},
    time::Time,
};

pub fn melee(
    attackers: Query<(Entity, &Actor, &Transform), Without<Weapon>>,
    mut targets: Query<(
        Entity,
        &Actor,
        &Transform,
        &mut Inertia,
        &mut Health,
        Option<&mut Player>,
    )>,
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

        for (target_entity, target_actor, target_transform, _, _, _) in targets.iter() {
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
            if let Ok((_, _, _, mut victim_inertia, mut victim_health, mut player)) =
                targets.get_mut(victim.entity)
            {
                let momentum = attacker_actor.config.melee_damage * attacker_actor.skill;
                let force = Vec2::from_length(momentum, victim.angle_objective);
                let force_angular = momentum * -victim.angle_subjective;

                victim_inertia.push(force, force_angular, true, false, true);

                if let Some(player) = player.as_mut() {
                    player.shake(force_angular * Inertia::PUSH_MULTIPLIER_ANGULAR);
                }

                commands.add(AudioPlay {
                    path: "sounds/melee_{n}.ogg",
                    volume: 0.6,
                    source: Some(attacker_transform.translation.xy()),
                    priority: AudioPlay::PRIORITY_LOWER,
                    ..AudioPlay::DEFAULT
                });

                victim_health.damage(momentum);
            }

            commands.add(ActorMeleeReset(attacker_entity));
        }
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
