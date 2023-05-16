use crate::model::AppState;
use crate::resource::EntityConverter;
use crate::resource::ServerData;
use crate::resource::TransformUpdateResource;
use crate::util::ext::AppExt;
use bevy::app::App;
use bevy::app::Plugin;
use bevy::prelude::IntoPipeSystem;
use bevy::prelude::IntoSystemConfig;

pub struct GameClientPlugin;

impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        use crate::system::game::*;
        use crate::system::net;

        // TODO: do automatically on enter or make it lazy
        // TODO: remove on exit
        app.insert_resource(EntityConverter::default());
        app.insert_resource(ServerData::default());
        app.insert_resource(TransformUpdateResource::default());
        app.insert_resource(net::InputSendData::default());

        let state = AppState::Game;
        app.add_state_system(state, input);
        app.add_state_system(state, net::interpolation);
        app.add_state_system(state, player.after(input));
        app.add_state_system(state, actor.after(player).after(net::interpolation));
        app.add_state_system(state, inertia.after(actor));
        app.add_state_system(state, net::input_send.after(player).after(actor));
        app.add_state_system(state, projectile.pipe(projectile_hit).after(inertia));
        app.add_state_system(state, net::message_receive);
        app.add_state_system(
            state,
            net::transform_update_receive
                .after(net::message_receive)
                .after(inertia),
        );
        app.add_state_system(state, net::connection_update);
        app.add_state_system(state, camera.after(inertia));
        app.add_state_system(state, terrain);
    }
}
