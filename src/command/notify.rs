use crate::{
    component::Notification,
    data::{FONT_PATH, FONT_PATH_BOLD},
    model::AudioPlay,
    resource::AudioTracker,
    util::SmartString,
};
use bevy::{
    ecs::{query::With, system::Command, world::World},
    prelude::{AssetServer, Color, PositionType},
    text::{TextAlignment, TextSection, TextStyle},
    time::Time,
    ui::{node_bundles::TextBundle, Style, UiRect, Val},
    window::{PrimaryWindow, Window},
};
use std::time::Duration;

const POSITION: f32 = 0.3;
const FONT_SCALE: f32 = 0.04;
const COLOR: Color = Color::WHITE;

#[derive(Default)]
pub struct Notify {
    pub text: SmartString<'static>,
    pub text_small: SmartString<'static>,
    pub duration: Duration,
}

impl Command for Notify {
    fn apply(self, world: &mut World) {
        let time = world.resource::<Time>().elapsed();

        let window_width = world
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .iter(world)
            .next()
            .map_or(600.0, |w| w.width());

        let asset_server = world.resource::<AssetServer>();

        let notification = if self.duration.is_zero() {
            Notification::new(time)
        } else {
            Notification::new_with_duration(time, self.duration)
        };

        world
            .spawn(
                TextBundle::from_sections([
                    TextSection::new(
                        format!("{}\n", self.text.as_ref()),
                        TextStyle {
                            font: asset_server.get_handle(FONT_PATH_BOLD),
                            font_size: window_width * FONT_SCALE,
                            color: COLOR,
                        },
                    ),
                    TextSection::new(
                        self.text_small.as_ref(),
                        TextStyle {
                            font: asset_server.get_handle(FONT_PATH),
                            font_size: window_width * FONT_SCALE / 2.0,
                            color: COLOR,
                        },
                    ),
                ])
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(POSITION * 100.0),
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                }),
            )
            .insert(notification);

        world.resource_mut::<AudioTracker>().queue(AudioPlay {
            path: "sounds/notification".into(),
            volume: 0.8,
            ..AudioPlay::DEFAULT
        });
    }
}
