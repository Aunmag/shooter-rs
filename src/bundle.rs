use crate::systems::player::PlayerSystem;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::DispatcherBuilder;
use amethyst::ecs::prelude::World;
use amethyst::error::Error;

pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(self, _: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(PlayerSystem, "player_system", &["input_system"]);
        return Ok(());
    }
}
