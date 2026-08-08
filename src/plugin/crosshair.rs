use crate::{
    data::{LAYER_CROSSHAIR, PIXELS_PER_METER},
    plugin::{camera::MainCamera, Actor},
    resource::AssetStorage,
    state::AppState,
    util::ext::{AppExt, Vec2Ext},
};
use bevy::{
    app::{App, Plugin},
    asset::{Asset, Assets},
    camera::Projection,
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query},
        world::World,
    },
    math::{Vec2, Vec3},
    mesh::Mesh2d,
    prelude::Transform,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d},
};

const SIZE: f32 = PIXELS_PER_METER * 1.2;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CrosshairMaterial>::default());
        app.add_state_system(
            AppState::Game,
            on_update.after(crate::plugin::camera_target::on_update),
        );
    }
}

#[derive(Component)]
pub struct Crosshair {
    attached_to: Entity,
}

impl Crosshair {
    pub fn spawn(world: &mut World, attached_to: Entity) {
        let mesh = world.resource::<AssetStorage>().dummy_mesh().clone();

        let material = world
            .resource_mut::<Assets<CrosshairMaterial>>()
            .add(CrosshairMaterial {});

        world.spawn((
            Crosshair { attached_to },
            Transform {
                translation: Vec3::new(0.0, 0.0, LAYER_CROSSHAIR),
                ..Transform::default()
            },
            Mesh2d(mesh),
            MeshMaterial2d(material),
        ));
    }

    pub fn despawn(world: &mut World, attached_to: Entity) {
        let mut to_despawn = Vec::new();

        for (entity, crosshair) in world.query::<(Entity, &Crosshair)>().iter(world) {
            if crosshair.attached_to == attached_to {
                to_despawn.push(entity);
            }
        }

        for entity in to_despawn {
            world.entity_mut(entity).despawn();
        }
    }
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
struct CrosshairMaterial {}

impl Material2d for CrosshairMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/crosshair.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}

fn on_update(
    mut crosshairs: Query<(Entity, &Crosshair, &mut Transform), Without<Actor>>,
    actors: Query<(&Actor, &Transform)>,
    cameras: Query<&Projection, With<MainCamera>>,
    mut commands: Commands,
) {
    let projection = cameras.iter().next();

    for (entity, crosshair, mut transform) in crosshairs.iter_mut() {
        let Ok((actor, actor_transform)) = actors.get(crosshair.attached_to) else {
            commands.entity(entity).despawn();
            continue;
        };

        let mut position = Vec2::new(actor.aim_distance, 0.0);
        position = position.rotate_by_quat(actor_transform.rotation);
        position += actor_transform.translation.truncate();

        transform.translation.x = position.x;
        transform.translation.y = position.y;
        transform.rotation = actor_transform.rotation;

        if let Some(Projection::Orthographic(projection)) = projection {
            transform.scale.x = SIZE * projection.scale;
            transform.scale.y = SIZE * projection.scale;
        }
    }
}
