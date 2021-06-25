use crate::models::GameType;
use crate::resources::EntityConverter;
use crate::resources::GameTask;
use crate::resources::GameTaskResource;
use crate::resources::NetResource;
use crate::resources::PositionUpdateResource;
use crate::resources::Wallpaper;
use crate::states::ui::UiState;
use crate::states::GameState;
use crate::utils;
use crate::utils::Timer;
use crate::utils::WorldExtCustom;
use amethyst::core::timing::Time;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::debug_drawing::DebugLines;
use amethyst::ui::UiEvent;
use amethyst::ui::UiEventType;
use amethyst::ui::UiFinder;
use amethyst::winit::VirtualKeyCode;
use std::time::Duration;

const ROOT_ID: &str = "loading";
const BUTTON_CANCEL_ID: &str = "loading.cancel";
const DOTS_ID: &str = "loading.dots";
const DOTS_INTERVAL: Duration = Duration::from_millis(400);

pub struct LoadingState {
    game_type: GameType,
    root: Option<Entity>,
    button_cancel: Option<Entity>,
    dots_timer: Timer,
    dots_count: u8,
}

impl LoadingState {
    pub const fn new(game_type: GameType) -> Self {
        return Self {
            game_type,
            root: None,
            button_cancel: None,
            dots_timer: Timer::new(DOTS_INTERVAL),
            dots_count: 1,
        };
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.exec(|finder: UiFinder| {
            self.root = finder.find(ROOT_ID);
            self.button_cancel = finder.find(BUTTON_CANCEL_ID);
        });

        self.set_wallpaper(data.world, Wallpaper::Play);
        self.set_visibility(data.world, true);

        data.world.remove::<NetResource>();
        data.world.insert(DebugLines::new());
        data.world.insert(EntityConverter::new());
        data.world.insert(GameTaskResource::new());
        data.world.insert(PositionUpdateResource::new());

        #[allow(clippy::unwrap_used)] // TODO: Resolve
        match self.game_type {
            GameType::Server(port) => {
                data.world.insert(NetResource::new_as_server(port).unwrap());
            }
            GameType::Client(address) => {
                data.world
                    .insert(NetResource::new_as_client(address).unwrap());
            }
        }

        data.world.set_state(Some(self.game_type));
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        self.set_visibility(data.world, false);
        data.world.set_state(None);
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        self.set_visibility(data.world, true);
        data.world.set_state(Some(self.game_type));
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.button_cancel = None;
        self.set_visibility(data.world, false);
        data.world.set_state(None);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let mut is_ready = false;

        match self.game_type {
            GameType::Server(..) => {
                is_ready = true;
            }
            GameType::Client(..) => {
                for task in data.world.read_resource::<GameTaskResource>().iter() {
                    if let GameTask::Start = *task {
                        is_ready = true;
                        break;
                    }
                }
            }
        }

        if is_ready {
            return Trans::Replace(Box::new(GameState::new(self.game_type)));
        }

        let now = data.world.read_resource::<Time>().absolute_real_time();

        if self.dots_timer.next_if_done(now) {
            let dots;

            match self.dots_count {
                1 => {
                    dots = ".".to_string();
                    self.dots_count = 2;
                }
                2 => {
                    dots = ". .".to_string();
                    self.dots_count = 3;
                }
                _ => {
                    dots = ". . .".to_string();
                    self.dots_count = 1;
                }
            }

            utils::ui::set_text(data.world, DOTS_ID, dots);
        }

        return Trans::None;
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
            }
            _ => {}
        }

        return Trans::None;
    }
}

impl UiState for LoadingState {
    fn get_root(&self) -> Option<Entity> {
        return self.root;
    }
}
