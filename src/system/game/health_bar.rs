use crate::{component::Health, HealthBarMaterial};
use bevy::{
    ecs::system::{Query, ResMut},
    prelude::{Assets, Children, Handle, Res},
    time::Time,
};
use std::{f32::consts::TAU, time::Duration};

const INTERPOLATION: f32 = 8.0;
const PULSE: Duration = Duration::from_millis(500);
const ALPHA: f32 = 0.6;
const HEALTH_LOW: f32 = 0.4;

pub fn health_bar(
    healths: Query<(&Health, &Children)>,
    handles: Query<&Handle<HealthBarMaterial>>,
    mut assets: ResMut<Assets<HealthBarMaterial>>,
    time: Res<Time>,
) {
    let pulse = (time.elapsed_seconds() * TAU / PULSE.as_secs_f32()).cos() / 2.0 + 0.5;
    let interpolation = f32::min(INTERPOLATION * time.delta().as_secs_f32(), 1.0);

    for (health, children) in healths.iter() {
        for child in children.iter() {
            if let Some(material) = handles.get(*child).ok().and_then(|h| assets.get_mut(h)) {
                material.value -= (material.value - health.get()) * interpolation;

                if material.value > HEALTH_LOW {
                    material.color.set_a(ALPHA);
                } else {
                    material.color.set_a(ALPHA * pulse);
                }
            }
        }
    }
}
