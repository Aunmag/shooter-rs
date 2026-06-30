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
    pub scenario: ScenarioSettings,
    /// 0.8 - easy, 1.0 - medium, 1.2 - hard
    pub difficulty: f32,
    pub level: u8,
    pub debug: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        return Self {
            scenario: ScenarioSettings::Waves,
            difficulty: 1.0,
            level: 1,
            debug: false,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioSettings {
    BenchProjectiles,
    BenchZombies,
    Test,
    TestBotSpread,
    Waves,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub mode: WindowModeSettings,
    pub window_w: u16,
    pub window_h: u16,
    pub v_sync: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        return Self {
            mode: WindowModeSettings::Borderless,
            window_w: 800,
            window_h: 800,
            v_sync: false,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowModeSettings {
    Fullscreen,
    Borderless,
    Windowed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub sources: usize,
}

impl Default for AudioSettings {
    fn default() -> Self {
        return Self { sources: 48 };
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
        let settings = toml::from_str(&encoded).context(context)?;
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
}

impl DisplaySettings {
    pub fn mode(&self) -> WindowMode {
        match self.mode {
            WindowModeSettings::Fullscreen => {
                return WindowMode::Fullscreen;
            }
            WindowModeSettings::Borderless => {
                return WindowMode::BorderlessFullscreen;
            }
            WindowModeSettings::Windowed => {
                return WindowMode::Windowed;
            }
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
    #[allow(clippy::unwrap_used)]
    fn test_default_and_actual() {
        let default = format!("{:?}", Settings::default());
        let actual = format!("{:?}", Settings::load().unwrap());
        assert_eq!(default, actual);
    }
}
