use crate::states::menu::HomeState;
use crate::utils::UiAwaiter;
use amethyst::prelude::*;
use amethyst::ui::UiCreator;
use amethyst::window::Window;

pub struct StartupState {
    ui_awaiter: UiAwaiter,
}

impl StartupState {
    pub fn new() -> Self {
        return Self {
            ui_awaiter: UiAwaiter::new(),
        };
    }
}

impl SimpleState for StartupState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.exec(|mut creator: UiCreator| {
            creator.create("ui/confirm.ron", ());
            creator.create("ui/home.ron", ());
            creator.create("ui/new_game.ron", ());
            creator.create("ui/quit.ron", ());
        });
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.ui_awaiter.update();

        if self.ui_awaiter.is_ready() {
            let window = data.world.read_resource::<Window>();

            #[allow(clippy::never_loop)]
            for monitor in window.get_available_monitors() {
                window.set_fullscreen(Some(monitor));
                break;
            }

            return Trans::Switch(Box::new(HomeState::new(true)));
        }

        return Trans::None;
    }
}
