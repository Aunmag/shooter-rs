use crate::resources::GameStatus;
use crate::resources::SpriteResource;
use crate::states::ui::HomeState;
use amethyst::assets::Completion;
use amethyst::assets::ProgressCounter;
use amethyst::prelude::*;
use amethyst::ui::UiCreator;
use amethyst::window::Window;

pub struct StartupState {
    progress: ProgressCounter,
}

impl StartupState {
    pub fn new() -> Self {
        return Self {
            progress: ProgressCounter::new(),
        };
    }
}

impl SimpleState for StartupState {
    fn on_start(&mut self, data: StateData<GameData>) {
        data.world.exec(|mut creator: UiCreator| {
            creator.create("ui/confirm.ron", &mut self.progress);
            creator.create("ui/home.ron", &mut self.progress);
            creator.create("ui/new_game.ron", &mut self.progress);
            creator.create("ui/quit.ron", &mut self.progress);
        });

        data.world.insert(SpriteResource::new(&data.world));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Loading => {
                return Trans::None;
            }
            Completion::Complete => {
                let window = data.world.read_resource::<Window>();

                #[allow(clippy::never_loop)]
                for monitor in window.get_available_monitors() {
                    window.set_fullscreen(Some(monitor));
                    break;
                }

                data.world.write_resource::<GameStatus>().is_loaded = true;

                return Trans::Switch(Box::new(HomeState::new(true)));
            }
            Completion::Failed => {
                log::error!("Failed to load assets");
                return Trans::Quit;
            }
        }
    }
}
