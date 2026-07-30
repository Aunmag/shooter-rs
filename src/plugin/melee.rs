use crate::{
    plugin::{
        collision::CollisionSystems, Actor, ActorAction, ActorActionsExt, ActorConfig, AudioPlay,
        AudioTracker, Weapon,
    },
    resource::HitResource,
    state::AppState,
    util::{
        ext::{AppExt, QuatExt, Vec2Ext},
        math,
    },
};
use bevy::{
    ecs::{entity::Entity, system::Deferred, world::World},
    prelude::{App, Commands, IntoScheduleConfigs, Plugin, Query, Res, Transform, Vec2, Without},
    time::Time,
};

pub struct MeleePlugin;

impl Plugin for MeleePlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update.after(CollisionSystems));
    }
}

fn on_update(
    attackers: Query<(Entity, &Actor, &Transform), Without<Weapon>>,
    targets: Query<(Entity, &Actor, &Transform)>,
    mut hits: Deferred<HitResource>,
    audio: Res<AudioTracker>,
    mut commands: Commands,
    time: Res<Time>,
) {
    crate::util::bench::bench!();
    let time = time.elapsed();

    for (entity, actor, transform) in attackers.iter() {
        if !actor.actions.is_attacking() {
            continue;
        }

        if actor.melee_next > time {
            continue;
        }

        let position = transform.translation.truncate();
        let rotation = transform.rotation.angle_z();

        if let Some(victim) = find_victim(actor.config, position, rotation, &targets) {
            let momentum = actor.config.melee_damage * actor.skill;

            hits.add(
                victim.entity,
                Vec2::from_length(momentum, victim.angle_objective),
                -victim.angle_subjective,
                false,
            );

            audio.queue(AudioPlay {
                path: "sounds/melee".into(),
                volume: 0.6,
                source: Some(position),
                ..AudioPlay::DEFAULT
            });

            commands.queue(move |world: &mut World| {
                if let Some(mut actor) = world.get_mut::<Actor>(entity) {
                    actor.actions.remove(ActorAction::Attack);
                    actor.melee_next = time + actor.config.melee_interval.div_f32(actor.skill);
                }
            });
        }
    }
}

fn find_victim(
    own_config: &ActorConfig,
    own_position: Vec2,
    own_rotation: f32,
    targets: &Query<(Entity, &Actor, &Transform)>,
) -> Option<Victim> {
    let mut victim = None;

    // TODO: optimize by using spatial index
    for (entity, actor, transform) in targets.iter() {
        if own_config.kind == actor.config.kind {
            continue;
        }

        let relative = transform.translation.truncate() - own_position;
        let distance_to_hit = own_config.melee_distance + actor.config.radius; // TODO: add own body radius

        if relative.is_long(distance_to_hit) {
            continue;
        }

        let angle_objective = relative.to_angle();
        let angle_subjective = math::angle_difference(angle_objective, own_rotation);
        let distance_angular = angle_subjective.abs() / own_config.melee_distance_angular * 2.0;

        if distance_angular > 1.0 {
            continue;
        }

        let distance = relative.length() / distance_to_hit;

        if victim
            .as_ref()
            .is_none_or(|v: &Victim| v.distance > distance)
        {
            victim = Some(Victim {
                entity,
                distance,
                angle_objective,
                angle_subjective,
            });
        }
    }

    return victim;
}

struct Victim {
    entity: Entity,
    distance: f32,
    angle_objective: f32,
    angle_subjective: f32,
}
