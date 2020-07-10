use crate::systems::player::PlayerSystem;
use amethyst::controls::CursorHideSystemDesc;
use amethyst::controls::MouseFocusUpdateSystemDesc;
use amethyst::core::bundle::SystemBundle;
use amethyst::core::SystemDesc;
use amethyst::ecs::prelude::DispatcherBuilder;
use amethyst::ecs::prelude::World;
use amethyst::error::Error;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(PlayerSystem, "player_system", &["input_system"]);

        builder.add(
            MouseFocusUpdateSystemDesc::default().build(world),
            "mouse_focus",
            &[],
        );

        builder.add(
            CursorHideSystemDesc::default().build(world),
            "cursor_hide",
            &["mouse_focus"],
        );

        return Ok(());
    }
}
