use crate::{material::BloodMaterial, util::ext::DurationExt};
use bevy::{
    ecs::system::{Query, ResMut},
    prelude::{Assets, Handle, Res},
    time::Time,
};
use std::time::Duration;

const SPREAD_DURATION: Duration = Duration::from_millis(150);

pub fn blood(
    materials: Query<&Handle<BloodMaterial>>,
    mut assets: ResMut<Assets<BloodMaterial>>,
    time: Res<Time>,
) {
    let time = time.elapsed();

    for handle in materials.iter() {
        if let Some(material) = assets.get_mut(handle) {
            material.spread = time.progress(material.spawned, material.spawned + SPREAD_DURATION);
        }
    }
}
