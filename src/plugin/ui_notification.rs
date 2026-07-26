use crate::{
    data::{FONT_PATH, FONT_PATH_BOLD},
    plugin::{AudioPlay, AudioTracker},
    util::{Envelope, SmartString},
};
use bevy::{
    app::{App, Plugin, Update},
    color::Alpha,
    ecs::{component::Component, query::With, system::Command, world::World},
    prelude::{AssetServer, Commands, Entity, PositionType, Query, Res},
    text::{JustifyText, TextColor, TextFont, TextLayout, TextSpan},
    time::Time,
    ui::{widget::Text, Node, UiRect, Val},
    window::{PrimaryWindow, Window},
};
use std::time::Duration;

const POSITION: f32 = 0.3;
const FONT_SCALE: f32 = 0.03;
const FADE_IN: Duration = Duration::from_millis(150);
const FADE_OUT: Duration = Duration::from_millis(300);
const DURATION_DEFAULT: Duration = Duration::from_millis(2500);

pub struct UiNotificationPlugin;

impl Plugin for UiNotificationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_update);
    }
}

#[derive(Component)]
struct UiNotification {
    created: Duration,
    envelope: Envelope,
}

impl UiNotification {
    fn new(time: Duration, duration: Duration) -> Self {
        return Self {
            created: time,
            envelope: Envelope::new(FADE_IN, duration, FADE_OUT),
        };
    }

    fn alpha(&self, time: Duration) -> f32 {
        return self.envelope.get(time.saturating_sub(self.created));
    }

    fn is_expired(&self, time: Duration) -> bool {
        return time > self.created + self.envelope.duration();
    }
}

fn on_update(
    mut query: Query<(Entity, &UiNotification, &mut TextColor)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (entity, notification, mut color) in query.iter_mut() {
        color.set_alpha(notification.alpha(time));

        if notification.is_expired(time) {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Default)]
pub struct Notify {
    pub text: SmartString<'static>,
    pub text_small: SmartString<'static>,
    pub duration: Duration,
}

impl Command for Notify {
    fn apply(mut self, world: &mut World) {
        let time = world.resource::<Time>().elapsed();

        if self.duration.is_zero() {
            self.duration = DURATION_DEFAULT;
        }

        let window_width = world
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .iter(world)
            .next()
            .map_or(600.0, |w| w.width());

        let font_bold = world
            .resource::<AssetServer>()
            .get_handle(FONT_PATH_BOLD)
            .unwrap_or_default();

        let font_small = world
            .resource::<AssetServer>()
            .get_handle(FONT_PATH)
            .unwrap_or_default();

        world
            .spawn((
                UiNotification::new(time, self.duration),
                Text::new(format!("{}\n", self.text.as_ref())),
                TextFont {
                    font: font_bold,
                    font_size: window_width * FONT_SCALE,
                    ..Default::default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..Default::default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(POSITION * 100.0),
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                },
            ))
            .with_child((
                UiNotification::new(time, self.duration),
                TextSpan::new(self.text_small.as_ref()),
                TextFont {
                    font: font_small,
                    font_size: window_width * FONT_SCALE / 2.0,
                    ..Default::default()
                },
            ));

        world.resource::<AudioTracker>().queue(AudioPlay {
            path: "sounds/notification".into(),
            volume: 0.8,
            ..AudioPlay::DEFAULT
        });
    }
}
