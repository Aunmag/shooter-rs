use crate::resources::GameStatus;
use crate::resources::SpriteResource;
use crate::states::ui::HomeState;
use amethyst::assets::AssetStorage;
use amethyst::assets::Completion;
use amethyst::assets::ProgressCounter;
use amethyst::prelude::*;
use amethyst::renderer::sprite::SpriteSheet;
use amethyst::ui::UiCreator;
use amethyst::window::Window;

const PIXELS_PER_METER: f32 = 32.0;

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
                enable_fullscreen_mode(&data.world);
                resize_sprites(&data.world);
                complete_startup(&data.world);
                return Trans::Switch(Box::new(HomeState::new(true)));
            }
            Completion::Failed => {
                log::error!("Failed to load assets");
                return Trans::Quit;
            }
        }
    }
}

fn enable_fullscreen_mode(world: &World) {
    let window = world.read_resource::<Window>();

    #[allow(clippy::never_loop)]
    for monitor in window.get_available_monitors() {
        window.set_fullscreen(Some(monitor));
        break;
    }
}

fn resize_sprites(world: &World) {
    let mut sprite_sheets = world.write_resource::<AssetStorage<SpriteSheet>>();

    for handle in world.write_resource::<SpriteResource>().data.values() {
        if let Some(sprite_sheet) = sprite_sheets.get_mut(handle) {
            for sprite in sprite_sheet.sprites.iter_mut() {
                sprite.width /= PIXELS_PER_METER;
                sprite.height /= PIXELS_PER_METER;

                for offset in sprite.offsets.iter_mut() {
                    *offset /= PIXELS_PER_METER;
                }
            }
        }
    }
}

fn complete_startup(world: &World) {
    world.write_resource::<GameStatus>().is_loaded = true;
}
