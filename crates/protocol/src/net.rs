use packer::{Decoder, DecoderError, Encoder, EncoderError};

#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub flags: u8,
    pub ack: u16,
    pub chunks: u8,
}

impl PacketHeader {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_raw(&[
            self.flags << 4 | (self.ack >> 8) as u8,
            self.ack as u8,
            self.chunks,
        ])?;

        Ok(())
    }

    pub fn decode(unpacker: &mut Decoder) -> Result<Self, DecoderError> {
        let flags_padding_ack = unpacker.read_byte()?;
        let ack = unpacker.read_byte()?;
        let chunks = unpacker.read_byte()?;

        Ok(Self {
            flags: (flags_padding_ack & 0b1111_0000) >> 4,
            ack: (((flags_padding_ack & 0b0000_0011) as u16) << 8) | (ack as u16),
            chunks,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChunkHeader {
    pub flags: u8,
    pub size: u16,
}

impl ChunkHeader {
    pub fn encode(&self, packer: &mut Encoder) -> Result<(), EncoderError> {
        packer.write_raw(&[
            (self.flags & 0b11) << 6 | ((self.size & 0b11_1111_0000) >> 4) as u8,
            (self.size & 0b00_0000_1111) as u8,
        ])
    }

    pub fn decode(unpacker: &mut Decoder) -> Result<Self, DecoderError> {
        let flags_size = unpacker.read_byte()?;
        let padding_size = unpacker.read_byte()?;

        Ok(Self {
            flags: (flags_size & 0b1100_0000) >> 6,
            size: ((((flags_size & 0b0011_1111) as u16) << 4)
                | (padding_size & 0b0000_1111) as u16),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChunkHeaderVital {
    pub flags: u8,
    pub size: u16,
    pub sequence: u16,
}

impl ChunkHeaderVital {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        let flags_size = (self.flags & 0b11) << 6 | ((self.size & 0b11_1111_0000) >> 4) as u8;
        let padding_size = (self.size & 0b0000_1111) as u8;

        encoder.write_raw(&[
            flags_size,
            (padding_size & 0b1111) | ((self.sequence & 0b11_1100_0000) >> 2) as u8,
            (self.sequence & 0b111_1111) as u8,
        ])
    }

    pub fn decode(decoder: &mut Decoder) -> Result<Self, DecoderError> {
        let flags_size = decoder.read_byte()?;
        let sequence_size = decoder.read_byte()?;
        let sequence = decoder.read_byte()?;

        let size = (((flags_size & 0x3F) as u16) << 4) | ((sequence_size & 0xF) as u16);
        let sequence = (((sequence_size & 0xF0) as u16) << 2) | (sequence as u16 & 0xFF);

        Ok(Self {
            flags: (flags_size >> 6) & 0b11,
            size,
            sequence,
        })
    }
}
