use crate::component::{Health, Inertia, Player};
use bevy::{
    ecs::{
        system::{Resource, SystemBuffer, SystemMeta},
        world::World,
    },
    prelude::{Entity, Vec2},
};

const PUSH_MULTIPLIER: f32 = 40.0;
const PUSH_MULTIPLIER_ANGULAR: f32 = 350.0;

#[derive(Default, Resource)]
pub struct HitResource {
    hits: Vec<Hit>,
}

impl HitResource {
    pub fn add(&mut self, entity: Entity, momentum: Vec2, angle: f32, is_recoil: bool) {
        self.hits.push(Hit {
            entity,
            momentum,
            angle,
            is_recoil,
        });
    }
}

impl SystemBuffer for HitResource {
    fn apply(&mut self, _: &SystemMeta, world: &mut World) {
        if self.hits.is_empty() {
            return;
        }

        let mut targets = world.query::<(&mut Inertia, &mut Health)>();
        let mut players = world.query::<&mut Player>();

        for hit in self.hits.drain(..) {
            let mut angle = hit.angle;
            let momentum_linear = hit.momentum.length();

            if !hit.is_recoil {
                angle *= momentum_linear;
                angle *= PUSH_MULTIPLIER_ANGULAR;
            }

            if let Ok((mut inertia, mut health)) = targets.get_mut(world, hit.entity) {
                inertia.push(hit.momentum * PUSH_MULTIPLIER, angle, false);
                health.damage(momentum_linear);
            }

            if let Ok(mut player) = players.get_mut(world, hit.entity) {
                player.shake(angle);
            }
        }
    }
}

struct Hit {
    entity: Entity,
    momentum: Vec2,
    angle: f32,
    is_recoil: bool,
}
