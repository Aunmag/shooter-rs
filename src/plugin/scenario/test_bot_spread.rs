use std::f32::consts::{FRAC_PI_2, PI};
use crate::{command::ActorSet, component::ActorConfig, model::{AppState, TransformLite}, plugin::{bot::ActorBotSet, camera_target::CameraTarget}, util::ext::AppExt};
use bevy::{
    app::Update, ecs::entity::Entity, math::Vec2, prelude::{App, Commands, IntoSystemConfigs, Plugin, Res, Time, World}, transform::components::Transform
};

// TODO: save image

pub struct TestBotSpreadScenarioPlugin;

impl Plugin for TestBotSpreadScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_state_system_enter(AppState::Game, on_enter);
    }
}

fn on_enter(mut commands: Commands) {
    let x = 0.5;
    let y = x * 2.0;
    spawn(Vec2::new(-x, y), &mut commands);
    spawn(Vec2::new(x, y), &mut commands);
    spawn(Vec2::new(0.0, -y), &mut commands);

    let bot = spawn(Vec2::new(0.0, 0.0), &mut commands); // TODO: insert bot
    commands.add(ActorBotSet { entity: bot });

    commands
        .spawn(Transform::default())
        .insert(CameraTarget::default());
}

fn spawn(p: Vec2, commands: &mut Commands) -> Entity {
    let entity = commands.spawn_empty().id();

    commands.add(ActorSet {
        entity,
        config: &ActorConfig::ZOMBIE,
        transform: TransformLite::new(p.x, p.y, FRAC_PI_2),
    });

    return entity;
}
