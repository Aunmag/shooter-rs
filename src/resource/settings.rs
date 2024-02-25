use anyhow::{Context, Result};
use bevy::{
    ecs::system::Resource,
    window::{PresentMode, WindowMode},
};
use serde::{Deserialize, Serialize};

const FILE: &str = "settings.toml";

#[derive(Debug, Default, Clone, Serialize, Deserialize, Resource)]
#[serde(default)]
pub struct Settings {
    pub game: GameSettings,
    pub display: DisplaySettings,
    pub audio: AudioSettings,
    pub controls: ControlsSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    /// 0.8 - easy, 1.0 - medium, 1.2 - hard
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub full_screen: bool,
    pub window_size_x: u16,
    pub window_size_y: u16,
    pub v_sync: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        return Self {
            full_screen: true,
            window_size_x: 800,
            window_size_y: 800,
            v_sync: false,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub sources: usize,
}

impl Default for AudioSettings {
    fn default() -> Self {
        return Self { sources: 128 };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        log::info!("Loading settings...");
        let context = "Failed to load settings";
        let encoded = std::fs::read_to_string(FILE).context(context)?;
        let mut settings: Self = toml::from_str(&encoded).context(context)?;
        settings.normalize();
        log::info!("Settings loaded: {:?}", settings);
        return Ok(settings);
    }

    pub fn load_or_default() -> Self {
        return Self::load().unwrap_or_else(|error| {
            log::error!("{:?}", error);
            log::warn!("Default settings will be used");
            let settings = Settings::default();
            settings.clone().save_in_background();
            settings
        });
    }

    pub fn save(&self) -> Result<()> {
        log::info!("Saving settings...");
        let context = "Failed to save settings";
        let encoded = toml::to_string_pretty(&self).context(context)?;
        std::fs::write(FILE, encoded).context(context)?;
        log::info!("Settings saved");
        return Ok(());
    }

    pub fn save_in_background(self) {
        std::thread::spawn(move || {
            if let Err(error) = self.save() {
                log::error!("{:?}", error);
            }
        });
    }

    pub fn normalize(&mut self) {
        self.game.modes.dedup();

        if self.game.modes.contains(&GameMode::Bench) {
            *self = Self::default();
            self.game.modes = vec![GameMode::Bench];
            self.display.full_screen = false;
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
        if self.full_screen {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_normalized() {
        let mut normalized = Settings::default();
        normalized.normalize();

        assert_eq!(
            format!("{:?}", normalized),
            format!("{:?}", Settings::default()),
        );
    }
}
