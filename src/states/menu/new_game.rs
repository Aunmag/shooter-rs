use crate::states::game::GameType;
use crate::states::menu::UiState;
use crate::states::GameState;
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::Join;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;
use amethyst::winit::VirtualKeyCode;
use std::net::SocketAddr;

const ROOT_ID: &str = "new_game";
const BUTTON_PLAY_SINGLE_ID: &str = "new_game.single";
const BUTTON_JOIN_ID: &str = "new_game.join";
const BUTTON_HOST_ID: &str = "new_game.host";
const BUTTON_BACK_ID: &str = "new_game.back";
const INPUT_IP_ID: &str = "new_game.ip";
const INPUT_PORT_ID: &str = "new_game.port";

pub struct NewGameState {
    ui_root: Option<Entity>,
    button_play_single: Option<Entity>,
    button_join: Option<Entity>,
    button_host: Option<Entity>,
    button_back: Option<Entity>,
}

impl NewGameState {
    pub fn new() -> Self {
        return Self {
            ui_root: None,
            button_play_single: None,
            button_join: None,
            button_host: None,
            button_back: None,
        };
    }

    fn parse_input_address(world: &World) -> Result<SocketAddr, String> {
        let mut ip = None;
        let mut port = None;

        for (ui_transform, ui_text) in (
            &world.read_storage::<UiTransform>(),
            &world.read_storage::<UiText>(),
        )
            .join()
        {
            #[allow(clippy::pattern_type_mismatch)] // TODO: Resolve
            match ui_transform.id.as_str() {
                INPUT_IP_ID => {
                    ip = Some(ui_text.text.clone());
                }
                INPUT_PORT_ID => {
                    port = Some(ui_text.text.clone());
                }
                _ => {}
            }

            if ip.is_some() && port.is_some() {
                break;
            }
        }

        let ip = ip
            .filter(|v| v != "")
            .ok_or_else(|| "No IP specified".to_string())?;

        let port = port
            .filter(|v| v != "")
            .ok_or_else(|| "No port specified".to_string())?;

        return format!("{}:{}", ip, port)
            .parse()
            .map_err(|e| format!("{}", e));
    }

    fn parse_input_port(world: &World) -> Result<u16, String> {
        for (ui_transform, ui_text) in (
            &world.read_storage::<UiTransform>(),
            &world.read_storage::<UiText>(),
        )
            .join()
        {
            if ui_transform.id == INPUT_PORT_ID {
                return ui_text
                    .text
                    .parse()
                    .map_err(|_| "Wrong server port".to_string());
            }
        }

        return Err("Failed to set port".to_string());
    }
}

impl SimpleState for NewGameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.ui_root = self.find_ui_root(&mut data.world);
        self.on_start_or_resume(&mut data.world);

        data.world.exec(|finder: UiFinder| {
            self.button_play_single = finder.find(BUTTON_PLAY_SINGLE_ID);
            self.button_join = finder.find(BUTTON_JOIN_ID);
            self.button_host = finder.find(BUTTON_HOST_ID);
            self.button_back = finder.find(BUTTON_BACK_ID);
        });
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.on_stop_or_pause(&mut data.world);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.on_start_or_resume(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.button_play_single = None;
        self.button_join = None;
        self.button_host = None;
        self.button_back = None;
        self.on_stop_or_pause(&mut data.world);
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
                if Some(target) == self.button_play_single {
                    return Trans::Replace(Box::new(GameState::new(GameType::Single)));
                }

                if Some(target) == self.button_join {
                    let address = Self::parse_input_address(&data.world).unwrap();
                    return Trans::Replace(Box::new(GameState::new(GameType::Join(address))));
                }

                if Some(target) == self.button_host {
                    let port = Self::parse_input_port(&data.world).unwrap();
                    return Trans::Replace(Box::new(GameState::new(GameType::Host(port))));
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
    fn get_ui_root_id(&self) -> &'static str {
        return ROOT_ID;
    }

    fn get_ui_root(&self) -> Option<Entity> {
        return self.ui_root;
    }
}
