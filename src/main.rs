mod bundle;
mod components;
mod states;
mod systems;
mod utils;

use crate::bundle::GameBundle;
use crate::states::game::ExampleTile;
use crate::states::game::Game;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::input::StringBindings;
use amethyst::prelude::*;
use amethyst::renderer::plugins::RenderFlat2D;
use amethyst::renderer::plugins::RenderToWindow;
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::tiles::MortonEncoder;
use amethyst::tiles::RenderTiles2D;
use amethyst::ui::RenderUi;
use amethyst::ui::UiBundle;
use amethyst::utils::application_root_dir;
use std::time::Duration;

const ARENA_SIZE: f32 = 100.0;
const FRAME_RATE: u32 = 144;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // TODO: `dunce::simplified` is a temporary workaround, remove in the future (see https://github.com/amethyst/amethyst/pull/2337#issue-438871838)
    let root = dunce::simplified(&application_root_dir()?).to_path_buf();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(
                root.join("config/input.ron"),
            )?,
        )?
        .with_bundle(GameBundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(root.join("config/display.ron"))?)
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderTiles2D::<ExampleTile, MortonEncoder>::default()),
        )?;

    Application::build(root.join("assets/"), Game::default())?
        .with_frame_limit(
            // TODO: Learn more
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_RATE,
        )
        .build(game_data)?
        .run();

    return Ok(());
}
