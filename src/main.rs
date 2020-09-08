#![warn(clippy::all, clippy::cargo, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::as_conversions,
    clippy::cargo_common_metadata,
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::default_trait_access,
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return, // TODO: Enable later excepting closures
    clippy::integer_arithmetic,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_docs_in_private_items,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::needless_return,
    clippy::type_complexity,
    clippy::wildcard_enum_match_arm,
)]

mod components;
mod input;
mod states;
mod systems;
mod utils;

use crate::components::Terrain;
use crate::input::CustomBindingTypes;
use crate::states::StartupState;
use crate::systems::GameEventSystemDesc;
use crate::systems::UiResizeSystem;
use amethyst::controls::CursorHideSystemDesc;
use amethyst::controls::MouseFocusUpdateSystemDesc;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::transform::TransformBundle;
use amethyst::core::HideHierarchySystemDesc;
use amethyst::input::InputBundle;
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

const FRAME_RATE: u32 = 144;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<CustomBindingTypes>::new()
                .with_bindings_from_file(root.join("config/input.ron"))?,
        )?
        .with_system_desc(MouseFocusUpdateSystemDesc::default(), "mouse_focus", &[])
        .with_system_desc(CursorHideSystemDesc::default(), "", &["mouse_focus"])
        .with_system_desc(UiResizeSystem::new(), "", &[])
        .with_system_desc(GameEventSystemDesc::default(), "", &[])
        .with_system_desc(HideHierarchySystemDesc::default(), "", &[]) // TODO: Maybe this system depends on something?
        .with_bundle(UiBundle::<CustomBindingTypes>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(
                    root.join("config/display.ron"),
                )?)
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderTiles2D::<Terrain, MortonEncoder>::default()),
        )?;

    Application::build(root.join("assets/"), StartupState::new())?
        .with_frame_limit(FrameRateLimitStrategy::Yield, FRAME_RATE)
        .build(game_data)?
        .run();

    return Ok(());
}
