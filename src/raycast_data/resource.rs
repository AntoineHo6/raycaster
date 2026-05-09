use bevy::prelude::*;

#[derive(Resource)]
pub struct RaycastData {
    pub ray_count: u8,
    pub dist: [f32; 64],
    pub hit_pos: [Vec2; 64],
    pub angle: [f32; 64],
    pub hit_vert_line: [bool; 64],
}

impl Default for RaycastData {
    fn default() -> Self {
        Self {
            ray_count: 64,
            dist: [0.; 64],
            hit_pos: [Vec2::ZERO; 64],
            angle: [0. ; 64],
            hit_vert_line: [false; 64]
        }
    }
}