use anyhow::{Context, Result};
use bevy::{
    ecs::system::Resource,
    window::{PresentMode, WindowMode},
};
use serde::Deserialize;

#[derive(Debug, Default, Clone, Deserialize, Resource)]
#[serde(default)]
pub struct Settings {
    pub game: GameSettings,
    pub display: DisplaySettings,
    pub graphic: GraphicSettings,
    pub audio: AudioSettings,
    pub controls: ControlsSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GameSettings {
    pub difficulty: f32,
    pub modes: Vec<GameMode>,
}

impl Default for GameSettings {
    fn default() -> Self {
        return Self {
            difficulty: 1.0,
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
pub struct DisplaySettings {
    pub fullscreen: bool,
    pub window_size_x: u16,
    pub window_size_y: u16,
    pub v_sync: bool,
}

impl Default for DisplaySettings {
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
pub struct GraphicSettings {
    pub decals: usize,
}

impl Default for GraphicSettings {
    fn default() -> Self {
        return Self { decals: 128 };
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudioSettings {
    pub sources: usize,
}

impl Default for AudioSettings {
    fn default() -> Self {
        return Self { sources: 128 };
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ControlsSettings {
    pub mouse_sensitivity: f32,
}

impl Default for ControlsSettings {
    fn default() -> Self {
        return Self {
            mouse_sensitivity: 0.003,
        };
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let context = "Failed to load settings";
        let mut settings = config::Config::builder()
            .add_source(config::File::with_name("settings.toml"))
            .build()
            .context(context)?
            .try_deserialize::<Self>()
            .context(context)?;

        settings.normalize();

        return Ok(settings);
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

impl DisplaySettings {
    pub fn mode(&self) -> WindowMode {
        if self.fullscreen {
            return WindowMode::BorderlessFullscreen;
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
