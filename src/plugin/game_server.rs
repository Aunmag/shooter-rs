use crate::model::AppState;
use crate::resource::EntityConverter;
use crate::resource::TransformUpdateResource;
use crate::util::ext::AppExt;
use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::IntoPipeSystem;
use bevy::prelude::IntoSystemConfig;
use std::time::Duration;

pub struct GameServerPlugin {
    sync_interval: Duration,
}

impl GameServerPlugin {
    pub fn new(sync_interval: Duration) -> Self {
        return Self { sync_interval };
    }
}

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        use crate::system::bot;
        use crate::system::game::*;
        use crate::system::net;

        // TODO: do automatically on enter or make it lazy
        // TODO: remove on exit
        app.insert_resource(EntityConverter::default());
        app.insert_resource(TransformUpdateResource::default()); // TODO: keep it on client only
        app.insert_resource(bot::TargetFindData::default());
        app.insert_resource(bot::TargetUpdateData::default());
        app.insert_resource(net::TransformUpdateSendData::new(self.sync_interval));

        let state = AppState::Game;
        app.add_state_system(state, input);
        app.add_state_system(state, health);
        app.add_state_system(state, player.after(input));
        app.add_state_system(state, actor.after(player));
        app.add_state_system(state, inertia.after(actor));
        app.add_state_system(state, collision_find.pipe(collision_resolve).after(inertia));
        app.add_state_system(state, weapon.after(collision_resolve));
        app.add_state_system(
            state,
            projectile.pipe(projectile_hit).after(collision_resolve),
        );
        app.add_state_system(state, net::transform_update_send.after(collision_resolve));
        app.add_state_system(state, net::message_receive);
        app.add_state_system(state, net::connection_update);
        app.add_state_system(state, camera.after(collision_resolve));
        app.add_state_system(state, terrain);
        app.add_state_system(state, bot::target_find);
        app.add_state_system(state, bot::target_update.after(bot::target_find));
        app.add_state_system(state, bot::target_follow.after(bot::target_update));
    }
}