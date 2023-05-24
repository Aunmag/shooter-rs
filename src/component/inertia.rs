use bevy::ecs::component::Component;
use bevy::math::Vec2;

#[derive(Component)]
pub struct Inertia {
    pub mass: f32,
    pub velocity: Vec2,
    pub velocity_angular: f32,
}

impl Inertia {
    pub const PUSH_MULTIPLIER: f32 = 30.0;
    pub const PUSH_MULTIPLIER_ANGULAR: f32 = 200.0;

    // TODO: make these component's properties
    pub const DRAG: f32 = 7.0;
    pub const DRAG_ANGULAR: f32 = 8.0;
    pub const RIGIDITY: f32 = 0.05;

    pub fn new(mass: f32) -> Self {
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
        with_mass: bool,
        with_drag: bool,
        with_push_multiplier: bool,
    ) {
        if with_mass {
            let inverse_mass = self.get_inverse_mass();
            force *= inverse_mass;
            force_angular *= inverse_mass;
        }

        if with_drag {
            force *= Self::DRAG;
            force_angular *= Self::DRAG_ANGULAR;
        }

        if with_push_multiplier {
            force *= Self::PUSH_MULTIPLIER;
            force_angular *= Self::PUSH_MULTIPLIER_ANGULAR;
        }

        self.velocity += force;
        self.velocity_angular += force_angular;
    }

    pub fn get_inverse_mass(&self) -> f32 {
        if self.mass == 0.0 {
            return 0.0;
        } else {
            return 1.0 / self.mass;
        }
    }
}
