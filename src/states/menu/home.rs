use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use crate::states::menu::ConfirmState;
use crate::states::menu::NewGameState;
use crate::states::menu::QuitState;
use crate::states::menu::UiState;
use crate::utils;
use amethyst::ecs::prelude::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;

const ROOT_ID: &str = "home";
const BUTTON_CONTINUE_ID: &str = "home.continue";
const BUTTON_NEW_GAME_ID: &str = "home.new_game";
const BUTTON_DISCONNECT_ID: &str = "home.disconnect";
const BUTTON_SETTINGS_ID: &str = "home.settings";
const BUTTON_HELP_ID: &str = "home.help";
const BUTTON_QUIT_ID: &str = "home.quit";
const DISCONNECTION_TITLE: &str = "Are you sure you want to disconnect?";

pub struct HomeState {
    is_root: bool,
    ui_root: Option<Entity>,
    button_continue: Option<Entity>,
    button_new_game: Option<Entity>,
    button_disconnect: Option<Entity>,
    button_quit: Option<Entity>,
}

impl HomeState {
    pub fn new(is_root: bool) -> Self {
        return Self {
            is_root,
            ui_root: None,
            button_continue: None,
            button_new_game: None,
            button_disconnect: None,
            button_quit: None,
        };
    }
}

impl SimpleState for HomeState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.ui_root = self.find_ui_root(&mut data.world);
        self.on_start_or_resume(&mut data.world);

        data.world.exec(|finder: UiFinder| {
            self.button_continue = finder.find(BUTTON_CONTINUE_ID);
            self.button_new_game = finder.find(BUTTON_NEW_GAME_ID);
            self.button_disconnect = finder.find(BUTTON_DISCONNECT_ID);
            self.button_quit = finder.find(BUTTON_QUIT_ID);
        });

        if let Some(button_new_game) = self.button_new_game {
            utils::set_entity_visibility(button_new_game, &mut data.world, self.is_root);
        }

        if let Some(button_disconnect) = self.button_disconnect {
            utils::set_entity_visibility(button_disconnect, &mut data.world, !self.is_root);
        }

        let mut ui_tasks = data.world.write_resource::<UiTaskResource>();

        ui_tasks.insert(
            BUTTON_CONTINUE_ID.to_string(),
            UiTask::SetButtonAvailability(!self.is_root),
        );

        ui_tasks.insert(
            BUTTON_HELP_ID.to_string(),
            UiTask::SetButtonAvailability(false),
        );

        ui_tasks.insert(
            BUTTON_SETTINGS_ID.to_string(),
            UiTask::SetButtonAvailability(false),
        );
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.on_stop_or_pause(&mut data.world);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.on_start_or_resume(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.button_continue = None;
        self.button_new_game = None;
        self.button_disconnect = None;
        self.button_quit = None;
        self.on_stop_or_pause(&mut data.world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if !self.is_root && is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Pop;
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_continue {
                    return Trans::Pop;
                }

                if Some(target) == self.button_new_game {
                    return Trans::Push(Box::new(NewGameState::new()));
                }

                if Some(target) == self.button_disconnect {
                    return Trans::Push(Box::new(ConfirmState::new(DISCONNECTION_TITLE, || {
                        Trans::Replace(Box::new(HomeState::new(true)))
                    })));
                }

                if Some(target) == self.button_quit {
                    return Trans::Push(Box::new(QuitState::new()));
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for HomeState {
    fn get_ui_root_id(&self) -> &'static str {
        return ROOT_ID;
    }

    fn get_ui_root(&self) -> Option<Entity> {
        return self.ui_root;
    }
}
