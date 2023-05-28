use crate::{
    component::{Actor, Health, Weapon},
    StatusBarMaterial,
};
use bevy::{
    ecs::system::{Query, ResMut},
    prelude::{Assets, Children, Handle, Res},
    time::Time,
};
use std::{f32::consts::TAU, time::Duration};

const INTERPOLATION: f32 = 8.0;
const PULSE: Duration = Duration::from_millis(500);

pub fn status_bar(
    targets: Query<(&Actor, &Health, Option<&Weapon>, &Children)>, // TODO: try to simplify
    handles: Query<&Handle<StatusBarMaterial>>,
    mut assets: ResMut<Assets<StatusBarMaterial>>,
    time: Res<Time>,
) {
    let pulse = (time.elapsed_seconds() * TAU / PULSE.as_secs_f32()).cos() / 2.0 + 0.5;
    let interpolation = f32::min(INTERPOLATION * time.delta().as_secs_f32(), 1.0);

    for (actor, health, weapon, children) in targets.iter() {
        for child in children.iter() {
            if let Some(material) = handles.get(*child).ok().and_then(|h| assets.get_mut(h)) {
                material.health -= (material.health - health.get()) * interpolation;

                if health.is_low() {
                    material.health_alpha = pulse;
                } else {
                    material.health_alpha = 1.0;
                }

                if let Some(weapon) = weapon {
                    material.ammo = weapon.get_ammo_normalized(time.elapsed());

                    if weapon.is_reloading() {
                        material.ammo_alpha = pulse;
                    } else {
                        material.ammo_alpha = 1.0;
                    }
                } else {
                    material.ammo_alpha = 0.0;
                }

                material.stamina = actor.stamina;
            }
        }
    }
}
