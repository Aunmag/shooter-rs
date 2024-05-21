use crate::{model::AppState, plugin::command_scheduler::CommandScheduler, util::{ext::AppExt, SmartString}};
use bevy::{
    app::Update,
    ecs::{system::{ResMut, Resource}, world::Mut},
    prelude::{App, Commands, IntoSystemConfigs, Plugin, Res, Time, World},
};
use std::time::Duration;
use crate::{
    command::ActorSet,
    component::{Actor, ActorConfig, ActorKind},
    data::VIEW_DISTANCE,
    event::ActorDeathEvent,
    model::TransformLite,
    plugin::{
        bot::ActorBotSet,
        player::{Player, PlayerSet},
        BonusSpawn, Health, Notify, WeaponConfig, WeaponSet,
    },
    util::ext::Vec2Ext,
};
use bevy::{
    ecs::{query::With, system::Command},
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg32;
use std::{any::Any, f32::consts::PI};

// TODO: handle events

const AGILE_CHANCE: f64 = 0.1;

const WAVES: &[Wave] = &[
    // melee zombies only
    Wave {
        size: 5,
        pistol_chance: 0.0,
        rifle_chance: 0.0,
        agile_chance: 0.0,
    },
    Wave {
        size: 25,
        pistol_chance: 0.0,
        rifle_chance: 0.0,
        agile_chance: 0.0,
    },
    // agile zombies
    Wave {
        size: 50,
        pistol_chance: 0.0,
        rifle_chance: 0.0,
        agile_chance: AGILE_CHANCE,
    },
    Wave {
        size: 75,
        pistol_chance: 0.0,
        rifle_chance: 0.0,
        agile_chance: 0.3,
    },
    // zombies with pistols
    Wave {
        size: 100,
        pistol_chance: 0.2,
        rifle_chance: 0.0,
        agile_chance: AGILE_CHANCE,
    },
    Wave {
        size: 125,
        pistol_chance: 0.3,
        rifle_chance: 0.0,
        agile_chance: AGILE_CHANCE,
    },
    // zombies with rifles
    Wave {
        size: 150,
        pistol_chance: 0.3,
        rifle_chance: 0.1,
        agile_chance: AGILE_CHANCE,
    },
];

const WAVE_BONUS: Wave = Wave {
    size: u16::MAX,
    pistol_chance: 0.4,
    rifle_chance: 0.2,
    agile_chance: 0.0,
};

const ENEMY_SPAWN_DISTANCE: f32 = VIEW_DISTANCE * 0.5;
const BONUSES_PER_WAVE: f32 = 3.0;
const GAME_OVER_TEXT_DURATION: Duration = Duration::from_secs(8);
const DEFAULT_INTERVAL: Duration = Duration::from_secs(2);
const WAVE_BONUS_HUMANS: u8 = 16;

pub struct WavesScenarioPlugin;

impl Plugin for WavesScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_init);
    }
}

fn on_init(mut scheduler: ResMut<CommandScheduler>) {
    scheduler.add(DEFAULT_INTERVAL, on_update);
}

fn on_update(world: &mut World) {
    world.resource_scope(|w, mut s: Mut<Scenario>| s.update(w));
}

#[derive(Resource)]
struct Scenario {
    state: State,
    wave_index: u8,
    zombies_spawned: u16,
    kills: u16,
    rng: Pcg32,
}

impl Scenario {
    pub fn new() -> Self {
        return Self {
            state: State::StartNextWave,
            wave_index: 0,
            zombies_spawned: 0,
            kills: 0,
            rng: Pcg32::seed_from_u64(32),
        };
    }

    fn update(&mut self, world: &mut World) {
        match self.state {
            State::StartNextWave => {
                self.on_start_next_wave(world);
            }
            State::SpawnEnemy => {
                self.on_spawn_enemy(world);
            }
            State::CheckWaveCompletion => {
                self.on_check_completion(world);
            }
            State::CompleteWave => {
                self.on_complete_wave(world);
            }
        }
    }

    fn on_start_next_wave(&mut self, world: &mut World) {
        self.zombies_spawned = 0;
        self.kills = 0;

        if self.is_wave_bonus() {
            notify(
                world,
                "Bonus wave".into(),
                "How long will you stay? Support is on the way...".into(),
            );
        } else {
            notify(
                world,
                format!("Wave {}/{}", self.get_wave_number(), WAVES.len()).into(),
                format!("Kill {} zombies", self.get_wave().size).into(),
            );
        }

        heal_humans.apply(world); // TODO: simplify
        self.set_state(State::SpawnEnemy, world);
    }

    fn on_spawn_enemy(&mut self, world: &mut World) {
        log::debug!("Spawning a zombie");
        let wave = self.get_wave();
        let mut spawn = SpawnActor {
            direction: self.rng.gen_range(-PI..PI),
            distance: ENEMY_SPAWN_DISTANCE,
            config: &ActorConfig::ZOMBIE,
            weapon: None,
        };

        if self.rng.gen_bool(wave.agile_chance) {
            spawn.config = &ActorConfig::ZOMBIE_AGILE;
        } else if self.rng.gen_bool(wave.rifle_chance) {
            spawn.weapon = Some(&WeaponConfig::AKS_74U_BROKEN);
        } else if self.rng.gen_bool(wave.pistol_chance) {
            spawn.weapon = Some(&WeaponConfig::PM_BROKEN);
        }

        spawn.apply(world);
        self.zombies_spawned += 1;

        if self.zombies_spawned < wave.size {
            self.set_state(State::SpawnEnemy, world);
        } else {
            self.set_state(State::CheckWaveCompletion, world);
        }
    }

    fn on_check_completion(&mut self, world: &mut World) {
        count_zombies.apply(world); // TODO: simplify
        log::trace!("Checking for wave completion");
        self.set_state(State::CheckWaveCompletion, world);
    }

    fn on_complete_wave(&mut self, world: &mut World) {
        if self.is_wave_last() {
            notify(
                world,
                "Congratulations!".into(),
                format!("You've completed the all {} waves", WAVES.len()).into(),
            );
        } else {
            notify(
                world,
                format!("Wave {} completed!", self.get_wave_number()).into(),
                "Prepare for the next".into(),
            );
        }

        self.wave_index = self.wave_index.saturating_add(1);
        self.set_state(State::StartNextWave, world);
    }

    fn set_state(&mut self, state: State, world: &mut World) {
        if self.state == state {
            return;
        }

        self.state = state;

        let timeout = match self.state {
            State::StartNextWave => DEFAULT_INTERVAL,
            State::SpawnEnemy => Duration::from_millis(800),
            State::CheckWaveCompletion => DEFAULT_INTERVAL,
            State::CompleteWave => Duration::from_secs(4),
        };

        // TODO: test
        world.resource_mut::<CommandScheduler>().reschedule(on_update.type_id(), timeout);
    }

    fn get_wave(&self) -> &'static Wave {
        return WAVES
            .get(usize::from(self.wave_index))
            .unwrap_or(&WAVE_BONUS);
    }

    fn get_wave_number(&self) -> u8 {
        return self.wave_index.saturating_add(1);
    }

    fn is_wave_last(&self) -> bool {
        return usize::from(self.get_wave_number()) == WAVES.len();
    }

    fn is_wave_bonus(&self) -> bool {
        return usize::from(self.wave_index) == WAVES.len();
    }
}

#[derive(PartialEq)]
enum State {
    StartNextWave,
    SpawnEnemy,
    CheckWaveCompletion,
    CompleteWave,
}

struct Wave {
    size: u16,
    pistol_chance: f64,
    rifle_chance: f64,
    agile_chance: f64,
}

struct SpawnActor {
    direction: f32,
    distance: f32,
    config: &'static ActorConfig,
    weapon: Option<&'static WeaponConfig>,
}

impl Command for SpawnActor {
    fn apply(self, world: &mut World) {
        let mut center = Vec2::ZERO;
        let mut players = 0.0;

        for transform in world
            .query_filtered::<&Transform, With<Player>>()
            .iter(world)
        {
            center += transform.translation.xy();
            players += 1.0;
        }

        if players > 0.0 {
            center /= players;
        }

        let entity = world.spawn_empty().id();
        let mut transform = TransformLite::new(center.x, center.y, self.direction);
        transform.translation -= Vec2::from_length(self.distance, self.direction);

        ActorSet {
            entity,
            config: self.config,
            transform,
        }
        .apply(world);

        ActorBotSet { entity }.apply(world);

        if let Some(weapon) = self.weapon {
            WeaponSet {
                entity,
                weapon: Some(weapon),
            }
            .apply(world);
        }
    }
}

fn notify(world: &mut World, text: SmartString<'static>, text_small: SmartString<'static>) {
    Notify {
        text,
        text_small,
        ..Default::default()
    }.apply(world);
}

// TODO: rename
fn count_zombies(world: &mut World) {
    if !world
        .query::<&Actor>()
        .iter(world)
        .any(|a| a.config.kind == ActorKind::Zombie)
    {
        world.resource_scope(|w, mut s: Mut<Scenario>| s.set_state(State::CompleteWave, w));
    }
}

fn heal_humans(world: &mut World) {
    for (actor, mut health) in world.query::<(&Actor, &mut Health)>().iter_mut(world) {
        if let ActorKind::Human = actor.config.kind {
            health.heal();
        }
    }
}
