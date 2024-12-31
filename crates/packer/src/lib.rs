use libtw2_common::digest::Sha256;
use libtw2_common::num::Cast;
use uuid::Uuid;

use std::io::Write;
use std::mem;

fn to_bit(b: bool, bit: u32) -> u8 {
    assert!(bit < 8);
    if b {
        1 << bit
    } else {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn write_int_as_string(&mut self, value: i32) -> Result<(), EncoderError> {
        self.write_string(&value.to_string())
    }

    pub fn write_string(&mut self, value: &str) -> Result<(), EncoderError> {
        self.write_raw(value.as_bytes())?;
        self.write_raw(&[0])?;

        Ok(())
    }

    pub fn write_int(&mut self, value: i32) -> Result<(), EncoderError> {
        let sign = if value < 0 { 1 } else { 0 };
        let mut value = (value ^ -sign) as u32;
        let next = (value & 0b0011_1111) as u8;
        value >>= 6;

        self.write_raw(&[to_bit(value != 0, 7) | to_bit(sign != 0, 6) | next])?;

        while value != 0 {
            let next = (value & 0b0111_1111) as u8;
            value >>= 7;
            self.write_raw(&[to_bit(value != 0, 7) | next])?;
        }

        Ok(())
    }

    pub fn write_data(&mut self, data: &[u8]) -> Result<(), EncoderError> {
        self.write_int(data.len().try_i32().ok_or(EncoderError::Unknown)?)?;
        self.write_raw(data)?;

        Ok(())
    }

    pub fn write_raw(&mut self, data: &[u8]) -> Result<(), EncoderError> {
        self.bytes.write(data).map_err(|_| EncoderError::Unknown)?;

        Ok(())
    }

    pub fn write_ints(&mut self, data: &[i32]) -> Result<(), EncoderError> {
        for value in data {
            self.write_int(*value)?;
        }

        Ok(())
    }

    pub fn write_uuid(&mut self, uuid: Uuid) -> Result<(), EncoderError> {
        self.write_raw(uuid.as_bytes())
    }
}

pub struct Decoder<'a> {
    bytes: &'a [u8],
    pub index: usize,
}

#[derive(Debug)]
pub enum EncoderError {
    Unknown,
    InvalidInt,
    CapacityReached,
    TooManyChunks,
}

#[derive(Debug)]
pub enum DecoderError {
    OutOfBounds,
    UnknownId,
    Other,
    InvalidInt,
    InvalidString,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.bytes.len()
    }

    pub fn read_string(&mut self) -> Result<&'a str, DecoderError> {
        let inital_index = self.index;

        while !self.is_empty() {
            let byte = self.read_byte()?;

            if byte == 0 {
                break;
            }
        }

        if self.index == 0 {
            return Err(DecoderError::OutOfBounds);
        }

        return Ok(
            std::str::from_utf8(&self.bytes[inital_index..self.index - 1])
                .map_err(|_| DecoderError::InvalidString)?,
        );
    }

    pub fn read_byte(&mut self) -> Result<u8, DecoderError> {
        if let Some(byte) = self.bytes.get(self.index) {
            self.index += 1;
            Ok(*byte)
        } else {
            Err(DecoderError::OutOfBounds)
        }
    }

    pub fn read_int(&mut self) -> Result<i32, DecoderError> {
        let mut value = 0;

        let mut byte = self.read_byte()?;
        let sign = ((byte >> 6) & 1) as i32;

        value |= (byte & 0b0011_1111) as i32;

        for i in 0..4 {
            if byte & 0b1000_0000 == 0 {
                break;
            }

            byte = self.read_byte()?;

            value |= ((byte & 0b0111_1111) as i32) << (6 + 7 * i);
        }

        value ^= -sign;

        Ok(value)
    }

    pub fn read_data(&mut self) -> Result<&'a [u8], DecoderError> {
        let size = self.read_int()? as usize;

        self.read_raw(size)
    }

    pub fn read_rest(&mut self) -> Result<&'a [u8], DecoderError> {
        self.read_raw(self.bytes.len() - self.index)
    }

    pub fn read_raw(&mut self, size: usize) -> Result<&'a [u8], DecoderError> {
        if size > self.bytes.len() - self.index {
            Err(DecoderError::OutOfBounds)
        } else {
            let bytes = &self.bytes[self.index..self.index + size];
            self.index += size;
            Ok(&bytes)
        }
    }

    pub fn read_sha256(&mut self) -> Result<Sha256, DecoderError> {
        Ok(Sha256::from_slice(self.read_raw(32)?))
    }

    pub fn read_uuid(&mut self) -> Result<Uuid, DecoderError> {
        Ok(Uuid::from_slice(self.read_raw(16)?))
    }

    pub fn read_int_as_string(&mut self) -> Result<i32, DecoderError> {
        Ok(self
            .read_string()?
            .parse()
            .map_err(|_| DecoderError::InvalidInt)?)
    }

    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

pub fn string_to_ints(result: &mut [i32], string: &[u8]) {
    assert!(string.iter().all(|&b| b != 0));
    // Strict less-than because of the NUL-termination.
    assert!(string.len() < result.len() * mem::size_of::<i32>());
    let mut output = result.iter_mut();
    let mut input = string.iter().cloned();
    while let Some(o) = output.next() {
        let v0 = input.next().unwrap_or(0).wrapping_add(0x80);
        let v1 = input.next().unwrap_or(0).wrapping_add(0x80);
        let v2 = input.next().unwrap_or(0).wrapping_add(0x80);
        // FIXME: Use .is_empty()
        let v3 = input
            .next()
            .unwrap_or(if output.len() != 0 { 0 } else { 0x80 })
            .wrapping_add(0x80);
        *o = (v0 as i32) << 24 | (v1 as i32) << 16 | (v2 as i32) << 8 | (v3 as i32);
    }
}

pub fn string_to_ints3(string: &[u8]) -> [i32; 3] {
    let mut result: [i32; 3] = Default::default();
    string_to_ints(&mut result, string);
    result
}

pub fn string_to_ints4(string: &[u8]) -> [i32; 4] {
    let mut result: [i32; 4] = Default::default();
    string_to_ints(&mut result, string);
    result
}

pub fn string_to_ints6(string: &[u8]) -> [i32; 6] {
    let mut result: [i32; 6] = Default::default();
    string_to_ints(&mut result, string);
    result
}
