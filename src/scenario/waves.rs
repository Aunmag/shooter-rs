use crate::{
    command::{ActorBotSet, ActorPlayerSet, ActorSet, BonusSpawn, Notify},
    component::{Actor, ActorConfig, ActorType, Health},
    event::ActorDeathEvent,
    model::TransformLite,
    resource::{Scenario, ScenarioLogic},
    util::{
        ext::{Pcg32Ext, Vec2Ext},
        math::interpolate,
    },
};
use bevy::{
    ecs::system::Command,
    math::{Vec2, Vec3Swizzles},
    prelude::{Commands, World},
    transform::components::Transform,
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{any::Any, f32::consts::PI, time::Duration};

const WAVE_FINAL: u8 = 6;
const WAVE_SIZE_INITIAL: u16 = 5;
const ZOMBIE_SPAWN_DISTANCE_MIN: f32 = 20.0;
const ZOMBIE_SPAWN_DISTANCE_MAX: f32 = 60.0;
const ZOMBIE_SKILL_MIN: f32 = 1.0;
const ZOMBIE_SKILL_MAX: f32 = 2.3;
const BONUSES_PER_WAVE: f32 = 3.0;

enum Task {
    Start,
    StartNextWave,
    SpawnZombie,
    CheckWaveCompletion,
    CompleteWave,
}

impl Task {
    fn get_timeout(&self) -> Duration {
        return match self {
            Self::Start => Duration::from_secs(2),
            Self::StartNextWave => Duration::from_secs(2),
            Self::SpawnZombie => Duration::from_millis(800),
            Self::CheckWaveCompletion => Duration::from_secs(2),
            Self::CompleteWave => Duration::from_secs(4),
        };
    }
}

pub struct WavesScenario {
    task: Task,
    wave: u8,
    zombies_spawned: u16,
    kills: u16,
    rng: Pcg32,
}

impl WavesScenario {
    pub fn new() -> Self {
        return Self {
            task: Task::Start,
            wave: 0,
            zombies_spawned: 0,
            kills: 0,
            rng: Pcg32::seed_from_u64(32),
        };
    }

    fn spawn_player(&mut self, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: &ActorConfig::HUMAN,
            skill: 1.0,
            transform: TransformLite::default(),
        });

        commands.add(ActorPlayerSet(entity));
    }

    fn update(&mut self, commands: &mut Commands) -> Task {
        match self.task {
            Task::Start => {
                log::debug!("Starting waves scenario");
                self.spawn_player(commands);
                return Task::StartNextWave;
            }
            Task::StartNextWave => {
                self.wave += 1;
                self.zombies_spawned = 0;
                self.kills = 0;

                if self.wave > WAVE_FINAL {
                    commands.add(Notify::new(
                        "Wait".to_string(),
                        "NOW IT IS TIME TO SUFFER".to_string(),
                    ));
                } else {
                    commands.add(HealHumans);
                    commands.add(Notify::new(
                        format!("Wave {}/{}", self.wave, WAVE_FINAL),
                        format!("Kill {} zombies", self.wave_size()),
                    ));
                }

                return Task::SpawnZombie;
            }
            Task::SpawnZombie => {
                log::debug!("Spawning a zombie");

                commands.add(SpawnZombie {
                    skill: interpolate(ZOMBIE_SKILL_MIN, ZOMBIE_SKILL_MAX, self.progress()),
                    distance: self.generate_spawn_distance(),
                    direction: self.rng.gen_range(-PI..PI),
                });

                self.zombies_spawned += 1;

                if self.zombies_spawned < self.wave_size() {
                    return Task::SpawnZombie;
                } else {
                    return Task::CheckWaveCompletion;
                }
            }
            Task::CheckWaveCompletion => {
                commands.add(CountZombies);
                log::debug!("Checking for wave completion");
                return Task::CheckWaveCompletion;
            }
            Task::CompleteWave => {
                if self.wave == WAVE_FINAL {
                    commands.add(Notify::new(
                        "Congratulations!".to_string(),
                        format!("You've completed the all {} waves", WAVE_FINAL),
                    ));
                } else {
                    commands.add(Notify::new(
                        format!("Wave {} completed!", self.wave),
                        "Prepare for the next".to_string(),
                    ));
                }

                return Task::StartNextWave;
            }
        }
    }

    fn wave_size(&self) -> u16 {
        if self.wave > WAVE_FINAL {
            return u16::MAX;
        } else {
            let wave = u16::from(self.wave);
            return WAVE_SIZE_INITIAL * wave * wave;
        }
    }

    fn progress(&self) -> f32 {
        return f32::min(f32::from(self.wave - 1) / f32::from(WAVE_FINAL - 1), 1.0);
    }

    fn generate_spawn_distance(&mut self) -> f32 {
        let min = ZOMBIE_SPAWN_DISTANCE_MIN;
        let max = interpolate(min, ZOMBIE_SPAWN_DISTANCE_MAX, self.progress());
        return self.rng.gen_range_safely(min, max);
    }
}

impl ScenarioLogic for WavesScenario {
    fn update(&mut self, commands: &mut Commands) -> Duration {
        let timeout = self.task.get_timeout();
        self.task = WavesScenario::update(self, commands);
        return timeout;
    }

    fn on_actor_death(&mut self, event: &ActorDeathEvent, commands: &mut Commands) {
        self.kills += 1;

        if self.kills == 1 {
            match self.wave {
                1 => {
                    commands.add(Notify::new(
                        String::new(),
                        "Press [R] to reload".to_string(),
                    ));
                }
                2 => {
                    commands.add(Notify::new(
                        String::new(),
                        "Press [SHIFT] to sprint".to_string(),
                    ));
                }
                3 => {
                    commands.add(Notify::new(
                        String::new(),
                        "Use mouse wheel to change zoom".to_string(),
                    ));
                }
                _ => {}
            }
        }

        let wave = f32::from(self.wave);
        let wave_size = f32::from(self.wave_size());

        if self
            .rng
            .gen_bool((BONUSES_PER_WAVE * wave / wave_size).into())
        {
            commands.add(BonusSpawn::new(event.position, self.wave));
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}

struct SpawnZombie {
    skill: f32,
    distance: f32,
    direction: f32,
}

impl Command for SpawnZombie {
    fn write(self, world: &mut World) {
        let mut center = Vec2::ZERO;
        let mut humans = 0.0;

        for (transform, actor) in world.query::<(&Transform, &Actor)>().iter(world) {
            if let ActorType::Human = actor.config.actor_type {
                center += transform.translation.xy();
                humans += 1.0;
            }
        }

        if humans > 0.0 {
            center /= humans;
        }

        let entity = world.spawn_empty().id();
        let offset = Vec2::from_length(self.distance, self.direction);
        let transform =
            TransformLite::new(center.x + offset.x, center.y + offset.y, self.direction);

        ActorSet {
            entity,
            config: &ActorConfig::ZOMBIE,
            skill: self.skill,
            transform,
        }
        .write(world);

        ActorBotSet(entity).write(world);
    }
}

struct CountZombies;

impl Command for CountZombies {
    fn write(self, world: &mut World) {
        if !world
            .query::<&Actor>()
            .iter(world)
            .any(|a| a.config.actor_type == ActorType::Zombie)
        {
            if let Some(scenario) = world.resource_mut::<Scenario>().logic::<WavesScenario>() {
                scenario.task = Task::CompleteWave; // TODO: count duration
            }
        }
    }
}

struct HealHumans;

impl Command for HealHumans {
    fn write(self, world: &mut World) {
        for (actor, mut health) in world.query::<(&Actor, &mut Health)>().iter_mut(world) {
            if let ActorType::Human = actor.config.actor_type {
                health.heal();
            }
        }
    }
}
