use crate::states::menu;
use crate::states::menu::HomeState;
use crate::states::GameEvent;
use crate::utils::UiAwaiter;
use amethyst::core::shrev::EventChannel;
use amethyst::prelude::*;
use amethyst::ui::UiCreator;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;

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
            {
                // TODO: Do I need this nested scope?
                menu::set_buttons_availability(
                    &[
                        menu::home::BUTTON_JOIN_ID,
                        menu::home::BUTTON_SETTINGS_ID,
                        menu::home::BUTTON_HELP_ID,
                    ],
                    false,
                    &mut data.world.write_storage::<UiTransform>(),
                    &mut data.world.write_storage::<UiText>(),
                );
            }

            data.world
                .write_resource::<EventChannel<GameEvent>>()
                .single_write(GameEvent::GameEnd);

            return Trans::Switch(Box::new(HomeState::new(true)));
        }

        return Trans::None;
    }
}
