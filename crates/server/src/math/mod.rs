mod angle;
mod collision;
mod vec2;

pub use angle::*;
pub use collision::*;
pub use vec2::*;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn saturated_add(min: f32, max: f32, value: f32, modifier: f32) -> f32 {
    if modifier < 0.0 {
        if value < min {
            return value;
        }
    } else {
        if value > max {
            return value;
        }
    }
    clamp(value + modifier, min, max)
}

pub fn closest_point_on_line(position: Vec2, line_start: Vec2, line_end: Vec2) -> Vec2 {
    let line = line_end - line_start;
    let t = Vec2::dot(line.normalize(), position - line_start) / line.length();
    Vec2::mix(line_start, line_end, clamp(t, 0.0, 1.0))
}
