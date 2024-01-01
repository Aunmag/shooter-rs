use crate::{material::BloodMaterial, util::ext::DurationExt};
use bevy::{
    ecs::system::ResMut,
    prelude::{Assets, Res},
    time::Time,
};
use std::time::Duration;

const SPREAD_DURATION: Duration = Duration::from_millis(150);

pub fn blood(mut materials: ResMut<Assets<BloodMaterial>>, time: Res<Time>) {
    let time = time.elapsed();

    for (_, material) in materials.iter_mut() {
        material.spread = time.progress(material.spawned, material.spawned + SPREAD_DURATION);
    }
}
