use crate::states::menu::home::Home;
use crate::states::menu::UiState;
use amethyst::ecs::prelude::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;

pub const ROOT_ID: &str = "quit";
pub const BUTTON_DISCONNECT_ID: &str = "quit.disconnect";
pub const BUTTON_EXIT_ID: &str = "quit.exit";
pub const BUTTON_CANCEL_ID: &str = "quit.cancel";

pub struct Quit {
    ui_root: Option<Entity>,
    button_disconnect: Option<Entity>,
    button_exit: Option<Entity>,
    button_cancel: Option<Entity>,
}

impl Quit {
    pub fn new() -> Self {
        return Self {
            ui_root: None,
            button_disconnect: None,
            button_exit: None,
            button_cancel: None,
        };
    }
}

impl SimpleState for Quit {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.ui_root = self.find_ui_root(&mut data.world);
        self.on_start_or_resume(&mut data.world);

        data.world.exec(|finder: UiFinder| {
            self.button_disconnect = finder.find(BUTTON_DISCONNECT_ID);
            self.button_exit = finder.find(BUTTON_EXIT_ID);
            self.button_cancel = finder.find(BUTTON_CANCEL_ID);
        });
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.button_disconnect = None;
        self.button_exit = None;
        self.button_cancel = None;
        self.on_stop_or_pause(&mut data.world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
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
                if Some(target) == self.button_cancel {
                    return Trans::Pop;
                }

                if Some(target) == self.button_disconnect {
                    return Trans::Replace(Box::new(Home::new(true)));
                }

                if Some(target) == self.button_exit {
                    return Trans::Quit;
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for Quit {
    fn get_ui_root_id(&self) -> &'static str {
        return ROOT_ID;
    }

    fn get_ui_root(&self) -> Option<Entity> {
        return self.ui_root;
    }
}
