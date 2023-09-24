use crate::{data::PIXELS_PER_METER, resource::Misc, LaserMaterial};
use bevy::{
    asset::Assets,
    ecs::system::Command,
    prelude::{BuildWorldChildren, Entity, Quat, Transform, Vec3, World},
    sprite::MaterialMesh2dBundle,
};
use std::f32::consts::PI;

const LENGTH: f32 = 26.0 * PIXELS_PER_METER;
const THICKNESS: f32 = 0.5 * PIXELS_PER_METER;

pub struct LaserSightSet(pub Entity);

impl Command for LaserSightSet {
    fn apply(self, world: &mut World) {
        let misc = world.resource::<Misc>();

        let image = if let Some(image) = misc.dummy_image.clone() {
            image
        } else {
            log::warn!("Failed to set laser sight. The dummy image isn't initialized");
            return;
        };

        let mesh = if let Some(mesh) = misc.dummy_mesh.clone() {
            mesh
        } else {
            log::warn!("Failed to set laser sight. The dummy mesh isn't initialized");
            return;
        };

        let material = world
            .resource_mut::<Assets<LaserMaterial>>()
            .add(LaserMaterial { image });

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: Vec3::new(LENGTH / 2.0 + PIXELS_PER_METER / 2.0, 0.0, -1.0),
                    scale: Vec3::new(LENGTH, THICKNESS, 1.0),
                    rotation: Quat::from_rotation_z(PI),
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .set_parent(self.0);
    }
}
