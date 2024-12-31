use crate::{
    constants::{
        CHUNK_FLAG_RESEND, CHUNK_FLAG_VITAL, PACKET_FLAG_COMPRESSION, PACKET_FLAG_CONNLESS,
        PACKET_FLAG_CONTROL,
    },
    net::{ChunkHeader, ChunkHeaderVital, PacketHeader},
};
use packer::{Decoder, DecoderError, Encoder, EncoderError};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Packet {
    Control {
        flags: u8,
        ack: u16,
        id: u8,
        payload: Vec<u8>,
    },
    Connless {
        id: [u8; 8],
        payload: Vec<u8>,
    },
    Chunks {
        flags: u8,
        ack: u16,
        chunks: Vec<Chunk>,
    },
}

impl Packet {
    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        let flags_byte = decoder.read_byte()?;
        decoder.index -= 1;

        if is_connless_packet(flags_byte) {
            let _ = decoder.read_raw(6)?;

            return Ok(Self::Connless {
                id: decoder
                    .read_raw(8)?
                    .try_into()
                    .map_err(|_| DecoderError::Other)?,
                payload: decoder.read_rest()?.to_vec(),
            });
        }

        let header = PacketHeader::decode(decoder)?;

        if header.flags & PACKET_FLAG_CONTROL != 0 {
            // if header.flags & PACKETFLAG_COMPRESSION != 0 {
            //     let compressed_payload = libtw2_huffman::compress(input)
            // }
            return Ok(Self::Control {
                flags: header.flags,
                ack: header.ack,
                id: decoder.read_byte()?,
                payload: decoder.read_rest()?.to_vec(),
            });
        }

        let mut chunks = vec![];

        for _ in 0..header.chunks {
            chunks.push(Chunk::decode(decoder)?);
        }

        Ok(Self::Chunks {
            flags: header.flags,
            ack: header.ack,
            chunks,
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        match self {
            Self::Connless { id, payload } => {
                encoder.write_raw(&[0xff; 6])?;
                encoder.write_raw(id)?;
                encoder.write_raw(payload)?;
            }
            Self::Control {
                mut flags,
                ack,
                id,
                payload,
            } => {
                if flags & PACKET_FLAG_CONTROL == 0 {
                    flags |= PACKET_FLAG_CONTROL;
                }

                let header = PacketHeader {
                    flags,
                    ack: *ack,
                    chunks: 0,
                };

                header.encode(encoder)?;
                encoder.write_int(*id as i32)?;
                encoder.write_raw(payload)?;
            }
            Self::Chunks { flags, ack, chunks } => {
                if chunks.len() > u8::MAX as usize {
                    return Err(EncoderError::TooManyChunks);
                }

                let header = PacketHeader {
                    flags: *flags,
                    ack: *ack,
                    chunks: chunks.len() as u8,
                };

                header.encode(encoder)?;

                if flags & PACKET_FLAG_COMPRESSION != 0 {
                    let mut payload_encoder = Encoder::new();
                    for chunk in chunks {
                        chunk.encode(&mut payload_encoder)?;
                    }

                    let compressed_payload = libtw2_huffman::compress(&payload_encoder.bytes());
                    encoder.write_raw(&compressed_payload)?;
                } else {
                    for chunk in chunks {
                        chunk.encode(encoder)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: ChunkId,
    pub kind: ChunkKind,
    pub payload: Vec<u8>,
    pub vital: Option<(u16, bool)>,
}

impl Chunk {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        let payload_size = self.payload.len()
            + match self.id {
                ChunkId::Ordinal(..) => 1,
                ChunkId::Uuid(..) => 16,
            };

        if let Some((sequence, resend)) = self.vital {
            ChunkHeaderVital {
                flags: CHUNK_FLAG_VITAL | if resend { CHUNK_FLAG_RESEND } else { 0 },
                sequence,
                size: payload_size as u16,
            }
            .encode(encoder)?;
        } else {
            ChunkHeader {
                flags: 0,
                size: payload_size as u16,
            }
            .encode(encoder)?;
        }

        let is_system = matches!(self.kind, ChunkKind::System);

        match self.id {
            ChunkId::Ordinal(value) => {
                encoder.write_int(((value << 1) | is_system as u8) as i32)?;
                encoder.write_raw(&self.payload)?;
            }
            _ => todo!("uuids not implemented yet"),
        }

        Ok(())
    }

    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        let flags = decoder.read_byte()?;
        decoder.index -= 1;

        let (vital, size): (Option<(u16, bool)>, u16) = if is_vital_chunk(flags) {
            let header = ChunkHeaderVital::decode(decoder)?;

            (
                Some((header.sequence, header.flags & CHUNK_FLAG_RESEND != 0)),
                header.size,
            )
        } else {
            let header = ChunkHeader::decode(decoder)?;

            (None, header.size)
        };

        let inital_index = decoder.index;

        let (id, kind) = ChunkId::decode_with_chunk_kind(decoder)?;

        let payload = decoder.read_raw(size as usize - (decoder.index - inital_index))?;

        Ok(Chunk {
            id,
            kind,
            payload: payload.to_vec(),
            vital,
        })
    }
}

fn is_vital_chunk(first_byte: u8) -> bool {
    let flags = (first_byte & 0b1100_0000) >> 6;

    flags & CHUNK_FLAG_VITAL != 0
}

impl ChunkId {
    pub fn decode(decoder: &mut Decoder) -> Result<ChunkId, DecoderError> {
        let id = decoder.read_int()? >> 1;

        Ok(if id != 0 {
            ChunkId::Ordinal(id as u8)
        } else {
            ChunkId::Uuid(decoder.read_uuid()?)
        })
    }

    pub fn decode_with_chunk_kind(
        decoder: &mut Decoder,
    ) -> Result<(ChunkId, ChunkKind), DecoderError> {
        let mut id = decoder.read_int()?;

        let is_system = id & 1 != 0;
        id = id >> 1;

        let message_id = if id != 0 {
            ChunkId::Ordinal(id as u8)
        } else {
            ChunkId::Uuid(decoder.read_uuid()?)
        };

        Ok(if is_system {
            (message_id, ChunkKind::System)
        } else {
            (message_id, ChunkKind::Game)
        })
    }
}

#[derive(Debug, Clone)]
pub enum ChunkId {
    Ordinal(u8),
    Uuid(Uuid),
}

impl Chunk {
    pub fn is_vital(&self) -> bool {
        matches!(self.vital, Some(..))
    }
}

#[derive(Debug, Clone)]
pub enum ChunkKind {
    System,
    Game,
}

pub fn is_connless_packet(flags_byte: u8) -> bool {
    let flags = (flags_byte & 0b1111_0000) >> 4;

    flags & PACKET_FLAG_CONNLESS != 0
}
