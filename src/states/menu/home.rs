use crate::states::menu::QuitState;
use crate::states::menu::UiState;
use crate::states::GameState;
use amethyst::ecs::prelude::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;

pub const ROOT_ID: &str = "home";
pub const BUTTON_CONTINUE_ID: &str = "home.continue";
pub const BUTTON_START_ID: &str = "home.start";
pub const BUTTON_JOIN_ID: &str = "home.join";
pub const BUTTON_SETTINGS_ID: &str = "home.settings";
pub const BUTTON_HELP_ID: &str = "home.help";
pub const BUTTON_QUIT_ID: &str = "home.quit";

pub struct HomeState {
    is_root: bool,
    ui_root: Option<Entity>,
    button_continue: Option<Entity>,
    button_start: Option<Entity>,
    button_help: Option<Entity>,
    button_quit: Option<Entity>,
}

impl HomeState {
    pub fn new(is_root: bool) -> Self {
        return Self {
            is_root,
            ui_root: None,
            button_continue: None,
            button_start: None,
            button_help: None,
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
            self.button_start = finder.find(BUTTON_START_ID);
            self.button_help = finder.find(BUTTON_HELP_ID);
            self.button_quit = finder.find(BUTTON_QUIT_ID);
        });
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.on_stop_or_pause(&mut data.world);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        self.on_start_or_resume(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.button_continue = None;
        self.button_start = None;
        self.button_help = None;
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

                if Some(target) == self.button_start {
                    return Trans::Replace(Box::new(GameState::new()));
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
