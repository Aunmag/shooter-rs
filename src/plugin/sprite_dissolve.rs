use crate::{util::ext::AppExt, AppState};
use bevy::{
    color::Alpha,
    ecs::{component::Component, entity::Entity, query::With, system::Commands},
    prelude::{App, Plugin, Query, Res, Time},
    sprite::Sprite,
};
use std::time::Duration;

const DURATION: Duration = Duration::from_secs(5);

pub struct SpriteDissolvePlugin;

impl Plugin for SpriteDissolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Component)]
pub struct SpriteDissolve;

fn on_update(
    mut query: Query<(Entity, &mut Sprite), With<SpriteDissolve>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let fade = (1.0 / DURATION.as_secs_f64() * time.delta().as_secs_f64()) as f32;

    for (entity, mut sprite) in query.iter_mut() {
        let alpha = sprite.color.alpha() - fade;

        if alpha <= 0.0 {
            commands.entity(entity).despawn();
            sprite.color.set_alpha(0.0);
        } else {
            sprite.color.set_alpha(alpha);
        }
    }
}
