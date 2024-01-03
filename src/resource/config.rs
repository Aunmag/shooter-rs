use anyhow::{Context, Result};
use bevy::{
    ecs::system::Resource,
    window::{PresentMode, WindowMode},
};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize, Resource)]
#[serde(default)]
pub struct Config {
    pub game: GameConfig,
    pub display: DisplayConfig,
    pub audio: AudioConfig,
    pub controls: ControlsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameConfig {
    pub modes: Vec<GameMode>,
}

impl Default for GameConfig {
    fn default() -> Self {
        return Self {
            modes: vec![GameMode::Waves],
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameMode {
    Waves,
    Debug,
    Bench,
    LaserSight,
}

impl GameMode {
    pub fn dependencies(&self) -> &'static [GameMode] {
        return match self {
            Self::Waves => &[],
            Self::Debug => &[],
            Self::Bench => &[Self::Debug],
            Self::LaserSight => &[],
        };
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisplayConfig {
    pub fullscreen: bool,
    pub window_size_x: u16,
    pub window_size_y: u16,
    pub v_sync: bool,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        return Self {
            fullscreen: true,
            window_size_x: 800,
            window_size_y: 800,
            v_sync: false,
        };
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioConfig {
    pub sources: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        return Self { sources: 128 };
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ControlsConfig {
    pub mouse_sensitivity: f32,
}

impl Default for ControlsConfig {
    fn default() -> Self {
        return Self {
            mouse_sensitivity: 0.003,
        };
    }
}

impl Config {
    // TODO: simplify
    // TODO: default if not found
    pub fn load_from(path: &str) -> Result<Self> {
        let global_context = || format!("Path: {}", path);

        let mut config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .context("Failed to load config from file")
            .with_context(global_context)?
            .try_deserialize::<Self>()
            .context("Failed to deserialize config")
            .with_context(global_context)?;

        config.normalize();

        return Ok(config);
    }

    pub fn normalize(&mut self) {
        self.game.modes.dedup();

        if self.game.modes.contains(&GameMode::Bench) {
            *self = Self::default();
            self.game.modes = vec![GameMode::Bench];
            self.display.fullscreen = false;
            self.audio.sources = 0;
        }

        loop {
            let mut modes_with_dependencies = self.game.modes.clone();

            for mode in &self.game.modes {
                for dependency in mode.dependencies() {
                    if !modes_with_dependencies.contains(dependency) {
                        modes_with_dependencies.push(*dependency);
                    }
                }
            }

            if modes_with_dependencies.len() == self.game.modes.len() {
                break;
            } else {
                self.game.modes = modes_with_dependencies;
            }
        }
    }
}

impl DisplayConfig {
    pub fn mode(&self) -> WindowMode {
        if self.fullscreen {
            return WindowMode::Fullscreen;
        } else {
            return WindowMode::Windowed;
        }
    }

    pub fn present_mode(&self) -> PresentMode {
        if self.v_sync {
            return PresentMode::AutoVsync;
        } else {
            return PresentMode::AutoNoVsync;
        }
    }
}
