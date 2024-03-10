use super::TileBlend;
use crate::{
    component::Inertia,
    model::AppState,
    resource::AssetGroups,
    util::ext::{AppExt, RngExt, TransformExt},
};
use bevy::{
    app::{App, Plugin},
    asset::Handle,
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, ResMut},
    },
    math::Quat,
    prelude::Res,
    render::texture::Image,
    time::Time,
    transform::components::Transform,
};

// TODO:
// - trim images
// - animate blood on death

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Component)]
pub struct Animation {
    config: &'static AnimationConfig,
    frames: f32,
    frame: f32,
    speed_saved: f32,
}

impl Animation {
    pub fn update(&mut self, speed: f32, delta: f32) -> Option<u8> {
        let frame_i_old = self.frame.trunc();

        if self.frame.is_nan() {
            self.frame = 0.0;
        }

        self.frame += speed * self.config.speed * delta * self.frames;

        if !self.config.is_final {
            self.frame %= self.frames;
        } else if self.frame > self.frames {
            self.frame = f32::max(0.0, self.frames - 1.0);
        }

        if self.frame < 0.0 {
            self.frame = self.frames - self.frame;
        }

        let frame_i_new = self.frame.trunc();

        if frame_i_new != frame_i_old {
            return Some(frame_i_new as u8);
        } else {
            return None;
        }
    }

    pub fn is_ended(&self) -> bool {
        return self.config.is_final && self.frame + 1.0 >= self.frames;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationId {
    ZombieWalk,
    ZombieDeath,
}

pub struct AnimationConfig {
    id: AnimationId,
    path: &'static str,
    speed: f32,
    is_final: bool,
}

impl AnimationConfig {
    pub const ZOMBIE_WALK: Self = Self {
        id: AnimationId::ZombieWalk,
        path: "actors/zombie/image/walk",
        speed: 0.8,
        is_final: false,
    };

    pub const ZOMBIE_DEATH: Self = Self {
        id: AnimationId::ZombieDeath,
        path: "actors/zombie/image/death",
        speed: 1.3,
        is_final: true,
    };

    pub fn to_component(&'static self) -> Animation {
        return Animation {
            config: self,
            frames: 0.0,
            frame: f32::NAN,
            speed_saved: f32::NAN,
        };
    }
}

fn on_update(
    mut query: Query<(
        Entity,
        &Transform,
        &Inertia,
        &mut Animation,
        &mut Handle<Image>,
    )>,
    mut images: ResMut<AssetGroups<Image>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (entity, transform, inertia, mut animation, mut image_current) in query.iter_mut() {
        let speed = match animation.config.id {
            AnimationId::ZombieWalk => {
                (Quat::from_rotation_z(-transform.direction()) * inertia.velocity.extend(0.0)).x
            }
            AnimationId::ZombieDeath => {
                if animation.speed_saved.is_nan() {
                    animation.speed_saved = rand::thread_rng().fuzz(animation.config.speed);
                }

                animation.speed_saved
            }
        };

        let group = images.get_group(animation.config.path);

        if let Some(group) = group.as_ref() {
            animation.frames = group.len() as f32;
        }

        if let Some(frame) = animation.update(speed, delta) {
            if let Some(image_next) = group.and_then(|g| g.get(usize::from(frame))) {
                *image_current = image_next;
            }
        }

        if animation.is_ended() {
            commands.entity(entity).remove::<Animation>();
            commands.add(TileBlend::Entity(entity)); // TODO: test
        }
    }
}
