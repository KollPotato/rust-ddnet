use bevy_ecs::system::Resource;
use ndarray::Array2;

use crate::math::Vec2;

pub trait Collision {
    fn check_point(&self, point: Vec2) -> Option<CollisionType>;

    fn check_line(&self, from: Vec2, to: Vec2) -> Option<(Vec2, CollisionType)> {
        let dist = Vec2::distance(from, to);

        let end = (dist + 1.0).trunc() as i32;

        for i in 0..end {
            let point = Vec2::mix(from, to, i as f32 / dist);
            if let Some(col) = self.check_point(point) {
                return Some((point, col));
            }
        }
        None
    }
    fn check_box(&self, pos: Vec2, box_: Vec2) -> bool {
        let diff1 = box_ * 0.5;
        let diff2 = Vec2::new(diff1.x, -diff1.y);
        false
            || self.check_point(pos + diff1).is_some()
            || self.check_point(pos - diff1).is_some()
            || self.check_point(pos + diff2).is_some()
            || self.check_point(pos - diff2).is_some()
    }
    fn move_box(&self, mut pos: Vec2, mut vel: Vec2, box_: Vec2) -> (Vec2, Vec2) {
        let dist = vel.length();
        // Magic number :(
        if dist > 0.00001 {
            let end = dist.round() as i32;
            let fraction = 1.0 / (end + 1) as f32;
            for _ in 0..end + 1 {
                let mut new_pos = pos + vel * fraction;
                if self.check_box(new_pos, box_) {
                    let mut hit = false;
                    if self.check_box(Vec2::new(pos.x, new_pos.y), box_) {
                        new_pos.y = pos.y;
                        vel.y = 0.0;
                        hit = true;
                    }
                    if self.check_box(Vec2::new(new_pos.x, pos.y), box_) {
                        new_pos.x = pos.x;
                        vel.x = 0.0;
                        hit = true;
                    }
                    if !hit {
                        // Original comment: This is a real _corner case_!
                        //
                        // Unfortunately, you actually see this happen, when
                        // diagonally moving towards an corner.
                        new_pos = pos;
                        vel = Vec2::new(0.0, 0.0);
                    }
                }
                pos = new_pos;
            }
        }
        (pos, vel)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CollisionType {
    Hookable,
    Unhookable,
}

#[derive(Resource)]
pub struct Map {
    pub name: String,
    pub crc: i32,
    pub collisions: Array2<Option<CollisionType>>,
    pub contents: Vec<u8>,
}

impl Map {
    fn load(path: &str) -> Self {
        let contents = std::fs::read(path).unwrap();
        let reader = libtw2_datafile::Reader::open(path).unwrap();
        let mut map = libtw2_map::MapReader::from_datafile(reader);
        map.check_version().unwrap();
        let gamelayers = map.game_layers().unwrap();
        let tiles = map.layer_tiles(gamelayers.game()).unwrap();

        Self {
            name: "dm1".into(),
            crc: 0xf2159e6e_u32 as i32,
            collisions: tiles.mapv(|tile| match tile.index {
                1 => Some(CollisionType::Hookable),
                3 => Some(CollisionType::Unhookable),
                _ => None,
            }),
            contents,
        }
    }
}

impl Default for Map {
    fn default() -> Map {
        Map::load("dm1.map")
    }
}

impl Collision for Map {
    fn check_point(&self, pos: Vec2) -> Option<CollisionType> {
        let (x, y) = (pos.x.round() as i32, pos.y.round() as i32);
        let (mut tx, mut ty) = (
            (x as f32 / 32.0).trunc() as i32,
            (y as f32 / 32.0).trunc() as i32,
        );
        if tx < 0 {
            tx = 0;
        }
        if tx > self.collisions.dim().1 as i32 {
            tx = self.collisions.dim().1 as i32 - 1;
        }
        if ty < 0 {
            ty = 0;
        }
        if ty > self.collisions.dim().0 as i32 {
            ty = self.collisions.dim().0 as i32 - 1;
        }
        self.collisions[(ty as usize, tx as usize)]
    }
}
