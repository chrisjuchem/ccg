use ccg::common::network::transport::{NetworkTransportPlugin, SERVER_ADDR};

use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetServer, ServerEvent};
use bevy_renet::RenetServerPlugin;

fn main() {
    App::new()
        .add_plugin(RenetServerPlugin)
        .insert_resource(RenetServer::new(ConnectionConfig::default()))
        .add_plugin(NetworkTransportPlugin(SERVER_ADDR))
        .add_plugins(MinimalPlugins)
        .add_system(log_events)
        .run();
}

fn log_events(mut events: EventReader<ServerEvent>) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Clinet connected! (id={client_id})");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client disconnected! (id={client_id}, reason={reason}");
            }
        }
    }
}
