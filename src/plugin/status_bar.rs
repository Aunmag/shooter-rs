use crate::{
    data::PIXELS_PER_METER,
    plugin::{Actor, Health, Weapon},
    resource::AssetStorage,
    state::AppState,
    util::ext::AppExt,
};
use bevy::{
    app::{App, Plugin},
    asset::Asset,
    ecs::system::{Query, ResMut},
    hierarchy::BuildChildren,
    prelude::{Assets, Children, Entity, Handle, Image, Res, Transform, Vec3, World},
    reflect::TypePath,
    render::{
        mesh::Mesh2d,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d},
    time::Time,
};
use std::{f32::consts::TAU, time::Duration};

const INTERPOLATION: f32 = 8.0;
const PULSE: Duration = Duration::from_millis(500);

pub struct StatusBarPlugin;

impl Plugin for StatusBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StatusBar>::default());
        app.add_state_system(AppState::Game, on_update);
    }
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct StatusBar {
    #[uniform(0)]
    health: f32,
    #[uniform(0)]
    health_alpha: f32,
    #[uniform(0)]
    ammo: f32,
    #[uniform(0)]
    ammo_alpha: f32,
    #[uniform(0)]
    stamina: f32,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl StatusBar {
    pub fn spawn(world: &mut World, parent: Entity) {
        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();
        let material = world.resource_mut::<Assets<StatusBar>>().add(StatusBar {
            health: 0.0,
            health_alpha: 0.0,
            ammo: 1.0,
            ammo_alpha: 0.0,
            stamina: 0.0,
            image,
        });

        let transform = Transform::default().with_scale(Vec3::splat(PIXELS_PER_METER * 1.2));

        world
            .spawn((transform, Mesh2d(mesh), MeshMaterial2d(material)))
            .set_parent(parent);
    }
}

impl Material2d for StatusBar {
    fn fragment_shader() -> ShaderRef {
        return "shader/status_bar.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}

fn on_update(
    targets: Query<(&Actor, &Health, Option<&Weapon>, &Children)>, // TODO: try to simplify
    handles: Query<&MeshMaterial2d<StatusBar>>,
    mut assets: ResMut<Assets<StatusBar>>,
    time: Res<Time>,
) {
    let pulse = (time.elapsed_secs() * TAU / PULSE.as_secs_f32()).cos() / 2.0 + 0.5;
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
