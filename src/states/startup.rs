use crate::resources::UiTask;
use crate::resources::UiTaskResource;
use crate::states::menu;
use crate::states::menu::HomeState;
use crate::utils::UiAwaiter;
use amethyst::prelude::*;
use amethyst::ui::UiCreator;

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
            creator.create("ui/home.ron", ());
            creator.create("ui/quit.ron", ());
        });
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.ui_awaiter.update();

        if self.ui_awaiter.is_ready() {
            let mut tasks = data.world.write_resource::<UiTaskResource>();

            tasks.push(UiTask::SetButtonAvailability(
                menu::home::BUTTON_CONTINUE_ID,
                false,
            ));

            tasks.push(UiTask::SetButtonAvailability(
                menu::home::BUTTON_HELP_ID,
                false,
            ));

            tasks.push(UiTask::SetButtonAvailability(
                menu::home::BUTTON_JOIN_ID,
                false,
            ));

            tasks.push(UiTask::SetButtonAvailability(
                menu::home::BUTTON_SETTINGS_ID,
                false,
            ));

            tasks.push(UiTask::SetButtonAvailability(
                menu::quit::BUTTON_DISCONNECT_ID,
                false,
            ));

            return Trans::Switch(Box::new(HomeState::new(true)));
        }

        return Trans::None;
    }
}
