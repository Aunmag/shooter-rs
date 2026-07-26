use crate::{
    data::{FONT_PATH, FONT_PATH_BOLD},
    plugin::{AudioPlay, AudioTracker},
    util::{Envelope, SmartString},
};
use bevy::{
    app::{App, Plugin, Update},
    color::{palettes::css::WHITE, Alpha},
    ecs::{component::Component, hierarchy::Children, system::Command, world::World},
    prelude::{AssetServer, Commands, Entity, PositionType, Query, Res},
    text::{FontSize, FontWeight, Justify, TextColor, TextFont, TextLayout, TextSpan},
    time::Time,
    ui::{widget::Text, Node, UiRect, Val},
};
use std::time::Duration;

const POSITION: f32 = 0.3;
const FONT_SIZE: FontSize = FontSize::Vw(3.0);
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
    notifications: Query<(Entity, &UiNotification, &Children)>,
    mut colors: Query<&mut TextColor>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for (entity, notification, children) in notifications.iter() {
        let alpha = notification.alpha(time);

        if let Ok(mut color) = colors.get_mut(entity) {
            color.set_alpha(alpha);
        }

        for child in children {
            if let Ok(mut color) = colors.get_mut(*child) {
                color.set_alpha(alpha);
            }
        }

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
    type Out = ();

    fn apply(mut self, world: &mut World) {
        let time = world.resource::<Time>().elapsed();

        if self.duration.is_zero() {
            self.duration = DURATION_DEFAULT;
        }

        let font_bold = world
            .resource::<AssetServer>()
            .get_handle(FONT_PATH_BOLD)
            .unwrap_or_default();

        let font_small = world
            .resource::<AssetServer>()
            .get_handle(FONT_PATH)
            .unwrap_or_default();

        let color = WHITE.with_alpha(0.0);

        world
            .spawn((
                UiNotification::new(time, self.duration),
                Text::new(format!("{}\n", self.text.as_ref())),
                TextColor(color.into()),
                TextFont {
                    font: font_bold.into(),
                    font_size: FONT_SIZE,
                    weight: FontWeight::BOLD,
                    ..Default::default()
                },
                TextLayout {
                    justify: Justify::Center,
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
                TextSpan::new(self.text_small.as_ref()),
                TextColor(color.into()),
                TextFont {
                    font: font_small.into(),
                    font_size: FONT_SIZE * 0.5,
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
