use crate::{
    command::{ActorBotSet, ActorPlayerSet, ActorSet, BonusSpawn, Notify, WeaponSet},
    component::{Actor, ActorConfig, ActorKind, Health, WeaponConfig},
    data::VIEW_DISTANCE,
    event::ActorDeathEvent,
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

const WAVES: &[Wave] = &[
    // melee zombies only
    Wave {
        size: 5,
        pistol_chance: 0.0,
        rifle_chance: 0.0,
    },
    Wave {
        size: 25,
        pistol_chance: 0.0,
        rifle_chance: 0.0,
    },
    Wave {
        size: 50,
        pistol_chance: 0.015, // may be a little surprise
        rifle_chance: 0.0,
    },
    // zombies with pistols
    Wave {
        size: 75,
        pistol_chance: 0.1,
        rifle_chance: 0.0,
    },
    Wave {
        size: 100,
        pistol_chance: 0.2,
        rifle_chance: 0.0,
    },
    Wave {
        size: 125,
        pistol_chance: 0.3,
        rifle_chance: 0.0,
    },
    // zombies with rifles
    Wave {
        size: 150,
        pistol_chance: 0.3,
        rifle_chance: 0.1,
    },
];

const WAVE_BONUS: Wave = Wave {
    size: u16::MAX,
    pistol_chance: 0.0,
    rifle_chance: 1.0,
};

const ZOMBIE_SPAWN_DISTANCE: f32 = VIEW_DISTANCE * 0.5;
const BONUSES_PER_WAVE: f32 = 3.0;
const GAME_OVER_TEXT_DURATION: Duration = Duration::from_secs(8);
const DEFAULT_INTERVAL: Duration = Duration::from_secs(2);

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
            Self::SpawnZombie => Duration::from_millis(800),
            Self::CheckWaveCompletion => DEFAULT_INTERVAL,
            Self::CompleteWave => Duration::from_secs(4),
        };
    }
}

pub struct WavesScenario {
    task: Task,
    wave_index: u8,
    zombies_spawned: u16,
    kills: u16,
    rng: Pcg32,
}

impl WavesScenario {
    pub fn new() -> Self {
        return Self {
            task: Task::StartNextWave,
            wave_index: 0,
            zombies_spawned: 0,
            kills: 0,
            rng: Pcg32::seed_from_u64(32),
        };
    }

    fn spawn_player(commands: &mut Commands) {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: &ActorConfig::HUMAN,
            skill: 1.0,
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
        let wave = self.wave();

        match self.task {
            Task::StartNextWave => {
                self.zombies_spawned = 0;
                self.kills = 0;

                if usize::from(self.wave_index) < WAVES.len() {
                    commands.add(HealHumans);
                    commands.add(Notify {
                        text: format!("Wave {}/{}", self.wave_number(), WAVES.len()).into(),
                        text_small: format!("Kill {} zombies", wave.size).into(),
                        ..Default::default()
                    });
                } else {
                    commands.add(Notify {
                        text: "Wait".into(),
                        text_small: "NOW IT IS TIME TO SUFFER".into(),
                        ..Default::default()
                    });
                }

                return Task::SpawnZombie;
            }
            Task::SpawnZombie => {
                log::debug!("Spawning a zombie");

                let weapon = if self.rng.gen_bool(wave.pistol_chance) {
                    Some(&WeaponConfig::PM)
                } else if self.rng.gen_bool(wave.rifle_chance) {
                    Some(&WeaponConfig::AKS_74U)
                } else {
                    None
                };

                commands.add(SpawnZombie {
                    skill: 1.0,
                    direction: self.rng.gen_range(-PI..PI),
                    weapon,
                });

                self.zombies_spawned += 1;

                if self.zombies_spawned < wave.size {
                    return Task::SpawnZombie;
                } else {
                    return Task::CheckWaveCompletion;
                }
            }
            Task::CheckWaveCompletion => {
                commands.add(CountZombies);
                log::trace!("Checking for wave completion");
                return Task::CheckWaveCompletion;
            }
            Task::CompleteWave => {
                if usize::from(self.wave_number()) == WAVES.len() {
                    commands.add(Notify {
                        text: "Congratulations!".into(),
                        text_small: format!("You've completed the all {} waves", WAVES.len())
                            .into(),
                        ..Default::default()
                    });
                } else {
                    commands.add(Notify {
                        text: format!("Wave {} completed!", self.wave_number()).into(),
                        text_small: "Prepare for the next".into(),
                        ..Default::default()
                    });
                }

                self.wave_index = self.wave_index.saturating_add(1);
                return Task::StartNextWave;
            }
        }
    }

    fn wave(&self) -> &'static Wave {
        return WAVES
            .get(usize::from(self.wave_index))
            .unwrap_or(&WAVE_BONUS);
    }

    fn wave_number(&self) -> u8 {
        return self.wave_index.saturating_add(1);
    }
}

impl ScenarioLogic for WavesScenario {
    fn on_start(&mut self, commands: &mut Commands) -> Duration {
        Self::spawn_player(commands);
        return DEFAULT_INTERVAL;
    }

    fn on_actor_death(&mut self, event: &ActorDeathEvent, commands: &mut Commands) {
        if let ActorKind::Zombie = event.kind {
            self.kills += 1;

            if self.kills == 1 {
                match self.wave_index {
                    0 => {
                        commands.add(Notify {
                            text_small: "Press [R] to reload".into(),
                            ..Default::default()
                        });
                    }
                    1 => {
                        commands.add(Notify {
                            text_small: "Press [SHIFT] to sprint".into(),
                            ..Default::default()
                        });
                    }
                    2 => {
                        commands.add(Notify {
                            text_small: "Use mouse wheel to change zoom".into(),
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
            }

            let wave = f32::from(self.wave_number());
            let wave_size = f32::from(self.wave().size);

            if self
                .rng
                .gen_bool(f32::min(BONUSES_PER_WAVE * wave / wave_size, 1.0).into())
            {
                commands.add(BonusSpawn::new(event.position, self.wave_number()));
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

struct Wave {
    size: u16,
    pistol_chance: f64,
    rifle_chance: f64,
}

struct SpawnZombie {
    skill: f32,
    direction: f32,
    weapon: Option<&'static WeaponConfig>,
}

impl Command for SpawnZombie {
    fn apply(self, world: &mut World) {
        let mut center = Vec2::ZERO;
        let mut humans = 0.0;

        for (transform, actor) in world.query::<(&Transform, &Actor)>().iter(world) {
            if let ActorKind::Human = actor.config.kind {
                center += transform.translation.xy();
                humans += 1.0;
            }
        }

        if humans > 0.0 {
            center /= humans;
        }

        let entity = world.spawn_empty().id();
        let offset = Vec2::from_length(ZOMBIE_SPAWN_DISTANCE, self.direction);
        let transform =
            TransformLite::new(center.x + offset.x, center.y + offset.y, self.direction);

        ActorSet {
            entity,
            config: &ActorConfig::ZOMBIE,
            skill: self.skill,
            transform,
        }
        .apply(world);

        ActorBotSet {
            entity,
            skill: self.skill,
        }
        .apply(world);

        if let Some(weapon) = self.weapon {
            WeaponSet {
                entity,
                weapon: Some(weapon),
            }
            .apply(world);
        }
    }
}

struct CountZombies;

impl Command for CountZombies {
    fn apply(self, world: &mut World) {
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
}

struct HealHumans;

impl Command for HealHumans {
    fn apply(self, world: &mut World) {
        for (actor, mut health) in world.query::<(&Actor, &mut Health)>().iter_mut(world) {
            if let ActorKind::Human = actor.config.kind {
                health.heal();
            }
        }
    }
}
