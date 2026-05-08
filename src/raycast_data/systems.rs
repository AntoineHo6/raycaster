use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::constants::{DEGREE, MAP};
use crate::player::components::Player;
use crate::utils::rot_angle;

use super::resource::*;

pub fn update_rays(mut player_query: Query<(&mut Transform, &mut Player), With<Player>>, mut raycast_data: ResMut<RaycastData>) {
    if let Ok((transform, player)) = player_query.single_mut() {
        let mut degree_offset = -(raycast_data.ray_count as f32/2.) * DEGREE;

        for i in 0..raycast_data.ray_count as usize {
            let (ray_pos_h, dist_h) = compute_h_ray_dist(
                &transform.translation.xy(),
                rot_angle(player.angle, degree_offset),
            );
            let (ray_pos_v, dist_v) = compute_v_ray_dist(
                &transform.translation.xy(),
                rot_angle(player.angle, degree_offset),
            );

            if dist_h > dist_v {
                raycast_data.hit_pos[i] = ray_pos_v;
                raycast_data.dist[i] = dist_v;
            } else {
                raycast_data.hit_pos[i] = ray_pos_h;
                raycast_data.dist[i] = dist_h;
            }

            degree_offset += DEGREE;
        }
    }
}

fn compute_h_ray_dist(player_pos: &Vec2, player_angle: f32) -> (Vec2, f32) {
    let arc_tan = 1. / player_angle.tan();

    let mut ray_x: f32;
    let mut ray_y: f32;

    let offset_x: f32;
    let offset_y: f32;

    let mut map_pos: Vec2;
    let mut map_idx: f32;
    let mut dof: u32 = 0;

    let epsilon: f32 = 0.001;

    // --- Check Horizontal lines ---
    if player_angle == 0. || player_angle == PI {
        ray_x = player_pos.x;
        ray_y = player_pos.y;

        offset_y = -64.;
        offset_x = 0.0;
        dof = 8;
    } else if player_angle > PI {
        // looking down
        ray_y = (player_pos.y / 64.).floor() * 64. - epsilon;
        ray_x = player_pos.x + (ray_y - player_pos.y) * arc_tan;

        offset_y = -64.;
        offset_x = offset_y * arc_tan;
    } else {
        // looking up
        ray_y = 64. + (player_pos.y / 64.).floor() * 64. + epsilon;
        ray_x = player_pos.x + (ray_y - player_pos.y) * arc_tan;

        offset_y = 64.;
        offset_x = offset_y * arc_tan;
    }

    while dof < 8 {
        map_pos = Vec2::new(
            ((ray_x + 256.) / 64.).trunc(),
            ((256. - ray_y) / 64.).trunc(),
        );

        map_idx = map_pos.y * 8. + map_pos.x;

        if map_idx < 64. && MAP[map_idx as usize] == 1 {
            dof = 8;
        } else {
            ray_x += offset_x;
            ray_y += offset_y;
            dof += 1;
        }
    }

    let ray_pos = Vec2::new(ray_x, ray_y);
    let dist_h = player_pos.distance(ray_pos);

    return (ray_pos, dist_h);
}

fn compute_v_ray_dist(player_pos: &Vec2, player_angle: f32) -> (Vec2, f32) {
    let mut ray_x: f32;
    let mut ray_y: f32;

    let offset_x: f32;
    let offset_y: f32;

    let mut map_pos: Vec2;
    let mut map_idx: f32;
    let mut dof: u32 = 0;

    let epsilon: f32 = 0.001;

    // --- Check VERTICAL lines ---
    if player_angle < FRAC_PI_2 || player_angle > 3. * FRAC_PI_2 {
        // looking RIGHT
        ray_x = 64. + (player_pos.x / 64.).floor() * 64. + epsilon;
        ray_y = player_pos.y + (ray_x - player_pos.x) * player_angle.tan();

        offset_x = 64.;
        offset_y = offset_x * player_angle.tan();
    } else if player_angle > FRAC_PI_2 && player_angle < 3. * FRAC_PI_2 {
        // looking left
        ray_x = (player_pos.x / 64.).floor() * 64. - epsilon;
        ray_y = player_pos.y + (ray_x - player_pos.x) * player_angle.tan();

        offset_x = -64.;
        offset_y = offset_x * player_angle.tan();
    } else {
        ray_x = player_pos.x;
        ray_y = player_pos.y;

        offset_y = -64.;
        offset_x = 0.0;
        dof = 8;
    }

    while dof < 8 {
        map_pos = Vec2::new(
            ((ray_x + 256.) / 64.).trunc(),
            ((256. - ray_y) / 64.).trunc(),
        );

        map_idx = map_pos.y * 8. + map_pos.x;

        if map_idx < 64. && MAP[map_idx as usize] == 1 {
            dof = 8;
        } else {
            ray_x += offset_x;
            ray_y += offset_y;
            dof += 1;
        }
    }

    let ray_pos = Vec2::new(ray_x, ray_y);
    let dist_v = player_pos.distance(ray_pos);

    return (ray_pos, dist_v);
}
