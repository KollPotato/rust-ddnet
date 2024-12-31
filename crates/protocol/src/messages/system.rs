use super::snaps::PlayerInputSnap;
use crate::snapshot::SnapItem;
use packer::*;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct InfoChunk<'a> {
    pub version: &'a str,
    pub password: &'a str,
}

#[derive(Clone, Copy)]
pub struct MapChangeChunk<'a> {
    pub name: &'a str,
    pub crc: i32,
    pub size: i32,
}

#[derive(Clone, Copy)]
pub struct MapDataChunk<'a> {
    pub last: i32,
    pub crc: i32,
    pub chunk: i32,
    pub data: &'a [u8],
}

#[derive(Clone, Copy)]
pub struct InputTiming {
    pub input_pred_tick: i32,
    pub time_left: i32,
}

#[derive(Clone, Copy)]
pub struct RconAuthStatus {
    pub auth_level: Option<i32>,
    pub receive_commands: Option<i32>,
}

#[derive(Clone, Copy)]
pub struct RconLine<'a> {
    pub line: &'a str,
}

#[derive(Clone, Copy)]
pub struct Ready;

#[derive(Clone, Copy)]
pub struct EnterGame;

#[derive(Clone, Copy)]
pub struct InputClientChunk {
    pub ack_snapshot: i32,
    pub intended_tick: i32,
    pub input_size: i32,
    pub input: PlayerInputSnap,
}

#[derive(Clone, Copy)]
pub struct RconCmd<'a> {
    pub cmd: &'a str,
}

#[derive(Clone, Copy)]
pub struct RconAuth<'a> {
    pub _unused: &'a str,
    pub password: &'a str,
    pub request_commands: Option<i32>,
}

#[derive(Clone, Copy)]
pub struct RequestMapData {
    pub chunk: i32,
}

#[derive(Clone, Copy)]
pub struct Ping;

#[derive(Clone, Copy)]
pub struct PingReply;

#[derive(Clone, Copy)]
pub struct RconCmdAdd<'a> {
    pub name: &'a str,
    pub help: &'a str,
    pub params: &'a str,
}

#[derive(Clone, Copy)]
pub struct RconCmdRemove<'a> {
    pub name: &'a str,
}

#[derive(Clone, Copy)]
pub struct WhatIs {
    pub uuid: Uuid,
}

#[derive(Clone, Copy)]
pub struct ItIs<'a> {
    pub uuid: Uuid,
    pub name: &'a str,
}

#[derive(Clone, Copy)]
pub struct IDontKnow {
    pub uuid: Uuid,
}

#[derive(Clone, Copy)]
pub struct RconType {
    pub username_required: i32,
}

#[derive(Clone, Copy)]
pub struct MapDetailsChunk<'a> {
    pub name: &'a str,
    pub hash: [u8; 32],
    pub crc: i32,
}

#[derive(Clone, Copy)]
pub struct Capabilities {
    pub version: i32,
    pub flags: i32,
}

#[derive(Clone, Copy)]
pub struct ClientVersion<'a> {
    pub connection_id: Uuid,
    pub ddnet_version: i32,
    pub ddnet_version_string: &'a str,
}

#[derive(Clone, Copy)]
pub struct PingEx {
    pub id: Uuid,
}

#[derive(Clone, Copy)]
pub struct PongEx {
    pub id: Uuid,
}

#[derive(Clone, Copy)]
pub struct ChecksumRequest {
    pub id: Uuid,
    pub start: i32,
    pub length: i32,
}

#[derive(Clone, Copy)]
pub struct ChecksumResponseChunk {
    pub id: Uuid,
    pub hash: [u8; 32],
}

#[derive(Clone, Copy)]
pub struct ChecksumErrorChunk {
    pub id: Uuid,
    pub error: i32,
}

impl<'a> InfoChunk<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<InfoChunk<'a>, DecoderError> {
        let result = Ok(InfoChunk {
            version: _p.read_string()?,
            password: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.version)?;
        _p.write_string(self.password)?;
        Ok(())
    }
}

impl<'a> MapChangeChunk<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<MapChangeChunk<'a>, DecoderError> {
        let result = Ok(MapChangeChunk {
            name: _p.read_string()?,
            crc: _p.read_int()?,
            size: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.name)?;
        _p.write_int(self.crc)?;
        _p.write_int(self.size)?;
        Ok(())
    }
}

impl<'a> MapDataChunk<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<MapDataChunk<'a>, DecoderError> {
        let result = Ok(MapDataChunk {
            last: _p.read_int()?,
            crc: _p.read_int()?,
            chunk: _p.read_int()?,
            data: _p.read_data()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_int(self.last)?;
        _p.write_int(self.crc)?;
        _p.write_int(self.chunk)?;
        _p.write_data(self.data)?;
        Ok(())
    }
}

impl InputTiming {
    pub fn decode(_p: &mut Decoder) -> Result<InputTiming, DecoderError> {
        let result = Ok(InputTiming {
            input_pred_tick: _p.read_int()?,
            time_left: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_int(self.input_pred_tick)?;
        _p.write_int(self.time_left)?;
        Ok(())
    }
}

impl RconAuthStatus {
    pub fn decode(_p: &mut Decoder) -> Result<RconAuthStatus, DecoderError> {
        let result = Ok(RconAuthStatus {
            auth_level: _p.read_int().ok(),
            receive_commands: _p.read_int().ok(),
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        assert!(self.auth_level.is_some());
        assert!(self.receive_commands.is_some());
        _p.write_int(self.auth_level.unwrap())?;
        _p.write_int(self.receive_commands.unwrap())?;
        Ok(())
    }
}

impl<'a> RconLine<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<RconLine<'a>, DecoderError> {
        let result = Ok(RconLine {
            line: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.line)?;
        Ok(())
    }
}

impl Ready {
    pub fn decode(_p: &mut Decoder) -> Result<Ready, DecoderError> {
        let result = Ok(Ready);

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        Ok(())
    }
}

impl EnterGame {
    pub fn decode(_p: &mut Decoder) -> Result<EnterGame, DecoderError> {
        let result = Ok(EnterGame);

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        Ok(())
    }
}

impl InputClientChunk {
    pub fn decode(decoder: &mut Decoder) -> Result<InputClientChunk, DecoderError> {
        Ok(InputClientChunk {
            ack_snapshot: decoder.read_int()?,
            intended_tick: decoder.read_int()?,
            input_size: decoder.read_int()?,
            input: PlayerInputSnap::decode(decoder)?,
        })
    }

    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_int(self.ack_snapshot)?;
        _p.write_int(self.intended_tick)?;
        _p.write_int(self.input_size)?;
        _p.write_ints(self.input.encode())?;

        Ok(())
    }
}

impl<'a> RconCmd<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<RconCmd<'a>, DecoderError> {
        let result = Ok(RconCmd {
            cmd: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.cmd)?;
        Ok(())
    }
}

impl<'a> RconAuth<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<RconAuth<'a>, DecoderError> {
        let result = Ok(RconAuth {
            _unused: _p.read_string()?,
            password: _p.read_string()?,
            request_commands: _p.read_int().ok(),
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        assert!(self.request_commands.is_some());
        _p.write_string(self._unused)?;
        _p.write_string(self.password)?;
        _p.write_int(self.request_commands.unwrap())?;
        Ok(())
    }
}

impl RequestMapData {
    pub fn decode(_p: &mut Decoder) -> Result<RequestMapData, DecoderError> {
        let result = Ok(RequestMapData {
            chunk: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_int(self.chunk)?;
        Ok(())
    }
}

impl Ping {
    pub fn decode(_p: &mut Decoder) -> Result<Ping, DecoderError> {
        let result = Ok(Ping);

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        Ok(())
    }
}

impl PingReply {
    pub fn decode(_p: &mut Decoder) -> Result<PingReply, DecoderError> {
        let result = Ok(PingReply);

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        Ok(())
    }
}

impl<'a> RconCmdAdd<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<RconCmdAdd<'a>, DecoderError> {
        let result = Ok(RconCmdAdd {
            name: _p.read_string()?,
            help: _p.read_string()?,
            params: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.name)?;
        _p.write_string(self.help)?;
        _p.write_string(self.params)?;
        Ok(())
    }
}

impl<'a> RconCmdRemove<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<RconCmdRemove<'a>, DecoderError> {
        let result = Ok(RconCmdRemove {
            name: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.name)?;
        Ok(())
    }
}

impl WhatIs {
    pub fn decode(_p: &mut Decoder) -> Result<WhatIs, DecoderError> {
        let result = Ok(WhatIs {
            uuid: Uuid::from_slice(_p.read_raw(16)?),
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.uuid.as_bytes())?;
        Ok(())
    }
}

impl<'a> ItIs<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<ItIs<'a>, DecoderError> {
        let result = Ok(ItIs {
            uuid: Uuid::from_slice(_p.read_raw(16)?),
            name: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.uuid.as_bytes())?;
        _p.write_string(self.name)?;
        Ok(())
    }
}
impl IDontKnow {
    pub fn decode(_p: &mut Decoder) -> Result<IDontKnow, DecoderError> {
        let result = Ok(IDontKnow {
            uuid: Uuid::from_slice(_p.read_raw(16)?),
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.uuid.as_bytes())?;
        Ok(())
    }
}

impl RconType {
    pub fn decode(_p: &mut Decoder) -> Result<RconType, DecoderError> {
        let result = Ok(RconType {
            username_required: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_int(self.username_required as i32)?;
        Ok(())
    }
}

impl<'a> MapDetailsChunk<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<MapDetailsChunk<'a>, DecoderError> {
        let result = Ok(MapDetailsChunk {
            name: _p.read_string()?,
            hash: _p.read_raw(32)?.try_into().unwrap(),
            crc: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_string(self.name)?;
        _p.write_raw(&self.hash)?;
        _p.write_int(self.crc)?;
        Ok(())
    }
}

impl Capabilities {
    pub fn decode(_p: &mut Decoder) -> Result<Capabilities, DecoderError> {
        let result = Ok(Capabilities {
            version: _p.read_int()?,
            flags: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_int(self.version)?;
        _p.write_int(self.flags)?;
        Ok(())
    }
}

impl<'a> ClientVersion<'a> {
    pub fn decode(_p: &mut Decoder<'a>) -> Result<ClientVersion<'a>, DecoderError> {
        let result = Ok(ClientVersion {
            connection_id: Uuid::from_slice(_p.read_raw(16)?),
            ddnet_version: _p.read_int()?,
            ddnet_version_string: _p.read_string()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.connection_id.as_bytes())?;
        _p.write_int(self.ddnet_version)?;
        _p.write_string(self.ddnet_version_string)?;
        Ok(())
    }
}
impl PingEx {
    pub fn decode(_p: &mut Decoder) -> Result<PingEx, DecoderError> {
        let result = Ok(PingEx {
            id: Uuid::from_slice(_p.read_raw(16)?),
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.id.as_bytes())?;
        Ok(())
    }
}

impl PongEx {
    pub fn decode(_p: &mut Decoder) -> Result<PongEx, DecoderError> {
        let result = Ok(PongEx {
            id: Uuid::from_slice(_p.read_raw(16)?),
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.id.as_bytes())?;
        Ok(())
    }
}

impl ChecksumRequest {
    pub fn decode(_p: &mut Decoder) -> Result<ChecksumRequest, DecoderError> {
        let result = Ok(ChecksumRequest {
            id: Uuid::from_slice(_p.read_raw(16)?),
            start: _p.read_int()?,
            length: _p.read_int()?,
        });

        result
    }
    pub fn encode(&self, _p: &mut Encoder) -> Result<(), EncoderError> {
        _p.write_raw(self.id.as_bytes())?;
        _p.write_int(self.start)?;
        _p.write_int(self.length)?;
        Ok(())
    }
}

impl ChecksumResponseChunk {
    pub fn decode(decoder: &mut Decoder) -> Result<ChecksumResponseChunk, DecoderError> {
        Ok(ChecksumResponseChunk {
            id: decoder.read_uuid()?,
            hash: decoder.read_raw(32)?.try_into().unwrap(),
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_raw(self.id.as_bytes())?;
        encoder.write_raw(&self.hash)?;

        Ok(())
    }
}

impl ChecksumErrorChunk {
    pub fn decode(decoder: &mut Decoder) -> Result<ChecksumErrorChunk, DecoderError> {
        Ok(ChecksumErrorChunk {
            id: Uuid::from_slice(decoder.read_raw(16)?),
            error: decoder.read_int()?,
        })
    }

    pub fn encode(&self, packer: &mut Encoder) -> Result<(), EncoderError> {
        packer.write_raw(self.id.as_bytes())?;
        packer.write_int(self.error)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SnapChunk<'a> {
    pub tick: i32,
    pub delta_tick: i32,
    pub num_parts: i32,
    pub part: i32,
    pub crc: i32,
    pub data: &'a [u8],
}

#[derive(Debug, Clone, Copy)]
pub struct EmptySnapChunk {
    pub tick: i32,
    pub delta_tick: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SingleSnapChunk<'a> {
    pub tick: i32,
    pub delta_tick: i32,
    pub crc: i32,
    pub data: &'a [u8],
}

impl<'a> SnapChunk<'a> {
    pub fn decode(unpacker: &mut Decoder<'a>) -> Result<SnapChunk<'a>, DecoderError> {
        Ok(SnapChunk {
            tick: unpacker.read_int()?,
            delta_tick: unpacker.read_int()?,
            num_parts: unpacker.read_int()?,
            part: unpacker.read_int()?,
            crc: unpacker.read_int()?,
            data: unpacker.read_data()?,
        })
    }

    pub fn encode(&self, packer: &mut Encoder) -> Result<(), EncoderError> {
        packer.write_int(self.tick)?;
        packer.write_int(self.delta_tick)?;
        packer.write_int(self.num_parts)?;
        packer.write_int(self.part)?;
        packer.write_int(self.crc)?;
        packer.write_data(self.data)?;

        Ok(())
    }
}

impl EmptySnapChunk {
    pub fn decode(unpacker: &mut Decoder) -> Result<EmptySnapChunk, DecoderError> {
        Ok(EmptySnapChunk {
            tick: unpacker.read_int()?,
            delta_tick: unpacker.read_int()?,
        })
    }

    pub fn encode(&self, packer: &mut Encoder) -> Result<(), EncoderError> {
        packer.write_int(self.tick)?;
        packer.write_int(self.delta_tick)?;

        Ok(())
    }
}

impl<'a> SingleSnapChunk<'a> {
    pub fn decode(unpacker: &mut Decoder<'a>) -> Result<SingleSnapChunk<'a>, DecoderError> {
        Ok(SingleSnapChunk {
            tick: unpacker.read_int()?,
            delta_tick: unpacker.read_int()?,
            crc: unpacker.read_int()?,
            data: unpacker.read_data()?,
        })
    }

    pub fn encode(&self, packer: &mut Encoder) -> Result<(), EncoderError> {
        packer.write_int(self.tick)?;
        packer.write_int(self.delta_tick)?;
        packer.write_int(self.crc)?;
        packer.write_data(self.data)?;

        Ok(())
    }
}
