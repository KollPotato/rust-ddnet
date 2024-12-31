use std::ops::*;

use super::{clamp, Angle};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        *self * (1.0 / self.length())
    }

    pub fn angle(&self) -> Angle {
        Angle::new(self.y.atan2(self.x))
    }

    pub fn distance(first: Vec2, second: Vec2) -> f32 {
        (second - first).length()
    }

    pub fn dot(first: Vec2, second: Vec2) -> f32 {
        first.x * second.x + first.y * second.y
    }

    pub fn mix(first: Vec2, second: Vec2, v: f32) -> Vec2 {
        // Needs to be an exact copy of the original, otherwise could use:
        // first * (1.0 - v) + second * v
        first + (second - first) * v
    }
}
