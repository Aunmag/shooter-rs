use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use crate::states::menu::UiState;
use crate::states::GameState;
use amethyst::ecs::prelude::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;

const ROOT_ID: &str = "new_game";
const BUTTON_PLAY_SINGLE_ID: &str = "new_game.single";
const BUTTON_JOIN_ID: &str = "new_game.join";
const BUTTON_HOST_ID: &str = "new_game.host";
const BUTTON_BACK_ID: &str = "new_game.back";

pub struct NewGameState {
    ui_root: Option<Entity>,
    button_play_single: Option<Entity>,
    button_back: Option<Entity>,
}

impl NewGameState {
    pub fn new() -> Self {
        return Self {
            ui_root: None,
            button_play_single: None,
            button_back: None,
        };
    }
}

impl SimpleState for NewGameState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.ui_root = self.find_ui_root(&mut data.world);
        self.on_start_or_resume(&mut data.world);

        data.world.exec(|finder: UiFinder| {
            self.button_play_single = finder.find(BUTTON_PLAY_SINGLE_ID);
            self.button_back = finder.find(BUTTON_BACK_ID);
        });

        let mut ui_tasks = data.world.write_resource::<UiTaskResource>();

        ui_tasks.insert(
            BUTTON_JOIN_ID.to_string(),
            UiTask::SetButtonAvailability(false),
        );

        ui_tasks.insert(
            BUTTON_HOST_ID.to_string(),
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
        self.button_play_single = None;
        self.button_back = None;
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
                if Some(target) == self.button_play_single {
                    return Trans::Replace(Box::new(GameState::new()));
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
