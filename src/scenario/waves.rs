use super::wave::WaveSeries;
use crate::{
    command::{ActorPlayerSet, ActorSet},
    component::{Actor, ActorConfig, ActorKind},
    event::ActorDeathEvent,
    model::TransformLite,
    plugin::{BonusSpawn, Health, Notify, WeaponConfig, WeaponSet},
    resource::{Scenario, ScenarioLogic},
};
use bevy::prelude::{Commands, World};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{any::Any, time::Duration};

const ENEMY_SPAWN_DISTANCE: f32 = 3.0; // TODO: reset
const BONUSES_PER_WAVE: f32 = 3.0;
const GAME_OVER_TEXT_DURATION: Duration = Duration::from_secs(8);
const DEFAULT_INTERVAL: Duration = Duration::from_secs(2);

// TODO: find center position

enum Task {
    StartNextWave,
    SpawnZombie,
    CheckWaveCompletion,
    CompleteWave,
}

impl Task {
    fn get_timeout(&self) -> Duration {
        return match self {
            Self::StartNextWave => DEFAULT_INTERVAL,
            Self::SpawnZombie => Duration::from_millis(800), // TODO: don't wait if everybody is killed
            Self::CheckWaveCompletion => DEFAULT_INTERVAL,
            Self::CompleteWave => Duration::from_secs(4),
        };
    }
}

pub struct WavesScenario {
    waves: WaveSeries,
    task: Task,
    rng: Pcg32,
}

impl WavesScenario {
    pub fn new() -> Self {
        return Self {
            waves: WaveSeries::new(),
            task: Task::StartNextWave,
            rng: Pcg32::seed_from_u64(32),
        };
    }

    fn spawn_player(commands: &mut Commands) {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: &ActorConfig::HUMAN,
            transform: TransformLite::default(),
        });

        commands.add(ActorPlayerSet {
            entity,
            is_controllable: true,
        });

        commands.add(WeaponSet {
            entity,
            weapon: Some(&WeaponConfig::PM),
        });
    }

    fn update(&mut self, commands: &mut Commands) -> Task {
        let Some(wave) = self.waves.get_current() else {
            return Task::CheckWaveCompletion;
        };

        match self.task {
            Task::StartNextWave => {
                let wave_size = wave.size;

                // TODO: test
                let tip = match self.waves.get_wave_number() {
                    1 => "\nTip: Press [R] to reload",
                    2 => "\nTip: Press [RMB] to aim",
                    3 => "\nTip: Press [SHIFT] to sprint",
                    4 => "\nTip: Use mouse wheel to change zoom",
                    _ => "",
                };

                commands.add(Notify {
                    text: format!(
                        "Wave {}/{}",
                        self.waves.get_wave_number(),
                        self.waves.get_waves_count(),
                    )
                    .into(),
                    text_small: format!("Kill {} zombies{}", wave_size, tip).into(),
                    ..Default::default()
                });

                commands.add(heal_humans);
                return Task::SpawnZombie;
            }
            Task::SpawnZombie => {
                log::debug!("Spawning a zombie");

                wave.spawn(
                    TransformLite::default(), // TODO: change
                    commands,
                    &mut self.rng,
                );

                if wave.is_complete() {
                    return Task::CheckWaveCompletion;
                } else {
                    return Task::SpawnZombie;
                }
            }
            Task::CheckWaveCompletion => {
                commands.add(count_zombies);
                log::trace!("Checking for wave completion");
                return Task::CheckWaveCompletion;
            }
            Task::CompleteWave => {
                if self.waves.is_final() {
                    commands.add(Notify {
                        text: "Congratulations!".into(),
                        text_small: format!(
                            "You've completed the all {} waves",
                            self.waves.get_waves_count(),
                        )
                        .into(),
                        ..Default::default()
                    });
                } else {
                    commands.add(Notify {
                        text: format!("Wave {} completed!", self.waves.get_wave_number()).into(),
                        text_small: "Prepare for the next".into(),
                        ..Default::default()
                    });
                }

                self.waves.next();
                return Task::StartNextWave;
            }
        }
    }
}

impl ScenarioLogic for WavesScenario {
    fn on_start(&mut self, commands: &mut Commands) -> Duration {
        Self::spawn_player(commands);
        return DEFAULT_INTERVAL;
    }

    fn on_actor_death(&mut self, event: &ActorDeathEvent, commands: &mut Commands) {
        if let ActorKind::Zombie = event.kind {
            let wave_size = self.waves.get_current().map(|w| w.size).unwrap_or(0);

            if wave_size > 0 {
                let wave = f32::from(self.waves.get_wave_number());

                if self
                    .rng
                    .gen_bool(f32::min(BONUSES_PER_WAVE * wave / f32::from(wave_size), 1.0).into())
                {
                    commands.add(BonusSpawn::new(
                        event.position,
                        self.waves.get_wave_number(),
                    ));
                }
            }
        }
    }

    fn on_player_death(&mut self, _: &ActorDeathEvent, commands: &mut Commands) {
        commands.add(Notify {
            text: "Game over".into(),
            text_small: "You died. Press [ESC] to exit".into(),
            duration: GAME_OVER_TEXT_DURATION,
        });
    }

    fn on_interval_update(&mut self, commands: &mut Commands) -> Duration {
        let timeout = self.task.get_timeout();
        self.task = WavesScenario::update(self, commands);
        return timeout;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}

fn count_zombies(world: &mut World) {
    if !world
        .query::<&Actor>()
        .iter(world)
        .any(|a| a.config.kind == ActorKind::Zombie)
    {
        if let Some(scenario) = world.resource_mut::<Scenario>().logic::<WavesScenario>() {
            scenario.task = Task::CompleteWave;
        }
    }
}

fn heal_humans(world: &mut World) {
    for (actor, mut health) in world.query::<(&Actor, &mut Health)>().iter_mut(world) {
        if let ActorKind::Human = actor.config.kind {
            health.heal();
        }
    }
}
