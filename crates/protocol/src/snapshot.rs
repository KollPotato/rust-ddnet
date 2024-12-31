use crate::messages::snaps::{
    ClientInfoSnap, GameInfoSnap, PlayerInputSnap, CLIENT_INFO, GAME_INFO, PLAYER_INPUT,
};
use packer::{Encoder, EncoderError};
use std::{collections::HashMap, mem::transmute, slice::from_ref};

pub trait SnapItem<'a>: Sized {
    fn type_id(&self) -> u16;

    fn encode(&self) -> &'a [i32] {
        unsafe { transmute(from_ref(self)) }
    }
}

impl<'a> SnapItem<'a> for GameInfoSnap {
    fn type_id(&self) -> u16 {
        GAME_INFO
    }
}

impl<'a> SnapItem<'a> for ClientInfoSnap {
    fn type_id(&self) -> u16 {
        CLIENT_INFO
    }
}

impl<'a> SnapItem<'a> for PlayerInputSnap {
    fn type_id(&self) -> u16 {
        PLAYER_INPUT
    }
}

#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    pub previous: Option<HashMap<i32, Vec<i32>>>,
    pub items: HashMap<i32, Vec<i32>>,
}

impl Snapshot {
    pub fn new() -> Self {
        Self {
            previous: None,
            items: HashMap::new(),
        }
    }

    pub fn add_item<'a>(&mut self, id: u16, item: impl SnapItem<'a>) {
        let key = create_key(item.type_id(), id);
        let data = item.encode().to_vec();

        self.items.insert(key, data);
    }

    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), EncoderError> {
        let mut deleted_keys: Vec<i32> = vec![];

        if let Some(previous) = &self.previous {
            for previous_key in previous.keys() {
                if !self.items.contains_key(previous_key) {
                    deleted_keys.push(*previous_key);
                }
            }
        }

        encoder.write_int(deleted_keys.len() as i32)?;
        encoder.write_int(self.items.keys().len() as i32)?;
        encoder.write_int(0)?;

        for key in deleted_keys {
            encoder.write_int(key)?;
        }

        for (key, data) in &mut self.items.clone() {
            if let Some(previous) = &self.previous {
                if let Some(previous_data) = previous.get(&key) {
                    for (value, previous_value) in data.iter_mut().zip(previous_data.iter()) {
                        *value = *value - previous_value;
                    }
                }
            }

            encoder.write_int(key_to_type_id(*key) as i32)?;
            encoder.write_int(key_to_id(*key) as i32)?;
            encoder.write_ints(data)?;
        }

        Ok(())
    }

    pub fn crc(&self) -> i32 {
        self.items.values().fold(0, |accum, data| {
            accum
                + data
                    .iter()
                    .fold(accum, |accum, &value| accum.wrapping_add(value))
        })
    }
}

pub fn key_to_type_id(key: i32) -> u16 {
    ((key as u32 >> 16) & 0xffff) as u16
}

pub fn key_to_id(key: i32) -> u16 {
    ((key as u32) & 0xffff) as u16
}

pub fn create_key(raw_type_id: u16, id: u16) -> i32 {
    (((raw_type_id as u32) << 16) | (id as u32)) as i32
}
