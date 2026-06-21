use crate::{
    component::{Actor, ActorKind},
    data::{LAYER_GROUND, LAYER_PROJECTILE},
    model::{AppState, AudioPlay},
    plugin::{collision::Collision, AudioTracker, TileBlend},
    resource::{AssetStorage, HitResource},
    util::ext::{AppExt, Vec2Ext},
};
use bevy::{
    app::{App, Plugin},
    asset::{Asset, AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Deferred, Res, ResMut},
        world::Command,
    },
    math::Vec3Swizzles,
    prelude::{
        Commands, DespawnRecursiveExt, IntoSystemConfigs, Query, Vec2, Vec3, Without, World,
    },
    reflect::TypePath,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        texture::Image,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    time::Time,
    transform::components::Transform,
};
use rand::Rng;
use std::{f32::consts::TAU, time::Duration};

const PUSH_MULTIPLIER: f32 = 20.0;
const RADIUS_MAX: f32 = 3.2;
const DURATION: Duration = Duration::from_millis(500);
const ENERGY: f32 = 6.0;
const FORCE_MIN: f32 = 0.01;

const CRATER_DIAMETER_MIN: f32 = 0.8;
const CRATER_DIAMETER_MAX: f32 = 1.2;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<ExplosionMaterial>::default());
        app.add_state_system(
            AppState::Game,
            on_update.after(crate::plugin::collision::on_update),
        );
    }
}

pub struct Explode {
    position: Vec2,
    friendlies: Option<ActorKind>,
}

impl Explode {
    pub fn new(position: Vec2, friendlies: Option<ActorKind>) -> Self {
        return Self {
            position,
            friendlies,
        };
    }
}

impl Command for Explode {
    fn apply(self, world: &mut World) {
        let explosion = Explosion {
            spawned: world.resource::<Time>().elapsed(),
            damaged: Vec::new(),
            friendlies: self.friendlies,
        };

        let assets = world.resource::<AssetStorage>();
        let image = assets.dummy_image().clone();
        let mesh = assets.dummy_mesh().clone();
        let material = world
            .resource_mut::<Assets<ExplosionMaterial>>()
            .add(ExplosionMaterial { alpha: 1.0, image });

        world
            .spawn(MaterialMesh2dBundle {
                transform: Transform {
                    translation: self.position.extend(LAYER_PROJECTILE),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    ..Default::default()
                },
                mesh: mesh.into(),
                material,
                ..Default::default()
            })
            .insert(explosion);

        world.resource::<AudioTracker>().queue(AudioPlay {
            path: "sounds/explosion".into(),
            volume: 1.2,
            source: Some(self.position),
            falloff: AudioPlay::FALLOFF_LONGEST,
            ..AudioPlay::DEFAULT
        });

        let image_path = "terrain/crater.png";
        if let Some(image) = world.resource::<AssetServer>().get_handle(image_path) {
            let direction = rand::thread_rng().gen_range(0.0..TAU);
            let diameter = rand::thread_rng().gen_range(CRATER_DIAMETER_MIN..CRATER_DIAMETER_MAX);
            TileBlend::image(
                image,
                self.position.extend(LAYER_GROUND),
                direction,
                Some(diameter),
            )
            .apply(world);
        } else {
            log::warn!("Image {} not found", image_path);
        }
    }
}

#[derive(Component)]
struct Explosion {
    spawned: Duration,
    damaged: Vec<Entity>,
    friendlies: Option<ActorKind>,
    // TODO: store shooter?
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
struct ExplosionMaterial {
    #[uniform(0)]
    alpha: f32,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Material2d for ExplosionMaterial {
    fn fragment_shader() -> ShaderRef {
        return "shader/explosion.wgsl".into();
    }
}

fn on_update(
    mut explosions: Query<(
        Entity,
        &mut Explosion,
        &mut Transform,
        &Handle<ExplosionMaterial>,
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
        let radius_factor = ((now - explosion.spawned).as_secs_f32() / DURATION.as_secs_f32())
            .clamp(0.0, 1.0)
            .powf(0.7);

        if radius_factor >= 1.0 {
            commands.entity(explosion_entity).despawn_recursive();
            continue;
        }

        let force_factor = 1.0 - radius_factor;
        let radius = RADIUS_MAX * radius_factor;
        let explosion_position = explosion_transform.translation.xy();
        explosion_transform.scale.x = radius * 2.0;
        explosion_transform.scale.y = radius * 2.0;

        if let Some(material) = assets.get_mut(material) {
            material.alpha = force_factor;
        }

        if force_factor < FORCE_MIN {
            continue;
        }

        for (actor_entity, actor, actor_transform, actor_body) in actors.iter() {
            if explosion.friendlies == Some(actor.config.kind) {
                continue;
            }

            let actor_position = actor_transform.translation.xy();

            if actor_position.is_close(explosion_position, radius + actor_body.radius) {
                if explosion.damaged.contains(&actor_entity) {
                    continue;
                }

                let energy =
                    (actor_position - explosion_position).normalize() * force_factor * ENERGY;
                hits.add(actor_entity, energy, 0.0, false);
                hits.add(actor_entity, energy * PUSH_MULTIPLIER, 0.0, true); // extra push without damage
                explosion.damaged.push(actor_entity);
            }
        }
    }
}
