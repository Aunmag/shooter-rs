use anyhow::Context;
use anyhow::Result;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use serde::Deserialize;
use std::net::SocketAddr;
use std::time::Duration;

// TODO: logging config

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub display: DisplayConfig,
    pub controls: ControlsConfig,
    pub net: NetConfig,
    pub misc: MiscConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct DisplayConfig {
    pub fullscreen: bool,
    pub window_size_x: f32,
    pub window_size_y: f32,
    pub v_sync: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct NetConfig {
    #[serde(with = "humantime_serde")]
    pub message_resend_interval: Duration, // TODO: detect from ping
    pub server: ServerConfig,
    pub client: ClientConfig,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    #[serde(with = "humantime_serde")]
    pub sync_interval: Duration, // TODO: use Hz
}

#[derive(Clone, Deserialize, Debug)]
pub struct ClientConfig {
    pub join: SocketAddr,
}

#[derive(Clone, Deserialize, Debug)]
pub struct ControlsConfig {
    pub mouse_sensitivity: f32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MiscConfig {
    pub lock_cursor: bool,
    pub show_ghost: bool,
    pub with_stress_test: bool,
}

impl Config {
    // TODO: simplify
    pub fn load_from(path: &str) -> Result<Self> {
        let global_context = || format!("Path: {}", path);

        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .context("Failed to load config from file")
            .with_context(global_context)?
            .try_deserialize::<Self>()
            .context("Failed to deserialize config")
            .with_context(global_context)?;

        return Ok(config);
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
