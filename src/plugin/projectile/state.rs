use crate::{
    data::DISTANCE_1MM,
    plugin::{projectile::config::ProjectilePhysics, Projectile, ProjectileConfig},
    util::ext::Vec2Ext,
};
use bevy::math::Vec2;
use std::time::Duration;

/// Allows to find projectile position and velocity at a given time deterministically with minimal
/// floating-point error
pub struct ProjectileState<'a> {
    pub projectile: &'a Projectile,
    traveled: Vec2,
}

impl<'a> ProjectileState<'a> {
    pub fn calc(projectile: &'a Projectile, time: Duration) -> Self {
        let t = time.saturating_sub(projectile.initial_time).as_secs_f32();
        let v = projectile.initial_velocity;

        let mut traveled = match projectile.config.physics {
            ProjectilePhysics::Bullet => {
                let a = projectile.config.acceleration();
                (t * a).exp_m1() * v / a
            }
            ProjectilePhysics::Rocket => calc_rocket_distance(v, t),
        };

        traveled *= ProjectileConfig::VELOCITY_VISUAL_FACTOR;
        traveled = traveled.clamp_length_max(projectile.distance_limit);

        return Self {
            projectile,
            traveled,
        };
    }

    pub fn update_by_traveled_distance(&mut self, distance: Vec2) {
        self.traveled = distance;
    }

    pub fn position(&self) -> Vec2 {
        return self.projectile.initial_position + self.traveled;
    }

    pub fn velocity(&self) -> Vec2 {
        let d = self.traveled / ProjectileConfig::VELOCITY_VISUAL_FACTOR;
        let v = self.projectile.initial_velocity;

        match self.projectile.config.physics {
            ProjectilePhysics::Bullet => return v + d * self.projectile.config.acceleration(),
            ProjectilePhysics::Rocket => {
                return v; // it actually might be slower due to acceleration but it doesn't matter for now
            }
        };
    }

    pub fn stopped(&self) -> bool {
        let distance_limit = self.projectile.distance_limit;
        return self.projectile.stopped
            || distance_limit < DISTANCE_1MM
            || self.traveled.is_long(distance_limit - DISTANCE_1MM)
            || self.velocity().is_short(ProjectileConfig::VELOCITY_MIN);
    }
}

fn calc_rocket_distance(v2: Vec2, t: f32) -> Vec2 {
    let a = ProjectileConfig::ROCKET_ACCELERATION_TIME.as_secs_f32();
    let v1 = v2 / ProjectileConfig::ROCKET_ACCELERATION_FACTOR;

    if t < a {
        return v1 * t + ((v2 - v1) / (2.0 * a) * t.powi(2));
    } else {
        return v2 * t - ((v2 - v1) / 2.0 * a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{plugin::ProjectileConfig, util::test::assert_vec_is_close};

    const SHOT_TIME: Duration = Duration::from_millis(1000);
    const MUZZLE_VELOCITY: f32 = 400.0;

    #[test]
    fn test_bullet() {
        let (t, d) = test_physics(&ProjectileConfig::_5_45X39, f32::INFINITY);
        assert_eq!(t, 735);
        assert_eq!(d, 13.244918);

        let (t, d) = test_physics(&ProjectileConfig::_5_45X39, 5.0);
        assert_eq!(t, 79);
        assert_eq!(d, 5.0);
    }

    #[test]
    fn test_rocket() {
        let (t, d) = test_physics(&ProjectileConfig::TBG_7V, f32::INFINITY);
        assert_eq!(t, 850);
        assert_eq!(d, 40.0);

        let (t, d) = test_physics(&ProjectileConfig::TBG_7V, 5.0);
        assert_eq!(t, 243);
        assert_eq!(d, 5.0);
    }

    /// Returns total travel duration (ms) and traveled distance
    #[must_use]
    fn test_physics(config: &'static ProjectileConfig, distance_limit: f32) -> (u64, f32) {
        let j = Projectile::new(
            config,
            SHOT_TIME,
            Vec2::ZERO,
            Vec2::new(MUZZLE_VELOCITY, 0.0),
            distance_limit,
            None,
        );

        // test state at shot time (0ms passed)
        let s_zero = ProjectileState::calc(&j, j.initial_time);
        assert_eq!(s_zero.traveled, Vec2::ZERO);
        assert_eq!(s_zero.position(), j.initial_position);
        assert_eq!(s_zero.velocity(), j.initial_velocity);

        // test state at past for bugs protection (-n ms passed)
        assert_ne!(j.initial_time, Duration::ZERO); // make sure we have time in past
        let s_past = ProjectileState::calc(&j, Duration::ZERO);
        assert_eq!(s_past.traveled, Vec2::ZERO);
        assert_eq!(s_past.position(), j.initial_position);
        assert_eq!(s_zero.velocity(), j.initial_velocity);

        let mut d_from_v_check = Vec2::ZERO;

        for t in 1..1000 {
            let s = ProjectileState::calc(&j, j.initial_time + Duration::from_millis(t));
            let d = s.traveled;
            let v = s.velocity();

            if s.stopped() {
                // test state at far future for bugs protection
                let s_future = ProjectileState::calc(&j, j.initial_time + Duration::from_hours(24));
                assert_vec_is_close!(s_future.traveled, s.traveled, 0.01);
                assert_vec_is_close!(s_future.position(), s.position(), 0.01);

                if config.physics == ProjectilePhysics::Bullet && distance_limit.is_infinite() {
                    assert_vec_is_close!(s_future.velocity(), Vec2::ZERO, 0.01);
                } else {
                    assert_vec_is_close!(s_future.velocity(), s.velocity(), 0.01);
                }

                return (t, d.length());
            }

            d_from_v_check += v;

            if config.physics != ProjectilePhysics::Rocket {
                // make sure calculated velocity transforms back to traveled distance well (don't work for rockets)
                assert_vec_is_close!(
                    d,
                    d_from_v_check * ProjectileConfig::VELOCITY_VISUAL_FACTOR / 1000.0,
                    0.0025
                );
            }
        }

        panic!("projectile went too far");
    }
}
