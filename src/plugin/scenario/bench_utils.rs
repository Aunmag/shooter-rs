use anyhow::Result;
use bevy::{
    app::AppExit,
    ecs::{system::Commands, world::World},
};
use chrono::Local;
use std::{fs::File, io::Write, path::Path, time::Duration};

const REPORTS_DIRECTORY: &str = "./target/bench/temp";
const WARMUP_DURATION: Duration = Duration::from_secs(5);
const SECOND: Duration = Duration::from_secs(1);

enum Stage {
    Warmup,
    Running,
    Complete,
}

pub struct Bench {
    stage: Stage,
    updates: usize,
    report: Vec<(usize, i32)>,
    timer: Duration,
    pub spawned: usize,
}

impl Default for Bench {
    fn default() -> Self {
        return Self {
            stage: Stage::Warmup,
            updates: 0,
            report: Vec::with_capacity(60 * 60),
            timer: Duration::ZERO,
            spawned: 0,
        };
    }
}

impl Bench {
    pub fn try_next(&mut self, now: Duration) -> bool {
        match self.stage {
            Stage::Warmup => {
                if self.timer.is_zero() {
                    self.timer = now + WARMUP_DURATION;
                } else if self.timer <= now {
                    self.stage = Stage::Running;
                    self.timer = now + SECOND;
                }

                return false;
            }
            Stage::Running => {
                self.updates += 1;

                if self.timer <= now {
                    self.calc_updates();
                    self.timer += SECOND;
                    return true;
                } else {
                    return false;
                }
            }
            Stage::Complete => {
                return false;
            }
        }
    }

    // TODO: rename
    fn calc_updates(&mut self) {
        let fps = (self.updates as f64 / SECOND.as_secs_f64()) as i32;
        log::debug!("Spawned: {} | FPS: {}", self.spawned, fps);
        self.report.push((self.spawned, fps));
        self.updates = 0;
    }

    pub fn save_report(&self) -> Result<()> {
        let directory_path = Path::new(REPORTS_DIRECTORY);
        let file_name = Local::now().format("%Y-%m-%d %H-%M-%S").to_string();
        let file_path = directory_path.join(file_name);

        std::fs::create_dir_all(directory_path)?;
        let mut file = File::create(&file_path)?;

        for (spawned, fps) in &self.report {
            file.write_all(format!("{},{}\n", spawned, fps).as_bytes())?;
        }

        log::info!("Report saved to file: {}", file_path.display());
        return Ok(());
    }

    pub fn finish(&mut self, commands: &mut Commands) {
        if matches!(self.stage, Stage::Complete) {
            return;
        }

        self.stage = Stage::Complete;

        if let Err(error) = self.save_report() {
            log::error!("Failed to save report: {:?}", error);
        }

        log::info!("Benchmark completed");

        commands.add(|w: &mut World| {
            w.send_event(AppExit::Success);
        });
    }
}
