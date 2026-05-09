use std::f32::consts::TAU;

use bevy::prelude::Vec2;


pub fn rot_angle(angle: f32, rotation: f32) -> f32 {
    return normalize_angle(angle + rotation);
}

pub fn normalize_angle(mut angle: f32) -> f32 {
    if angle > TAU {
        angle -= TAU;
    } else if angle < 0. {
        angle += TAU;
    }

    return angle;
}

pub fn screen_pos_to_grid_pos (coord: Vec2) -> Vec2 {
    let x = ((coord.x + 256.) / 64.).trunc();
    let y = ((256. - coord.y) / 64.).trunc();

    return Vec2::new(x, y);
}