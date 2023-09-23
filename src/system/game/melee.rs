use crate::{
    command::ActorMeleeReset,
    component::{Actor, ActorConfig, Weapon},
    model::{ActorActionsExt, AudioPlay, TransformLite},
    resource::{AudioTracker, HitResource},
    util::{ext::Vec2Ext, math},
};
use bevy::{
    ecs::entity::Entity,
    math::Vec2Swizzles,
    prelude::{Commands, Query, Res, ResMut, Transform, Vec2, Without},
    time::Time,
};

pub fn melee(
    attackers: Query<(Entity, &Actor, &Transform), Without<Weapon>>,
    targets: Query<(Entity, &Actor, &Transform)>,
    mut hits: ResMut<HitResource>,
    mut audio: ResMut<AudioTracker>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (attacker_entity, attacker_actor, attacker_transform) in attackers.iter() {
        if !attacker_actor.actions.is_attacking() {
            continue;
        }

        if attacker_actor.melee_next > time {
            continue;
        }

        let attacker_transform = TransformLite::from(attacker_transform);
        let mut victim: Option<TargetData> = None;

        for (target_entity, target_actor, target_transform) in targets.iter() {
            if attacker_actor.config.kind == target_actor.config.kind {
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
            let momentum = attacker_actor.config.melee_damage * attacker_actor.skill;
            let force = Vec2::from_length(momentum, victim.angle_objective);
            hits.add(victim.entity, force, -victim.angle_subjective);

            audio.queue(AudioPlay {
                path: "sounds/melee".into(),
                volume: 0.6,
                source: Some(attacker_transform.translation.xy()),
                ..AudioPlay::DEFAULT
            });

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
    let distance_to_hit = attacker.melee_distance + target.radius;

    if relative.is_longer_than(distance_to_hit) {
        return None;
    }

    let angle_objective = relative.angle();
    let angle_subjective = math::angle_difference(angle_objective, attacker_transform.direction);
    let distance_angular = angle_subjective.abs() / (attacker.melee_distance_angular / 2.0);

    if distance_angular > 1.0 {
        return None;
    }

    let distance = relative.length() / distance_to_hit;

    return Some(TargetData {
        entity: target_entity,
        distance: distance * distance_angular,
        angle_objective,
        angle_subjective,
    });
}
