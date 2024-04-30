use crate::plugin::{kinetics::Kinetics, CameraTarget, Health};
use bevy::{
    ecs::{
        system::{Resource, SystemBuffer, SystemMeta},
        world::World,
    },
    prelude::{Entity, Vec2},
};

const PUSH_MULTIPLIER: f32 = 40.0;
const SPIN_MULTIPLIER: f32 = 350.0;

#[derive(Default, Resource)]
pub struct HitResource {
    hits: Vec<Hit>,
}

impl HitResource {
    pub fn add(&mut self, entity: Entity, momentum: Vec2, spin: f32, is_recoil: bool) {
        self.hits.push(Hit {
            entity,
            momentum,
            spin,
            is_recoil,
        });
    }
}

impl SystemBuffer for HitResource {
    fn apply(&mut self, _: &SystemMeta, world: &mut World) {
        if self.hits.is_empty() {
            return;
        }

        let mut targets = world.query::<(&mut Kinetics, &mut Health, Option<&mut CameraTarget>)>();

        for hit in self.hits.drain(..) {
            if let Ok((mut kinetics, mut health, camera)) = targets.get_mut(world, hit.entity) {
                let momentum_linear = hit.momentum.length();
                let mut push = hit.momentum;
                let mut spin = hit.spin * momentum_linear;

                if !hit.is_recoil {
                    push *= PUSH_MULTIPLIER;
                    spin *= SPIN_MULTIPLIER;
                    health.damage(momentum_linear);
                }

                kinetics.push(push, spin, false);

                if let Some(mut camera) = camera {
                    camera.shake(push, spin);
                }
            }
        }
    }
}

struct Hit {
    entity: Entity,
    momentum: Vec2,
    spin: f32,
    is_recoil: bool,
}
