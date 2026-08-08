use crate::{
    data::{LAYER_GROUND, LAYER_PROJECTILE},
    plugin::{
        collision::{Collision, CollisionSystems},
        Actor, AudioPlay, AudioTracker, ProjectileExplosion, TileBlend,
    },
    resource::{AssetStorage, HitResource},
    state::AppState,
    util::ext::{AppExt, Fuzz, Vec2Ext},
};
use bevy::{
    app::{App, Plugin},
    asset::{Asset, Assets},
    color::{palettes::css::WHITE, Alpha},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Command, Deferred, Res, ResMut},
    },
    math::Vec3Swizzles,
    mesh::Mesh2d,
    prelude::{Commands, IntoScheduleConfigs, Query, Vec2, Vec3, Without, World},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};
use rand::RngExt;
use std::{f32::consts::TAU, time::Duration};

const PUSH_MULTIPLIER: f32 = 20.0;
const DURATION: Duration = Duration::from_millis(500);
const FORCE_MIN: f32 = 0.01;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ExplosionMaterial>::default());
        app.add_state_system(AppState::Game, on_update.after(CollisionSystems));
    }
}

pub struct Explode {
    pub config: &'static ProjectileExplosion,
    pub position: Vec2,
    pub shooter: Option<Entity>,
}

impl Command for Explode {
    type Out = ();

    fn apply(self, world: &mut World) {
        let explosion = Explosion {
            config: self.config,
            spawned: world.resource::<Time>().elapsed(),
            damaged: Vec::new(),
            shooter: self.shooter,
        };

        let mesh = world.resource::<AssetStorage>().dummy_mesh().clone();
        let material = world
            .resource_mut::<Assets<ExplosionMaterial>>()
            .add(ExplosionMaterial { alpha: 1.0 });

        world
            .spawn((
                Transform {
                    translation: self.position.extend(LAYER_PROJECTILE),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                Mesh2d(mesh),
                MeshMaterial2d(material),
            ))
            .insert(explosion);

        world.resource::<AudioTracker>().queue(AudioPlay {
            path: "sounds/explosion".into(),
            volume: 1.2,
            source: Some(self.position),
            falloff: AudioPlay::FALLOFF_LONGEST,
            ..AudioPlay::DEFAULT
        });

        let mut rng = rand::rng();

        TileBlend::Image {
            image: "terrain/crater.png",
            color: WHITE.with_alpha(0.8).into(),
            position: self.position.extend(LAYER_GROUND),
            direction: rng.random_range(0.0..TAU),
            size: self.config.radius.fuzz_with(&mut rng, 0.2),
            flip: rng.random(),
        }
        .apply(world);
    }
}

#[derive(Component)]
struct Explosion {
    config: &'static ProjectileExplosion,
    spawned: Duration,
    damaged: Vec<Entity>,
    shooter: Option<Entity>,
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
struct ExplosionMaterial {
    #[uniform(0)]
    alpha: f32,
}

impl Material2d for ExplosionMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/explosion.wgsl".into();
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        return AlphaMode2d::Blend;
    }
}

fn on_update(
    mut explosions: Query<(
        Entity,
        &mut Explosion,
        &mut Transform,
        &MeshMaterial2d<ExplosionMaterial>,
    )>,
    actors: Query<(Entity, &Actor, &Transform, &Collision), Without<Explosion>>,
    mut assets: ResMut<Assets<ExplosionMaterial>>,
    mut hits: Deferred<HitResource>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let now = time.elapsed();

    for (explosion_entity, mut explosion, mut explosion_transform, material) in
        explosions.iter_mut()
    {
        let elapsed = now.saturating_sub(explosion.spawned);
        let radius_factor = (elapsed.as_secs_f32() / DURATION.as_secs_f32())
            .clamp(0.0, 1.0)
            .powf(0.7);

        if radius_factor >= 1.0 {
            commands.entity(explosion_entity).despawn();
            continue;
        }

        let force_factor = 1.0 - radius_factor;
        let radius = explosion.config.radius * radius_factor;
        let explosion_position = explosion_transform.translation.xy();
        explosion_transform.scale.x = radius * 2.0;
        explosion_transform.scale.y = radius * 2.0;

        if let Some(mut material) = assets.get_mut(material) {
            material.alpha = force_factor;
        }

        if force_factor < FORCE_MIN {
            continue;
        }

        let shooter_kind = explosion
            .shooter
            .and_then(|e| actors.get(e).ok())
            .map(|a| a.1.config.kind);

        for (actor_entity, actor, actor_transform, actor_body) in actors.iter() {
            if explosion.shooter == Some(actor_entity) {
                continue;
            }

            if shooter_kind == Some(actor.config.kind) {
                continue;
            }

            let actor_position = actor_transform.translation.xy();

            if actor_position.is_close(explosion_position, radius + actor_body.radius) {
                if explosion.damaged.contains(&actor_entity) {
                    continue;
                }

                let energy = (actor_position - explosion_position).normalize()
                    * explosion.config.energy
                    * force_factor;

                hits.add(actor_entity, energy, 0.0, false);
                hits.add(actor_entity, energy * PUSH_MULTIPLIER, 0.0, true); // extra push without damage
                explosion.damaged.push(actor_entity);
            }
        }
    }
}
