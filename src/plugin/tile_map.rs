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
    wait_frames: u8,
}

impl TileMap {
    pub fn count_layers(&self) -> usize {
        let mut layers = Vec::new();

        for layer in self.tiles.keys().map(|i| i.layer) {
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
    for entity in tile_map.to_remove.drain(..) {
        commands.entity(entity).despawn_recursive();
    }

    tile_map.wait_frames = tile_map.wait_frames.saturating_sub(1);

    if tile_map.wait_frames != 0 {
        return;
    }

    let Some((index, entities)) = tile_map.to_blend.pop() else {
        return;
    };

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

    let camera = spawn_camera(&mut commands, index, target);
    tile_map.to_remove.push(camera);
    tile_map.wait_frames = 3; // I don't know why we have to wait exactly 3 frames
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
            let index = Index::from(position);
            world.entity_mut(entity).insert(index.render_layers());

            let mut indexes = Vec::with_capacity(4);
            indexes.push(index);

            for x in [-SPRITE_REACH, 0.0, SPRITE_REACH] {
                for y in [-SPRITE_REACH, 0.0, SPRITE_REACH] {
                    let mut near_position = position;
                    near_position.x += x;
                    near_position.y += y;
                    let near_index = Index::from(near_position);

                    if !indexes.contains(&near_index) {
                        indexes.push(near_index);
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
                        .or_insert_with(Vec::new)
                        .push(entity);
                }
            });
        } else if let Self::Entity(entity) = self {
            world.entity_mut(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Index {
    x: i32,
    y: i32,
    layer: u8,
}

impl Index {
    fn render_layers(&self) -> RenderLayers {
        return RenderLayers::layer(self.layer + 1);
    }
}

impl From<Vec3> for Index {
    fn from(v: Vec3) -> Self {
        let layer = if v.z < LAYER_TREE {
            0
        } else {
            1
        };

        return Self {
            x: floor_by(v.x, TILE_SIZE) as i32,
            y: floor_by(v.y, TILE_SIZE) as i32,
            layer,
        };
    }
}

impl From<Index> for Vec3 {
    fn from(i: Index) -> Self {
        let z = if i.layer == 0 {
            LAYER_GROUND
        } else {
            LAYER_TREE
        };

        return Vec3::new(i.x as f32, i.y as f32, z);
    }
}

fn spawn_camera(commands: &mut Commands, index: Index, target: Handle<Image>) -> Entity {
    let mut translation = Vec3::from(index);
    translation.x += TILE_SIZE / 2.0;
    translation.y += TILE_SIZE / 2.0;
    translation.z = 1.0;

    return commands
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
            transform: Transform {
                translation,
                scale: TRANSFORM_SCALE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(index.render_layers())
        .insert(UiCameraConfig { show_ui: false }) // TODO: remove later. issue: https://github.com/bevyengine/bevy/issues/6069
        .id();
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
