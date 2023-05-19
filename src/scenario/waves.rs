use crate::command::ActorBotSet;
use crate::command::ActorPlayerSet;
use crate::command::ActorSet;
use crate::component::Actor;
use crate::component::ActorConfig;
use crate::component::ActorType;
use crate::data::VIEW_DISTANCE;
use crate::model::TransformLiteU8;
use crate::resource::Scenario;
use crate::resource::ScenarioLogic;
use crate::util::math::compress_radians;
use bevy::ecs::system::Command;
use bevy::math::Vec2;
use bevy::math::Vec3Swizzles;
use bevy::prelude::Commands;
use bevy::prelude::World;
use bevy::transform::components::Transform;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use std::any::Any;
use std::f32::consts::PI;
use std::time::Duration;

const ZOMBIES_SPAWN_QUANTITY: u16 = 5;
const ZOMBIES_SPAWN_DISTANCE: f32 = VIEW_DISTANCE / 1.5;

enum Task {
    Start,
    PrepareNextWave,
    StartNextWave,
    SpawnZombie,
    CheckWaveCompletion,
    CompleteWave,
}

impl Task {
    fn get_timeout(&self) -> Duration {
        return match self {
            Self::Start => Duration::ZERO,
            Self::PrepareNextWave => Duration::from_secs(2),
            Self::StartNextWave => Duration::ZERO,
            Self::SpawnZombie => Duration::from_millis(500),
            Self::CheckWaveCompletion => Duration::from_secs(2),
            Self::CompleteWave => Duration::ZERO,
        };
    }
}

pub struct WavesScenario {
    task: Task,
    wave: u16,
    zombies_to_spawn: u16,
    rng: Pcg32,
}

impl WavesScenario {
    pub fn new() -> Self {
        return Self {
            task: Task::Start,
            wave: 0,
            zombies_to_spawn: 0,
            rng: Pcg32::seed_from_u64(32),
        };
    }

    fn spawn_player(&mut self, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: ActorConfig::HUMAN,
            transform: TransformLiteU8::default(),
            is_ghost: false,
        });

        commands.add(ActorPlayerSet(entity));
    }

    fn update(&mut self, commands: &mut Commands) -> Task {
        match self.task {
            Task::Start => {
                log::debug!("Starting waves scenario");
                self.spawn_player(commands);
                return Task::PrepareNextWave;
            }
            Task::PrepareNextWave => {
                log::info!("Prepare for the wave");
                return Task::StartNextWave;
            }
            Task::StartNextWave => {
                self.wave += 1;
                self.zombies_to_spawn = ZOMBIES_SPAWN_QUANTITY * self.wave * self.wave;
                log::info!("Wave {}. Kill {} zombies", self.wave, self.zombies_to_spawn);
                return Task::SpawnZombie;
            }
            Task::SpawnZombie => {
                if self.zombies_to_spawn > 0 {
                    commands.add(SpawnZombie {
                        direction: self.rng.gen_range(-PI..PI),
                    });
                    self.zombies_to_spawn -= 1;
                    log::debug!("Spawning a zombie");
                }

                if self.zombies_to_spawn > 0 {
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
                log::info!("Wave {} completed!", self.wave);
                return Task::PrepareNextWave;
            }
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

        let transform = TransformLiteU8::new(
            center.x + ZOMBIES_SPAWN_DISTANCE * self.direction.cos(),
            center.y + ZOMBIES_SPAWN_DISTANCE * self.direction.sin(),
            compress_radians(self.direction),
        );

        ActorSet {
            entity,
            config: ActorConfig::ZOMBIE,
            transform,
            is_ghost: false,
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
