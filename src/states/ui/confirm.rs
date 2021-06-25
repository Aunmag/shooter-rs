use crate::resources::Wallpaper;
use crate::states::ui::UiState;
use crate::utils;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;

const ROOT_ID: &str = "confirm";
const LABEL_TITLE_ID: &str = "confirm.title";
const BUTTON_YES_ID: &str = "confirm.yes";
const BUTTON_NO_ID: &str = "confirm.no";

pub struct ConfirmState {
    title: &'static str,
    root: Option<Entity>,
    button_yes: Option<Entity>,
    button_no: Option<Entity>,
    wallpaper: Wallpaper,
    on_confirm: fn() -> SimpleTrans,
}

impl ConfirmState {
    pub fn new(title: &'static str, wallpaper: Wallpaper, on_confirm: fn() -> SimpleTrans) -> Self {
        return Self {
            title,
            root: None,
            button_yes: None,
            button_no: None,
            wallpaper,
            on_confirm,
        };
    }
}

impl SimpleState for ConfirmState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.exec(|finder: UiFinder| {
            self.root = finder.find(ROOT_ID);
            self.button_yes = finder.find(BUTTON_YES_ID);
            self.button_no = finder.find(BUTTON_NO_ID);
        });

        utils::ui::set_text(data.world, LABEL_TITLE_ID, self.title.to_string());

        self.set_wallpaper(data.world, self.wallpaper);
        self.set_visibility(data.world, true);
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        self.set_visibility(data.world, false);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        self.set_visibility(data.world, true);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.button_yes = None;
        self.button_no = None;
        self.set_visibility(data.world, false);
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
                if Some(target) == self.button_yes {
                    return (self.on_confirm)();
                }

                if Some(target) == self.button_no {
                    return Trans::Pop;
                }
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for ConfirmState {
    fn get_root(&self) -> Option<Entity> {
        return self.root;
    }
}
