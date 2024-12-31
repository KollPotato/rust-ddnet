use crate::address::{AddrPacked, AddrPackedSliceExt};
use packer::*;

#[derive(Debug, Clone, Copy)]
pub struct ClientConnless<'a> {
    pub name: &'a str,
    pub clan: &'a str,
    pub country: i32,
    pub score: i32,
    pub is_player: i32,
}

impl<'a> ClientConnless<'a> {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_string(self.name)?;
        encoder.write_string(self.clan)?;
        encoder.write_string(&self.country.to_string())?;
        encoder.write_string(&self.score.to_string())?;
        encoder.write_string(&self.is_player.to_string())?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListConnless<'a> {
    pub servers: &'a [AddrPacked],
}

#[derive(Debug, Clone, Copy)]
pub struct CountConnless {
    pub count: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct RequestInfoConnless {
    pub token: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct InfoConnless<'a> {
    pub token: i32,
    pub version: &'a str,
    pub name: &'a str,
    pub map: &'a str,
    pub game_type: &'a str,
    pub flags: i32,
    pub num_players: i32,
    pub max_players: i32,
    pub num_clients: i32,
    pub max_clients: i32,
    pub clients: &'a [ClientConnless<'a>],
}

#[derive(Debug, Clone, Copy)]
pub struct InfoExtendedConnless<'a> {
    pub token: i32,
    pub version: &'a str,
    pub name: &'a str,
    pub map: &'a str,
    pub map_crc: i32,
    pub map_size: i32,
    pub game_type: &'a str,
    pub flags: i32,
    pub num_players: i32,
    pub max_players: i32,
    pub num_clients: i32,
    pub max_clients: i32,
    pub reserved: &'a str,
    pub clients: &'a [ClientConnless<'a>],
}

#[derive(Debug, Clone, Copy)]
pub struct InfoExtendedMoreConnless<'a> {
    pub token: i32,
    pub packet_no: i32,
    pub reserved: &'a str,
    pub clients: &'a [ClientConnless<'a>],
}

#[derive(Debug, Clone, Copy)]
pub struct HeartbeatConnless {
    pub alt_port: u16,
}

impl<'a> ListConnless<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<ListConnless<'a>, DecoderError> {
        Ok(ListConnless {
            servers: AddrPackedSliceExt::from_bytes(encoder.read_rest()?),
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_raw(self.servers.as_bytes())?;

        Ok(())
    }
}

impl CountConnless {
    pub fn decode(encoder: &mut Decoder) -> Result<CountConnless, DecoderError> {
        let result = Ok(CountConnless {
            count: {
                let s = encoder.read_raw(2)?;
                u16::from_be_bytes([s[0], s[1]])
            },
        });

        result
    }
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_raw(&self.count.to_be_bytes())?;
        Ok(())
    }
}

impl RequestInfoConnless {
    pub fn decode(encoder: &mut Decoder) -> Result<RequestInfoConnless, DecoderError> {
        let result = Ok(RequestInfoConnless {
            token: encoder.read_raw(1)?[0],
        });

        result
    }
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_raw(&[self.token])?;

        Ok(())
    }
}

impl<'a> InfoConnless<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<InfoConnless<'a>, DecoderError> {
        Ok(InfoConnless {
            token: encoder.read_int_as_string()?,
            version: encoder.read_string()?,
            name: encoder.read_string()?,
            map: encoder.read_string()?,
            game_type: encoder.read_string()?,
            flags: encoder.read_int_as_string()?,
            num_players: encoder.read_int_as_string()?,
            max_players: encoder.read_int_as_string()?,
            num_clients: encoder.read_int_as_string()?,
            max_clients: encoder.read_int_as_string()?,
            clients: &[],
        })
    }

    pub fn encode(&self, packer: &mut Encoder) -> Result<(), EncoderError> {
        packer.write_int_as_string(self.token)?;
        packer.write_string(self.version)?;
        packer.write_string(self.name)?;
        packer.write_string(self.map)?;
        packer.write_string(self.game_type)?;
        packer.write_int_as_string(self.flags)?;
        packer.write_int_as_string(self.num_players)?;
        packer.write_int_as_string(self.max_players)?;
        packer.write_int_as_string(self.num_clients)?;
        packer.write_int_as_string(self.max_clients)?;

        Ok(())
    }
}

impl<'a> InfoExtendedConnless<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<InfoExtendedConnless<'a>, DecoderError> {
        Ok(InfoExtendedConnless {
            token: encoder.read_string()?.parse().unwrap(),
            version: encoder.read_string()?,
            name: encoder.read_string()?,
            map: encoder.read_string()?,
            map_crc: encoder.read_string()?.parse().unwrap(),
            map_size: encoder.read_string()?.parse().unwrap(),
            game_type: encoder.read_string()?,
            flags: encoder.read_string()?.parse().unwrap(),
            num_players: encoder.read_string()?.parse().unwrap(),
            max_players: encoder.read_string()?.parse().unwrap(),
            num_clients: encoder.read_string()?.parse().unwrap(),
            max_clients: encoder.read_string()?.parse().unwrap(),
            reserved: encoder.read_string()?,
            clients: &[],
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int_as_string(self.token)?;
        encoder.write_string(self.version)?;
        encoder.write_string(self.name)?;
        encoder.write_string(self.map)?;
        encoder.write_int_as_string(self.map_crc)?;
        encoder.write_int_as_string(self.map_size)?;
        encoder.write_string(self.game_type)?;
        encoder.write_int_as_string(self.flags)?;
        encoder.write_int_as_string(self.num_players)?;
        encoder.write_int_as_string(self.max_players)?;
        encoder.write_int_as_string(self.num_clients)?;
        encoder.write_int_as_string(self.max_clients)?;
        encoder.write_string(self.reserved)?;

        Ok(())
    }
}

impl<'a> InfoExtendedMoreConnless<'a> {
    pub fn decode(encoder: &mut Decoder<'a>) -> Result<InfoExtendedMoreConnless<'a>, DecoderError> {
        Ok(InfoExtendedMoreConnless {
            token: encoder.read_int_as_string()?,
            packet_no: encoder.read_int_as_string()?,
            reserved: encoder.read_string()?,
            clients: &[],
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_int_as_string(self.token)?;
        encoder.write_int_as_string(self.packet_no)?;
        encoder.write_string(self.reserved)?;
        Ok(())
    }
}

impl HeartbeatConnless {
    pub fn decode(encoder: &mut Decoder) -> Result<HeartbeatConnless, DecoderError> {
        Ok(HeartbeatConnless {
            alt_port: {
                let s = encoder.read_raw(2)?;
                u16::from_be_bytes([s[0], s[1]])
            },
        })
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        encoder.write_raw(&self.alt_port.to_be_bytes())?;

        Ok(())
    }
}
