use super::{Network, PacketEvent, Server, ServerInfo};

use crate::math::{Angle, Map, Vec2};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use packer::{Decoder, Encoder};
use protocol::{
    constants::{self, system::ENTER_GAME, MAX_CLIENTS, VERSION},
    messages::{
        connless::InfoConnless,
        game::StartInfoClientChunk,
        snaps::PlayerInputSnap,
        system::{EmptySnapChunk, InfoChunk, InputClientChunk, InputTiming, MapChangeChunk},
    },
    packet::{Chunk, ChunkId, ChunkKind, Packet},
    snapshot::Snapshot,
};
use std::net::SocketAddr;

#[derive(Debug, Clone, Component)]
pub struct Connection {
    pub address: SocketAddr,
    pub queued_chunks: Vec<Chunk>,
    pub vital_chunks_sent: u16,
    pub vital_chunks_acknowledged: u16,
}

#[derive(Debug, Clone, Component)]
pub struct ConnectionSnapshot {
    snapshot: Snapshot,
}

impl Connection {
    pub fn queue(&mut self, chunk: Chunk) {
        self.queued_chunks.push(chunk);
    }
}

#[derive(Component, Debug, Default)]
pub enum ConnectionState {
    #[default]
    None,
    Authorized,
    Ready,
    Identified,
    Playing,
}

#[derive(Component, Debug)]
pub struct PlayerId(pub u8);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Debug)]
pub struct Username(pub String);

#[derive(Component, Debug)]
pub struct Clan(pub String);

pub struct ConnectionPlugin;

impl Plugin for ConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_request_info,
                handle_info,
                handle_connect,
                handle_ready,
                handle_start_info,
                handle_input,
                handle_enter_game,
                send_snapshots,
            ),
        );
    }
}

pub fn send_snapshots(
    network: Res<Network>,
    server: Res<Server>,
    mut connections: Query<(&mut Connection, &ConnectionState)>,
) {
    if server.current_tick % 2 != 0 {
        return;
    }

    for (mut connection, state) in &mut connections {
        match &state {
            ConnectionState::Identified | ConnectionState::Playing => {
                let mut encoder = Encoder::new();

                EmptySnapChunk {
                    tick: server.current_tick as i32,
                    delta_tick: -1,
                }
                .encode(&mut encoder)
                .unwrap();

                let payload = encoder.bytes();

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::system::EMPTY_SNAP),
                    kind: ChunkKind::System,
                    payload: payload.clone(),
                    vital: None,
                });

                network.flush(&mut connection);
            }
            _ => continue,
        }

        // snapshots.snapshot.add_item(
        //     0,
        //     GameInfoSnap {
        //         game_flags: 0,
        //         game_state_flags: 0,
        //         round_current: 1,
        //         round_num: 1,
        //         round_start_tick: Tick(0),
        //         score_limit: 20,
        //         time_limit: 0,
        //         warmup_timer: 0,
        //     },
        // );
        //

        // snapshots.snapshot.add_item(
        //     player_id.0 as u16,
        //     ClientInfoSnap {
        //         clan: [0xff; 3],
        //         color_body: 500,
        //         color_feet: 500,
        //         country: -1,
        //         name: [0xff; 4],
        //         skin: [0xff; 6],
        //         use_custom_color: 0,
        //     },
        // );

        // let tick = server.current_tick as i32;
        // let delta_tick = -1;
        // let mut cur_part = 0;

        // let crc = snapshots.snapshot.crc();

        // let mut encoder = Encoder::new();
        // snapshots.snapshot.encode(&mut encoder);

        // let data = encoder.bytes();

        // let num_parts: i32 = ((data.len() + MAX_SNAPSHOT_PACKSIZE as usize - 1)
        //     / MAX_SNAPSHOT_PACKSIZE as usize) as i32;

        // pub const MAX_SNAPSHOT_PACKSIZE: i32 = 900;

        // while cur_part < num_parts {
        //     if num_parts == 0 {
        //         let mut encoder = Encoder::new();

        //         EmptySnapChunk { tick, delta_tick }
        //             .encode(&mut encoder)
        //             .unwrap();

        //         network.send(
        //             connection.address,
        //             Packet::Chunks {
        //                 flags: 0,
        //                 ack: connection.vital_chunks_acknowledged,
        //                 chunks: vec![Chunk {
        //                     id: ChunkId::Ordinal(constants::system::EMPTY_SNAP),
        //                     kind: ChunkKind::System,
        //                     payload: encoder.bytes(),
        //                     vital: None,
        //                 }],
        //             },
        //         );
        //     } else if num_parts == 1 {
        //         let mut encoder = Encoder::new();
        //         SingleSnapChunk {
        //             tick,
        //             delta_tick,
        //             crc,
        //             data: &data,
        //         }
        //         .encode(&mut encoder)
        //         .unwrap();

        //         network.send(
        //             connection.address,
        //             Packet::Chunks {
        //                 flags: 0,
        //                 ack: connection.vital_chunks_acknowledged,
        //                 chunks: vec![Chunk {
        //                     id: ChunkId::Ordinal(constants::system::SINGLE_SNAP),
        //                     kind: ChunkKind::System,
        //                     payload: encoder.bytes(),
        //                     vital: None,
        //                 }],
        //             },
        //         );
        //     } else {
        //         let index = cur_part as usize;
        //         let start = MAX_SNAPSHOT_PACKSIZE as usize * index;
        //         let end = std::cmp::min(MAX_SNAPSHOT_PACKSIZE as usize * (index + 1), data.len());

        //         let mut encoder = Encoder::new();
        //         SnapChunk {
        //             tick,
        //             delta_tick,
        //             num_parts,
        //             part: cur_part,
        //             crc,
        //             data: &data[start..end],
        //         }
        //         .encode(&mut encoder)
        //         .unwrap();

        //         network.send(
        //             connection.address,
        //             Packet::Chunks {
        //                 flags: 0,
        //                 ack: connection.vital_chunks_acknowledged,
        //                 chunks: vec![Chunk {
        //                     id: ChunkId::Ordinal(constants::system::SNAP),
        //                     kind: ChunkKind::System,
        //                     payload: encoder.bytes(),
        //                     vital: None,
        //                 }],
        //             },
        //         );
        //     };
        //     cur_part += 1;
        // }

        // snapshots.snapshot = Snapshot {
        //     items: HashMap::new(),
        //     previous: Some(snapshots.snapshot.items.clone()),
        // };
    }
}

pub fn handle_request_info(
    network: Res<Network>,
    server_info: Res<ServerInfo>,
    map: Res<Map>,
    mut events: EventReader<PacketEvent>,
) {
    for event in &mut events.read() {
        if let Packet::Connless {
            id: constants::connless::REQUEST_INFO,
            payload,
        } = &event.packet
        {
            let mut decoder = Decoder::new(&payload);

            let Ok(token) = decoder.read_byte() else {
                continue;
            };

            let info = InfoConnless {
                token: token as i32,
                clients: &[],
                flags: if server_info.password.is_some() {
                    constants::connless::INFO_FLAG_PASSWORD
                } else {
                    0
                },
                name: &server_info.name,
                game_type: &server_info.game_type,
                map: &map.name,
                max_clients: MAX_CLIENTS,
                max_players: MAX_CLIENTS,
                num_clients: 0,
                num_players: 0,
                version: VERSION,
            };

            let mut packer = Encoder::new();

            let Ok(..) = info.encode(&mut packer) else {
                continue;
            };

            network.send(
                event.address,
                Packet::Connless {
                    id: constants::connless::INFO,
                    payload: packer.bytes(),
                },
            );
        }
    }
}

pub fn handle_connect(
    network: Res<Network>,
    mut commands: Commands,
    mut events: EventReader<PacketEvent>,
) {
    for event in &mut events.read() {
        if let Packet::Control {
            id: constants::control::CONNECT,
            ..
        } = &event.packet
        {
            commands.spawn((
                Connection {
                    address: event.address,
                    queued_chunks: vec![],
                    vital_chunks_acknowledged: 0,

                    vital_chunks_sent: 0,
                },
                ConnectionState::None,
                ConnectionSnapshot {
                    snapshot: Snapshot::new(),
                },
            ));

            network.send(
                event.address,
                Packet::Control {
                    flags: 0,
                    ack: 0,
                    id: constants::control::CONNECT_ACCEPT,
                    payload: vec![0xff; 4],
                },
            );
        }
    }
}

pub fn handle_info(
    network: Res<Network>,
    server_info: Res<ServerInfo>,
    map: Res<Map>,
    mut events: EventReader<PacketEvent>,
    mut connections: Query<(&mut Connection, &mut ConnectionState)>,
) {
    for event in &mut events.read() {
        let Some((mut connection, mut state)) = connections
            .iter_mut()
            .find(|(connection, ..)| connection.address == event.address)
        else {
            continue;
        };

        let Packet::Chunks { chunks, .. } = &event.packet else {
            continue;
        };

        for chunk in chunks {
            if let Chunk {
                id: ChunkId::Ordinal(constants::system::INFO),
                kind: ChunkKind::System,
                payload,
                ..
            } = chunk
            {
                connection.vital_chunks_acknowledged += 1;

                let mut decoder = Decoder::new(&payload);
                let info = InfoChunk::decode(&mut decoder).unwrap();

                if info.version != VERSION {
                    network.close(connection.address, "Skill issue");

                    continue;
                }

                if let Some(server_password) = &server_info.password {
                    if info.password != server_password {
                        network.close(connection.address, "Wrong password");

                        continue;
                    }
                }

                *state = ConnectionState::Authorized;

                let mut encoder = Encoder::new();

                MapChangeChunk {
                    name: &map.name,
                    crc: map.crc,
                    size: map.contents.len() as i32,
                }
                .encode(&mut encoder)
                .unwrap();

                let vital_chunks_sent = connection.vital_chunks_sent;

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::system::MAP_CHANGE),
                    kind: ChunkKind::System,
                    payload: encoder.bytes(),
                    vital: Some((vital_chunks_sent, false)),
                });

                network.flush(&mut connection);
            }
        }
    }
}

pub fn handle_enter_game(
    mut events: EventReader<PacketEvent>,
    mut connections: Query<(Entity, &mut Connection, &mut ConnectionState)>,
    mut commands: Commands,
) {
    for event in &mut events.read() {
        let Some((entity, mut connection, mut state)) = connections
            .iter_mut()
            .find(|(_, connection, ..)| connection.address == event.address)
        else {
            continue;
        };

        let Packet::Chunks { chunks, .. } = &event.packet else {
            continue;
        };

        for chunk in chunks {
            if let Chunk {
                id: ChunkId::Ordinal(ENTER_GAME),
                kind: ChunkKind::System,
                ..
            } = chunk
            {
                connection.vital_chunks_acknowledged += 1;

                *state = ConnectionState::Playing;

                commands.entity(entity).insert((
                    Velocity(Vec2::default()),
                    Position(Vec2::new(160.0, 160.0)),
                    Angle::default(),
                    MovementDirection::default(),
                    PlayerInput::default(),
                ));
            } else if let Chunk {
                id: ChunkId::Ordinal(constants::system::RCON_CMD),
                kind: ChunkKind::System,
                ..
            } = chunk
            {
                connection.vital_chunks_acknowledged += 1;
            } else if let Chunk {
                id: ChunkId::Ordinal(constants::system::AUTH_START),
                kind: ChunkKind::System,
                ..
            } = chunk
            {
                connection.vital_chunks_acknowledged += 1;
            }
        }
    }
}

pub fn handle_start_info(
    network: Res<Network>,
    mut events: EventReader<PacketEvent>,
    server_info: Res<ServerInfo>,
    mut connections: Query<(Entity, &mut Connection, &mut ConnectionState)>,
    mut commands: Commands,
) {
    for event in &mut events.read() {
        let Some((entity, mut connection, mut state)) = connections
            .iter_mut()
            .find(|(_, connection, ..)| connection.address == event.address)
        else {
            continue;
        };

        let Packet::Chunks { chunks, .. } = &event.packet else {
            continue;
        };

        for chunk in chunks {
            if let Chunk {
                id: ChunkId::Ordinal(constants::game::CL_START_INFO),
                kind: ChunkKind::Game,
                payload,
                ..
            } = chunk
            {
                connection.vital_chunks_acknowledged += 1;

                let mut decoder = Decoder::new(&payload);

                let start_info = StartInfoClientChunk::decode(&mut decoder).unwrap();

                commands.entity(entity).insert((
                    PlayerId(0),
                    Username(start_info.name.into()),
                    Clan(start_info.clan.into()),
                ));

                *state = ConnectionState::Identified;

                let vital_chunks_sent = connection.vital_chunks_sent;

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::game::SV_VOTE_CLEAR_OPTIONS),
                    kind: ChunkKind::Game,
                    payload: vec![],
                    vital: Some((vital_chunks_sent, false)),
                });

                let mut packer = Encoder::new();
                server_info.tune_params.encode(&mut packer).unwrap();

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::game::SV_TUNE_PARAMS),
                    kind: ChunkKind::Game,
                    payload: packer.bytes(),
                    vital: Some((vital_chunks_sent + 1, false)),
                });

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::game::SV_READY_TO_ENTER),
                    kind: ChunkKind::Game,
                    payload: vec![],
                    vital: Some((vital_chunks_sent + 2, false)),
                });

                network.flush(&mut connection);
            }
        }
    }
}

pub fn handle_ready(
    network: Res<Network>,
    mut events: EventReader<PacketEvent>,
    mut connections: Query<(&mut Connection, &mut ConnectionState)>,
) {
    for event in &mut events.read() {
        let Some((mut connection, mut state)) = connections
            .iter_mut()
            .find(|(connection, ..)| connection.address == event.address)
        else {
            continue;
        };

        let Packet::Chunks { chunks, .. } = &event.packet else {
            continue;
        };

        for chunk in chunks {
            if let Chunk {
                id: ChunkId::Ordinal(constants::system::READY),
                kind: ChunkKind::System,
                ..
            } = chunk
            {
                connection.vital_chunks_acknowledged += 1;

                *state = ConnectionState::Ready;

                let vital_chunks_sent = connection.vital_chunks_sent;

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::system::CON_READY),
                    kind: ChunkKind::System,
                    payload: vec![],
                    vital: Some((vital_chunks_sent, false)),
                });

                network.flush(&mut connection);
            }
        }
    }
}

pub fn handle_input(
    mut events: EventReader<PacketEvent>,
    mut connections: Query<(&mut Connection, &mut PlayerInput)>,
) {
    for event in &mut events.read() {
        let Some((mut connection, mut player_input)) = connections
            .iter_mut()
            .find(|(connection, ..)| connection.address == event.address)
        else {
            continue;
        };

        let Packet::Chunks { chunks, .. } = &event.packet else {
            continue;
        };

        for chunk in chunks {
            if let Chunk {
                id: ChunkId::Ordinal(constants::system::INPUT),
                kind: ChunkKind::System,
                payload,
                ..
            } = chunk
            {
                let mut decoder = Decoder::new(&payload);

                let chunk = InputClientChunk::decode(&mut decoder).unwrap();

                *player_input = PlayerInput(chunk.input);

                let response = InputTiming {
                    input_pred_tick: chunk.intended_tick,
                    time_left: 0,
                };

                let mut packer = Encoder::new();
                response.encode(&mut packer).unwrap();

                connection.queue(Chunk {
                    id: ChunkId::Ordinal(constants::system::INPUT_TIMING),
                    kind: ChunkKind::System,
                    payload: packer.bytes(),
                    vital: None,
                });
            }
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct PlayerInput(pub PlayerInputSnap);

#[derive(Component, Debug, Default)]
pub enum MovementDirection {
    #[default]
    None = 0,
    Left = -1,
    Right = 1,
}

impl MovementDirection {
    pub fn from_net(value: i32) -> Self {
        if value < 0 {
            Self::Left
        } else if value > 0 {
            Self::Right
        } else {
            Self::None
        }
    }

    pub fn to_net(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Left => -1,
            Self::Right => 1,
        }
    }
}
