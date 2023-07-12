use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::transport::NetcodeServerPlugin;
use std::net::UdpSocket;
use std::time::SystemTime;

const MAX_CLIENTS: usize = 64;
const PROTOCOL_ID: u64 = 0;
pub const SERVER_ADDR: &str = "127.0.0.1:5000";
pub const CLIENT_ADDR: &str = "127.0.0.1:0";

pub struct NetworkTransportPlugin(pub &'static str);

impl Plugin for NetworkTransportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetcodeServerPlugin);
        let socket = UdpSocket::bind(self.0).unwrap();
        let server_config = ServerConfig {
            max_clients: MAX_CLIENTS,
            protocol_id: PROTOCOL_ID,
            public_addr: SERVER_ADDR.parse().unwrap(),
            authentication: ServerAuthentication::Unsecure,
        };
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();
        app.insert_resource(transport);
    }
}
