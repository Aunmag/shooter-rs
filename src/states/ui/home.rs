use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use crate::resources::Wallpaper;
use crate::states::ui::ConfirmState;
use crate::states::ui::NewGameState;
use crate::states::ui::UiState;
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
const QUIT_TITLE: &str = "Are you sure you want to quit?";

pub struct HomeState {
    is_root: bool,
    root: Option<Entity>,
    button_continue: Option<Entity>,
    button_new_game: Option<Entity>,
    button_disconnect: Option<Entity>,
    button_quit: Option<Entity>,
}

impl HomeState {
    pub fn new(is_root: bool) -> Self {
        return Self {
            is_root,
            root: None,
            button_continue: None,
            button_new_game: None,
            button_disconnect: None,
            button_quit: None,
        };
    }
}

impl SimpleState for HomeState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.exec(|finder: UiFinder| {
            self.root = finder.find(ROOT_ID);
            self.button_continue = finder.find(BUTTON_CONTINUE_ID);
            self.button_new_game = finder.find(BUTTON_NEW_GAME_ID);
            self.button_disconnect = finder.find(BUTTON_DISCONNECT_ID);
            self.button_quit = finder.find(BUTTON_QUIT_ID);
        });

        if let Some(button_new_game) = self.button_new_game {
            utils::set_entity_visibility(&data.world, button_new_game, self.is_root);
        }

        if let Some(button_disconnect) = self.button_disconnect {
            utils::set_entity_visibility(&data.world, button_disconnect, !self.is_root);
        }

        {
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

        self.set_wallpaper(&data.world, Wallpaper::Home);
        self.set_visibility(&data.world, true);
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        self.set_visibility(&data.world, false);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        self.set_visibility(&data.world, true);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.button_continue = None;
        self.button_new_game = None;
        self.button_disconnect = None;
        self.button_quit = None;
        self.set_visibility(&data.world, false);
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
                    return Trans::Push(Box::new(ConfirmState::new(
                        DISCONNECTION_TITLE,
                        Wallpaper::Home, // TODO: Change
                        || Trans::Replace(Box::new(HomeState::new(true))),
                    )));
                }

                if Some(target) == self.button_quit {
                    return Trans::Push(Box::new(ConfirmState::new(
                        QUIT_TITLE,
                        Wallpaper::Quit,
                        || Trans::Quit,
                    )));
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for HomeState {
    fn get_root(&self) -> Option<Entity> {
        return self.root;
    }
}
