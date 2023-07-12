use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::RenetClientPlugin;
use ccg::common::network::transport::{NetworkTransportPlugin, CLIENT_ADDR};

fn main() {
    App::new()
        .add_plugin(RenetClientPlugin)
        .insert_resource(RenetClient::new(ConnectionConfig::default()))
        .add_plugin(NetworkTransportPlugin(CLIENT_ADDR))
        .add_plugins(DefaultPlugins)
        .run();
}
