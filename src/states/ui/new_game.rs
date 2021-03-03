use crate::resources::Wallpaper;
use crate::states::game::GameType;
use crate::states::ui::UiState;
use crate::states::GameState;
use crate::utils;
use amethyst::ecs::prelude::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;
use std::net::SocketAddr;

const ROOT_ID: &str = "new_game";
const BUTTON_HOST_ID: &str = "new_game.host";
const BUTTON_JOIN_ID: &str = "new_game.join";
const BUTTON_BACK_ID: &str = "new_game.back";
const INPUT_IP_ID: &str = "new_game.ip";
const INPUT_PORT_ID: &str = "new_game.port";

pub struct NewGameState {
    root: Option<Entity>,
    button_host: Option<Entity>,
    button_join: Option<Entity>,
    button_back: Option<Entity>,
}

impl NewGameState {
    pub fn new() -> Self {
        return Self {
            root: None,
            button_host: None,
            button_join: None,
            button_back: None,
        };
    }

    fn parse_input_ip(world: &World) -> Result<String, &str> {
        if let Some(ip) = utils::ui::fetch_text(world, INPUT_IP_ID) {
            return Ok(ip);
        } else {
            return Err("No IP specified");
        }
    }

    #[allow(clippy::map_err_ignore)]
    fn parse_input_port(world: &World) -> Result<u16, &str> {
        if let Some(port) = utils::ui::fetch_text(world, INPUT_PORT_ID) {
            return port.parse().map_err(|_| "Wrong port");
        } else {
            return Err("No port specified");
        }
    }

    #[allow(clippy::map_err_ignore)]
    fn parse_input_address(world: &World) -> Result<SocketAddr, &str> {
        let ip = Self::parse_input_ip(world)?;
        let port = Self::parse_input_port(world)?;

        return format!("{}:{}", ip, port)
            .parse()
            .map_err(|_| "Wrong address");
    }
}

impl SimpleState for NewGameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.exec(|finder: UiFinder| {
            self.root = finder.find(ROOT_ID);
            self.button_host = finder.find(BUTTON_HOST_ID);
            self.button_join = finder.find(BUTTON_JOIN_ID);
            self.button_back = finder.find(BUTTON_BACK_ID);
        });

        self.set_wallpaper(&data.world, Wallpaper::Play);
        self.set_visibility(&data.world, true);
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        self.set_visibility(&data.world, false);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        self.set_visibility(&data.world, true);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.button_host = None;
        self.button_join = None;
        self.button_back = None;
        self.set_visibility(&data.world, false);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Pop;
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_host {
                    match Self::parse_input_port(&data.world) {
                        Ok(port) => {
                            let game_type = GameType::Host(port);
                            return Trans::Replace(Box::new(GameState::new(game_type)));
                        }
                        Err(error) => {
                            log::error!("{}", error); // TODO: Show error page
                        }
                    }
                }

                if Some(target) == self.button_join {
                    match Self::parse_input_address(&data.world) {
                        Ok(address) => {
                            let game_type = GameType::Join(address);
                            return Trans::Replace(Box::new(GameState::new(game_type)));
                        }
                        Err(error) => {
                            log::error!("{}", error); // TODO: Show error page
                        }
                    }
                }

                if Some(target) == self.button_back {
                    return Trans::Pop;
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for NewGameState {
    fn get_root(&self) -> Option<Entity> {
        return self.root;
    }
}
