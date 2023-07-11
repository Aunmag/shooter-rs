use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Inertia {
    pub mass: f32,
    pub velocity: Vec2,
    pub velocity_angular: f32,
}

impl Inertia {
    pub const PUSH_MULTIPLIER: f32 = 40.0;
    pub const PUSH_MULTIPLIER_ANGULAR: f32 = 350.0;
    pub const DRAG_FACTOR: f32 = 500.0;

    // TODO: make component's property
    pub const RIGIDITY: f32 = 0.05;

    pub fn new(mass: f32) -> Self {
        assert!(mass > 0.0, "Mass must be greater than zero");

        return Self {
            mass,
            velocity: Vec2::new(0.0, 0.0),
            velocity_angular: 0.0,
        };
    }

    pub fn bounce(inertia_1: &Self, inertia_2: &Self, relative_angle: Vec2) -> Vec2 {
        let dot = Vec2::dot(inertia_2.velocity - inertia_1.velocity, relative_angle);

        if dot < 0.0 && dot.is_finite() {
            return dot
                * inertia_1.mass
                * inertia_2.mass
                * (1.0 + Self::RIGIDITY) // TODO: f32::min(i1.rigidity, i3.rigidity))
                / (inertia_1.mass + inertia_2.mass)
                * relative_angle;
        } else {
            return Vec2::new(0.0, 0.0);
        }
    }

    pub fn push(
        &mut self,
        mut force: Vec2,
        mut force_angular: f32,
        with_drag: bool,
        with_push_multiplier: bool,
    ) {
        let mass_inverse = self.mass_inverse();
        force *= mass_inverse;
        force_angular *= mass_inverse;

        if with_drag {
            let drag = self.drag();
            force *= drag;
            force_angular *= drag;
        }

        if with_push_multiplier {
            force *= Self::PUSH_MULTIPLIER;
            force_angular *= Self::PUSH_MULTIPLIER_ANGULAR;
        }

        self.velocity += force;
        self.velocity_angular += force_angular;
    }

    pub fn mass_inverse(&self) -> f32 {
        return 1.0 / self.mass;
    }

    pub fn drag(&self) -> f32 {
        return self.mass_inverse() * Self::DRAG_FACTOR;
    }
}
