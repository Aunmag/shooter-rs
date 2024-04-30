use crate::{model::AppState, util::ext::AppExt};
use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{App, Plugin, Query, Res, Time, Transform},
};

pub struct KineticsPlugin;

impl Plugin for KineticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Component)]
pub struct Kinetics {
    pub mass: f32,
    pub drag: f32,
    pub velocity: Vec2,
    pub velocity_angular: f32,
}

impl Kinetics {
    pub const DRAG_DEFAULT: f32 = 300.0;
    pub const DRAG_PLAYER: f32 = 400.0;
    pub const RIGIDITY: f32 = 0.05;

    pub fn new(mass: f32) -> Self {
        assert!(mass > 0.0, "Mass must be greater than zero");

        return Self {
            mass,
            drag: Self::DRAG_DEFAULT,
            velocity: Vec2::new(0.0, 0.0),
            velocity_angular: 0.0,
        };
    }

    pub fn bounce(kinetics_1: &Self, kinetics_2: &Self, relative_angle: Vec2) -> Vec2 {
        let dot = Vec2::dot(kinetics_2.velocity - kinetics_1.velocity, relative_angle);

        if dot < 0.0 && dot.is_finite() {
            return dot
                * kinetics_1.mass
                * kinetics_2.mass
                * (1.0 + Self::RIGIDITY) // f32::min(i1.rigidity, i3.rigidity))
                / (kinetics_1.mass + kinetics_2.mass)
                * relative_angle;
        } else {
            return Vec2::new(0.0, 0.0);
        }
    }

    pub fn push(&mut self, mut push: Vec2, mut spin: f32, with_drag: bool) {
        push /= self.mass;
        spin /= self.mass;

        if with_drag {
            let drag = self.drag();
            push *= drag;
            spin *= drag;
        }

        self.velocity += push;
        self.velocity_angular += spin;
    }

    pub fn drag(&self) -> f32 {
        return self.drag / self.mass;
    }
}

pub fn on_update(mut query: Query<(&mut Transform, &mut Kinetics)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, mut kinetics) in query.iter_mut() {
        transform.translation.x += kinetics.velocity.x * delta;
        transform.translation.y += kinetics.velocity.y * delta;
        transform.rotate_local_z(kinetics.velocity_angular * delta);

        let drag = 1.0 - kinetics.drag() * delta;
        kinetics.velocity *= drag;
        kinetics.velocity_angular *= drag;
    }
}
