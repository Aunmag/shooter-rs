use crate::components::TransformSync;
use crate::data::LAYER_ACTOR;
use crate::resources::EntityIndexMap;
use crate::utils;
use amethyst::core::transform::Transform;
use amethyst::core::Parent;
use amethyst::ecs::prelude::World;
use amethyst::ecs::Component;
use amethyst::ecs::DenseVecStorage;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::palette::Srgba;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;
use amethyst::renderer::Transparent;

pub struct Actor;

impl Actor {
    pub const MOVEMENT_VELOCITY: f32 = 50.0;

    pub fn create_entity(
        world: &mut World,
        root: Entity,
        public_id: u16,
        x: f32,
        y: f32,
        angle: f32,
        is_ghost: bool,
    ) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, LAYER_ACTOR);
        transform.set_rotation_2d(angle);

        // TODO: Cache renderer
        let renderer = SpriteRender::new(
            utils::load_sprite_sheet(world, "actors/human/image.png", "actors/human/image.ron"),
            0,
        );

        let mut builder = world
            .create_entity()
            .with(Parent { entity: root })
            .with(Actor)
            .with(transform)
            .with(TransformSync::new(x, y, angle))
            .with(renderer);

        if is_ghost {
            builder = builder
                .with(Tint(Srgba::new(0.6, 0.6, 0.6, 0.4)))
                .with(Transparent);
        }

        let actor = builder.build();

        if public_id != 0 {
            world
                .fetch_mut::<EntityIndexMap>()
                .insert(actor.id(), public_id);
        }

        return actor;
    }
}

impl Component for Actor {
    type Storage = DenseVecStorage<Self>;
}
