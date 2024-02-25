use crate::{
    data::{LAYER_GROUND, LAYER_TREE, PIXELS_PER_METER, TRANSFORM_SCALE},
    model::AppState,
    util::{
        ext::{AppExt, HashMapExt, ImageExt},
        math::floor_by,
    },
};
use bevy::{
    app::{App, Plugin},
    core_pipeline::{
        clear_color::ClearColorConfig,
        core_2d::{Camera2d, Camera2dBundle},
    },
    ecs::{
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Command, Commands, Res, ResMut},
        world::Mut,
    },
    hierarchy::DespawnRecursiveExt,
    math::{Quat, Vec3},
    prelude::{Assets, Handle, Image, Resource, SpriteBundle, Transform, World},
    render::{
        camera::{Camera, CameraOutputMode, RenderTarget},
        render_resource::{BlendState, LoadOp, TextureUsages},
        view::RenderLayers,
    },
    sprite::{Anchor, Sprite},
    ui::camera_config::UiCameraConfig,
};
use std::collections::HashMap;

// TODO: fix. strange white pixels

const TILE_SIZE: f32 = 16.0;
const TILE_SIZE_PX: u32 = (TILE_SIZE * PIXELS_PER_METER) as u32;
const SPRITE_REACH: f32 = 4.0;
const RENDER_LAYER: RenderLayers = RenderLayers::layer(1);

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::default());
        app.add_state_system(
            AppState::Game,
            on_update.run_if(|r: Res<TileMap>| r.has_work()),
        );
    }
}

#[derive(Default, Resource)]
pub struct TileMap {
    tiles: HashMap<Index, Handle<Image>>,
    to_blend: HashMap<Index, Vec<Entity>>,
    to_remove: Vec<Entity>,
    wait: bool, // TODO: rework
}

impl TileMap {
    pub fn count_layers(&self) -> usize {
        let mut layers = Vec::new();

        for layer in self.tiles.keys().map(|i| i.2) {
            if !layers.contains(&layer) {
                layers.push(layer);
            }
        }

        return layers.len();
    }

    pub fn count_tiles(&self) -> usize {
        return self.tiles.len();
    }

    pub fn count_queue(&self) -> usize {
        let mut queued = 0;

        for entities in self.to_blend.values() {
            queued += entities.len();
        }

        return queued;
    }

    fn has_work(&self) -> bool {
        return !self.to_blend.is_empty() || !self.to_remove.is_empty();
    }
}

fn on_update(mut tile_map: ResMut<TileMap>, mut commands: Commands) {
    if tile_map.wait {
        tile_map.wait = false;
        return;
    }

    let mut wait = false;

    for blended in tile_map.to_remove.drain(..) {
        commands.entity(blended).despawn_recursive();
        wait = true;
    }

    tile_map.wait = wait;

    if tile_map.wait {
        return;
    }

    let Some((index, entities)) = tile_map.to_blend.pop() else {
        return;
    };

    let position: Vec3 = index.into();

    for entity in entities {
        let mut is_last = true;

        for entities in tile_map.to_blend.values() {
            if entities.contains(&entity) {
                is_last = false;
                break;
            }
        }

        if is_last {
            tile_map.to_remove.push(entity);
        }
    }

    let Some(target) = tile_map.tiles.get(&index).cloned() else {
        log::warn!("Tile {:?} not exists", index);
        return;
    };

    let mut transform = Transform::default();
    transform.translation.x = position.x + TILE_SIZE / 2.0;
    transform.translation.y = position.y + TILE_SIZE / 2.0;
    transform.scale = TRANSFORM_SCALE;

    // TODO: don't spawn every time?
    let camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                target: RenderTarget::Image(target),
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    color_attachment_load_op: LoadOp::Load,
                },
                ..Default::default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            transform,
            ..Default::default()
        })
        .insert(RENDER_LAYER)
        .insert(UiCameraConfig { show_ui: false }) // TODO: remove later. issue: https://github.com/bevyengine/bevy/issues/6069
        .id();

    tile_map.to_remove.push(camera);
}

pub enum TileBlend {
    Entity(Entity),
    Image {
        image: Handle<Image>,
        position: Vec3,
        direction: f32,
    },
}

impl TileBlend {
    pub fn image(image: Handle<Image>, position: Vec3, direction: f32) -> Self {
        return Self::Image {
            image,
            position,
            direction,
        };
    }

    fn provide_position(&self, world: &World) -> Option<Vec3> {
        match self {
            Self::Entity(entity) => {
                return world.get::<Transform>(*entity).map(|t| t.translation);
            }
            Self::Image { position, .. } => {
                return Some(*position);
            }
        }
    }

    fn provide_entity(self, world: &mut World) -> Entity {
        match self {
            Self::Entity(entity) => {
                return entity;
            }
            Self::Image {
                position,
                direction,
                image,
                ..
            } => {
                return world
                    .spawn(SpriteBundle {
                        transform: Transform {
                            translation: position,
                            rotation: Quat::from_rotation_z(direction),
                            scale: TRANSFORM_SCALE,
                        },
                        texture: image,
                        ..Default::default()
                    })
                    .id();
            }
        }
    }
}

impl Command for TileBlend {
    fn apply(self, world: &mut World) {
        if let Some(position) = self.provide_position(world) {
            let entity = self.provide_entity(world);
            world.entity_mut(entity).insert(RENDER_LAYER);

            let mut indexes = Vec::with_capacity(4);

            for x in [-SPRITE_REACH, 0.0, SPRITE_REACH] {
                for y in [-SPRITE_REACH, 0.0, SPRITE_REACH] {
                    let mut position_near = position;
                    position_near.x += x;
                    position_near.y += y;
                    let index = Index::from(position_near);

                    if !indexes.contains(&index) {
                        indexes.push(index);
                    }
                }
            }

            world.resource_scope(|world: &mut World, mut tile_map: Mut<TileMap>| {
                for index in indexes {
                    tile_map
                        .tiles
                        .entry(index)
                        .or_insert_with(|| spawn_tile(world, index.into()));

                    tile_map
                        .to_blend
                        .entry(index)
                        .and_modify(|e| e.push(entity))
                        .or_insert_with(|| vec![entity]);
                }
            });
        } else if let Self::Entity(entity) = self {
            world.entity_mut(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Index(i32, i32, u8);

impl From<Vec3> for Index {
    fn from(v: Vec3) -> Self {
        return Self(
            floor_by(v.x, TILE_SIZE) as i32,
            floor_by(v.y, TILE_SIZE) as i32,
            v.z.clamp(LAYER_GROUND, LAYER_TREE).trunc() as u8,
        );
    }
}

impl From<Index> for Vec3 {
    fn from(i: Index) -> Self {
        return Vec3::new(i.0 as f32, i.1 as f32, i.2 as f32);
    }
}

fn spawn_tile(world: &mut World, position: Vec3) -> Handle<Image> {
    let mut image = Image::blank(TILE_SIZE_PX, TILE_SIZE_PX);
    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;

    let handle = world.resource_mut::<Assets<Image>>().add(image);

    world.spawn(SpriteBundle {
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform {
            translation: position,
            scale: TRANSFORM_SCALE,
            ..Default::default()
        },
        texture: handle.clone(),
        ..Default::default()
    });

    return handle;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_size_px() {
        assert_eq!(TILE_SIZE_PX, 512);
    }
}
