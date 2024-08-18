use crate::{
    command::ActorSet,
    component::ActorConfig,
    model::TransformLite,
    plugin::{bot::ActorBotSet, player::PlayerSet, WeaponConfig, WeaponSet},
    resource::ScenarioLogic,
};
use bevy::{app::AppExit, ecs::world::World, prelude::Commands};
use chrono::Local;
use std::{
    any::Any,
    fs::File,
    io::{Result, Write},
    path::Path,
    time::Duration,
};

const SPAWN_BATCH: usize = 100;
const SPAWN_MAX: usize = 3000;
const INTERVAL: Duration = Duration::from_secs(1);
const REPORTS_DIRECTORY: &str = "./bench/temp";

#[derive(Default)]
pub struct BenchScenario {
    spawned: usize,
    updates: usize,
    updates_per_second: Vec<(usize, i32)>,
    is_complete: bool,
}

impl BenchScenario {
    fn spawn_player(&mut self, commands: &mut Commands) {
        let entity = commands.spawn_empty().id();

        commands.add(ActorSet {
            entity,
            config: &ActorConfig::HUMAN,
            transform: TransformLite::default(),
        });

        commands.add(PlayerSet {
            entity,
            is_controllable: false,
        });

        commands.add(WeaponSet {
            entity,
            weapon: Some(&WeaponConfig::PM),
        });

        self.spawned += 1;
    }

    fn spawn_batch(&mut self, commands: &mut Commands) {
        for _ in 0..SPAWN_BATCH {
            let entity = commands.spawn_empty().id();

            commands.add(ActorSet {
                entity,
                config: &ActorConfig::ZOMBIE,
                transform: TransformLite::default(),
            });

            commands.add(ActorBotSet { entity });

            self.spawned += 1;
        }
    }

    fn calc_updates(&mut self) {
        let fps = (self.updates as f64 / INTERVAL.as_secs_f64()) as i32;
        log::debug!("Spawned {} actors, FPS: {}", self.spawned, fps);
        self.updates_per_second.push((self.spawned, fps));
        self.updates = 0;
    }

    fn save_report(&self) -> Result<()> {
        let directory_path = Path::new(REPORTS_DIRECTORY);
        let file_name = Local::now().format("%Y-%m-%d %H-%M-%S").to_string();
        let file_path = directory_path.join(file_name);

        std::fs::create_dir_all(directory_path)?;
        let mut file = File::create(&file_path)?;

        for (spawned, fps) in &self.updates_per_second {
            file.write_all(format!("{},{}\n", spawned, fps).as_bytes())?;
        }

        log::info!("Report saved to file: {}", file_path.display());
        return Ok(());
    }
}

impl ScenarioLogic for BenchScenario {
    fn on_start(&mut self, commands: &mut Commands) -> Duration {
        self.spawn_player(commands);
        return INTERVAL;
    }

    fn on_interval_update(&mut self, commands: &mut Commands) -> Duration {
        self.calc_updates();

        if self.spawned < SPAWN_MAX {
            self.spawn_batch(commands);
        } else if !self.is_complete {
            self.is_complete = true;

            if let Err(error) = self.save_report() {
                log::error!("Failed to save report: {:?}", error);
            }

            log::info!("Benchmark completed");
            commands.add(|w: &mut World| {
                w.send_event(AppExit::Success);
            });
        }

        return INTERVAL;
    }

    fn on_constant_update(&mut self, _: &mut Commands) {
        self.updates += 1;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        return self;
    }
}
