use crate::{
    command::ActorRelease,
    component::Actor,
    event::ActorDeathEvent,
    model::{AppState, AudioPlay},
    plugin::{player::Player, AudioTracker, BloodSpawn, FleshParticleSpawn},
    resource::{GameMode, Settings},
    util::{ext::AppExt, Timer},
};
use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        query::Has,
        system::{Local, Query, Res},
    },
    math::{Vec2, Vec3Swizzles},
    prelude::{Commands, DespawnRecursiveExt, Entity, EventWriter, IntoSystemConfigs, Transform},
    time::Time,
};
use std::time::Duration;

/// Increased buffering helps to summarize small and frequent damage events into one which is good
/// for visual effects like blood. But also it increases the delay
const BUFFERING: Duration = Duration::from_millis(100);
const LOW_VALUE: f32 = 0.4;
const FLESH_PARTICLE_PER_DAMAGE: f32 = 0.2;
const FLESH_PARTICLES_MAX: i32 = 8;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(
            AppState::Game,
            on_update.run_if(|mut r: Local<Timer>, t: Res<Time>| {
                r.next_if_ready(t.elapsed(), || BUFFERING)
            }),
        );
    }
}

/// NOTE: health must not be affected by skill, excepting player
#[derive(Component)]
pub struct Health {
    resistance: f32,
    /// In range of `0.0`` and `1.0`
    health: f32,
    /// In range of `0.0` and `INFINITY`
    damage: f32,
    just_died: bool,
}

impl Health {
    pub fn new(resistance: f32) -> Self {
        return Self {
            resistance,
            health: 1.0,
            damage: 0.0,
            just_died: false,
        };
    }

    pub fn damage(&mut self, mut damage: f32) {
        debug_assert!(
            damage > 0.0,
            "Damage must be greater than zero. Got instead: {}",
            damage,
        );

        let was_alive = self.is_alive();
        damage = f32::max(damage, 0.0) / self.resistance;
        self.health = (self.health - damage).clamp(0.0, 1.0);
        self.damage += damage;

        if was_alive && !self.is_alive() {
            self.just_died = true;
        }
    }

    pub fn heal(&mut self) {
        if self.is_alive() {
            self.health = 1.0;
        }
    }

    pub fn multiply_resistance(&mut self, n: f32) {
        self.resistance *= n;
    }

    pub fn get(&self) -> f32 {
        return self.health;
    }

    pub fn is_alive(&self) -> bool {
        return self.health > 0.0;
    }

    pub fn is_low(&self) -> bool {
        return self.health < LOW_VALUE;
    }
}

fn on_update(
    mut query: Query<(Entity, &Actor, &mut Health, &Transform, Has<Player>)>,
    mut death_events: EventWriter<ActorDeathEvent>,
    mut commands: Commands,
    settings: Res<Settings>,
    audio: Res<AudioTracker>,
) {
    for (entity, actor, mut health, transform, is_player) in query.iter_mut() {
        let actor = actor.config;
        let point = transform.translation.xy();

        if health.is_alive() && health.damage > actor.pain_threshold {
            audio.queue(AudioPlay {
                path: format!("{}/pain", actor.get_assets_path()).into(),
                volume: 0.9,
                source: Some(point),
                ..AudioPlay::DEFAULT
            });
        }

        spawn_blood(&health, &mut commands, point);
        spawn_flesh(&health, &mut commands, entity);

        if settings.game.modes.contains(&GameMode::Bench) {
            health.health = 1.0;
            health.just_died = false;
        }

        if health.just_died {
            audio.queue(AudioPlay {
                path: format!("{}/death", actor.get_assets_path()).into(),
                volume: 1.0,
                source: Some(point),
                ..AudioPlay::DEFAULT
            });

            commands.add(ActorRelease(entity));

            death_events.send(ActorDeathEvent {
                kind: actor.kind,
                position: point,
                is_player,
            });

            commands.entity(entity).despawn_recursive();
            health.just_died = false;
        }

        health.damage = 0.0;
    }
}

fn spawn_blood(health: &Health, commands: &mut Commands, point: Vec2) {
    let mut amount = health.damage;

    if health.just_died {
        amount *= 1.5;
    }

    if let Some(blood) = BloodSpawn::new(point, f32::min(amount, 1.0)) {
        commands.add(blood);
    }
}

fn spawn_flesh(health: &Health, commands: &mut Commands, entity: Entity) {
    let particles = if health.just_died {
        // TODO: play smash sound
        FLESH_PARTICLES_MAX
    } else {
        i32::min(
            (health.damage / FLESH_PARTICLE_PER_DAMAGE) as i32,
            FLESH_PARTICLES_MAX,
        )
    };

    for _ in 0..particles {
        commands.add(FleshParticleSpawn(entity));
    }
}
