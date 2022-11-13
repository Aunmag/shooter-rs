use crate::resource::NetResource;
use bevy::prelude::ResMut;

pub fn connection_update(mut net: ResMut<NetResource>) {
    net.update_connections();
}
