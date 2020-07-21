use crate::states::game::GameEvent;
use crate::states::menu;
use amethyst::core::shrev::EventChannel;
use amethyst::core::shrev::ReaderId;
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::WriteStorage;
use amethyst::ecs::Read;
use amethyst::ecs::System;
use amethyst::ecs::SystemData;
use amethyst::ui::UiText;
use amethyst::ui::UiTransform;

#[derive(SystemDesc)]
#[system_desc(name(GameEventSystemDesc))]
pub struct GameEventSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<GameEvent>,
}

impl GameEventSystem {
    pub fn new(reader: ReaderId<GameEvent>) -> Self {
        return GameEventSystem { reader };
    }
}

impl<'a> System<'a> for GameEventSystem {
    type SystemData = (
        Read<'a, EventChannel<GameEvent>>,
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
    );

    fn run(&mut self, (events, mut texts, mut transforms): Self::SystemData) {
        let mut has_game = None;

        for event in events.read(&mut self.reader) {
            match event {
                GameEvent::GameStart => {
                    has_game = Some(true);
                }
                GameEvent::GameEnd => {
                    has_game = Some(false);
                }
            }
        }

        if let Some(has_game) = has_game {
            menu::set_buttons_availability(
                &[
                    menu::home::BUTTON_CONTINUE_ID,
                    menu::quit::BUTTON_DISCONNECT_ID,
                ],
                has_game,
                &mut transforms,
                &mut texts,
            );
        }
    }
}
