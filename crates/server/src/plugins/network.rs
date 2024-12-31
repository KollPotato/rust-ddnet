use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use packer::{Decoder, Encoder};
use protocol::{constants, packet::Packet};

use super::Connection;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let settings = app
            .world_mut()
            .get_resource_or_insert_with(NetworkSettings::default)
            .clone();

        let socket = UdpSocket::bind(settings.address).unwrap();
        socket.set_nonblocking(true).unwrap();

        app.insert_resource(Network { socket });

        app.add_event::<PacketEvent>();

        app.add_systems(First, handle_packets);
    }
}

#[derive(Debug, Clone, Resource)]
pub struct NetworkSettings {
    address: SocketAddr,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            address: SocketAddr::from_str("0.0.0.0:8304").unwrap(),
        }
    }
}

#[derive(Debug, Resource)]
pub struct Network {
    pub socket: UdpSocket,
}

impl Network {
    pub fn send(&self, address: impl Into<SocketAddr>, packet: Packet) {
        let address = address.into();
        let mut encoder = Encoder::new();

        if let Err(error) = packet.encode(&mut encoder) {
            log::error!("Could not encode the packet {:?}", error);
            return;
        }

        let bytes = &encoder.bytes();

        if let Err(error) = self.socket.send_to(bytes, address) {
            log::error!("Could not send the packet {:?}", error);
            return;
        };
    }

    pub fn close(&self, address: impl Into<SocketAddr>, reason: &str) {
        let mut encoder = Encoder::new();
        encoder.write_string(reason).unwrap();

        self.send(
            address,
            Packet::Control {
                flags: 0,
                ack: 0,
                id: constants::control::CLOSE,
                payload: encoder.bytes(),
            },
        )
    }

    pub fn flush(&self, connection: &mut Connection) {
        if connection.queued_chunks.len() == 0 {
            return;
        }

        let mut vital_chunk_count = 0;

        for chunk in &connection.queued_chunks {
            if chunk.is_vital() {
                vital_chunk_count += 1;
            }
        }

        connection.vital_chunks_sent += vital_chunk_count;

        self.send(
            connection.address,
            Packet::Chunks {
                flags: 0,
                ack: connection.vital_chunks_acknowledged,
                chunks: connection.queued_chunks.clone(),
            },
        );

        connection.queued_chunks = vec![];
    }
}

fn handle_packets(network: Res<Network>, mut events: EventWriter<PacketEvent>) {
    let mut bytes = vec![0; 1024];

    let Ok((size, address)) = network.socket.recv_from(&mut bytes) else {
        return;
    };

    unsafe { bytes.set_len(size) };

    let mut decoder = Decoder::new(&bytes);

    match Packet::decode(&mut decoder) {
        Ok(packet) => {
            events.send(PacketEvent { address, packet });
        }
        Err(error) => {
            log::debug!("Failed to decode packet: {:?}", error);
            log::debug!("{:?}", &bytes);
        }
    }
}

#[derive(Event)]
pub struct PacketEvent {
    pub address: SocketAddr,
    pub packet: Packet,
}
