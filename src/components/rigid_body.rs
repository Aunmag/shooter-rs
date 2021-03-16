use amethyst::core::math::Vector2;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;

pub struct RigidBody {
    pub mass: f32,
    pub drag: f32,
    pub drag_angular: f32,
    pub rigidity: f32,
    pub velocity: Vector2<f32>,
    pub spinning: f32,
}

impl RigidBody {
    pub fn new(mass: f32, drag: f32, drag_angular: f32, rigidity: f32) -> Self {
        return Self {
            mass,
            drag,
            drag_angular,
            rigidity,
            velocity: Vector2::new(0.0, 0.0),
            spinning: 0.0,
        };
    }

    pub fn bounce(body_1: &Self, body_2: &Self, relative_angle: Vector2<f32>) -> Vector2<f32> {
        let dot = Vector2::dot(&(body_2.velocity - body_1.velocity), &relative_angle);

        if dot < 0.0 && dot.is_finite() {
            return dot
                * body_1.mass
                * body_2.mass
                * (1.0 + f32::min(body_1.rigidity, body_2.rigidity))
                / (body_1.mass + body_2.mass)
                * relative_angle;
        } else {
            return Vector2::new(0.0, 0.0);
        }
    }

    pub fn push(
        &mut self,
        mut x: f32,
        mut y: f32,
        mut spinning: f32,
        using_mass: bool,
        using_drag: bool,
    ) {
        if using_mass {
            let inverse_mass = self.get_inverse_mass();
            x *= inverse_mass;
            y *= inverse_mass;
            spinning *= inverse_mass;
        }

        if using_drag {
            x *= self.drag;
            y *= self.drag;
            spinning *= self.drag_angular;
        }

        self.velocity.x += x;
        self.velocity.y += y;
        self.spinning += spinning;
    }

    pub fn get_inverse_mass(&self) -> f32 {
        if self.mass == 0.0 {
            return 0.0;
        } else {
            return 1.0 / self.mass;
        }
    }
}

impl Component for RigidBody {
    type Storage = DenseVecStorage<Self>;
}
