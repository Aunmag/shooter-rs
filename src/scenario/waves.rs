use crate::{
    command::{ActorBotSet, ActorPlayerSet, ActorSet, Notify},
    component::{Actor, ActorConfig, ActorType, Health},
    data::VIEW_DISTANCE,
    model::TransformLite,
    resource::{Scenario, ScenarioLogic},
    util::ext::Vec2Ext,
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

const WAVE_FINAL: u16 = 6;
const ZOMBIES_SPAWN_QUANTITY: u16 = 5;
const ZOMBIES_SPAWN_DISTANCE: f32 = VIEW_DISTANCE / 1.5;

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
            Self::SpawnZombie => Duration::from_millis(500),
            Self::CheckWaveCompletion => Duration::from_secs(2),
            Self::CompleteWave => Duration::from_secs(4),
        };
    }
}

pub struct WavesScenario {
    task: Task,
    wave: u16,
    zombies_spawned: u16,
    rng: Pcg32,
}

impl WavesScenario {
    pub fn new() -> Self {
        return Self {
            task: Task::Start,
            wave: 0,
            zombies_spawned: 0,
            rng: Pcg32::seed_from_u64(32),
        };
    }

    fn spawn_player(&mut self, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: ActorConfig::HUMAN,
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

                if self.wave > WAVE_FINAL {
                    commands.add(Notify::new(
                        "Wait".to_string(),
                        "NOW IT IS TIME TO SUFFER".to_string(),
                    ));
                } else {
                    commands.add(HealHumans);
                    commands.add(Notify::new(
                        format!("Wave {}/{}", self.wave, WAVE_FINAL),
                        format!("Kill {} zombies", self.zombies_to_spawn()),
                    ));
                }

                return Task::SpawnZombie;
            }
            Task::SpawnZombie => {
                log::debug!("Spawning a zombie");

                commands.add(SpawnZombie {
                    direction: self.rng.gen_range(-PI..PI),
                });

                self.zombies_spawned += 1;

                if self.wave == 2 && self.zombies_spawned == 5 {
                    commands.add(Notify::new(
                        "".to_string(),
                        "Press [SHIFT] to sprint".to_string(),
                    ));
                }

                if self.zombies_spawned < self.zombies_to_spawn() {
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

    fn zombies_to_spawn(&self) -> u16 {
        if self.wave > WAVE_FINAL {
            return u16::MAX;
        } else {
            return ZOMBIES_SPAWN_QUANTITY * self.wave * self.wave;
        }
    }
}

impl ScenarioLogic for WavesScenario {
    fn update(&mut self, commands: &mut Commands) -> Duration {
        let timeout = self.task.get_timeout();
        self.task = WavesScenario::update(self, commands);
        return timeout;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}

struct SpawnZombie {
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
        let offset = Vec2::from_length(ZOMBIES_SPAWN_DISTANCE, self.direction);
        let transform =
            TransformLite::new(center.x + offset.x, center.y + offset.y, self.direction);

        ActorSet {
            entity,
            config: ActorConfig::ZOMBIE,
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
