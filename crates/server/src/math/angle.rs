use bevy_ecs::prelude::*;

use crate::math::Vec2;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    pub fn new(radians: f32) -> Self {
        Self { radians }
    }

    pub fn degress(self) -> f32 {
        self.radians / 2.0 / std::f32::consts::PI * 360.0
    }

    pub fn radians(self) -> f32 {
        self.radians
    }

    pub fn direction(self) -> Vec2 {
        let (y, x) = self.radians().sin_cos();

        Vec2::new(x, y)
    }
}

impl Angle {
    fn to_net(&self) -> i32 {
        (self.radians() * 256.0).trunc() as i32
    }

    fn from_net(value: i32) -> Self {
        Self::new((value as f32) / 256.0)
    }
}
