use bevy::prelude::*;

use bevy::color::palettes::basic::RED;
use bevy::math::Rot2;
use std::f32::consts::{FRAC_PI_2, PI};
use crate::constants::{MAP, DEGREE};

use super::components::Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::from(RED),
            custom_size: Some(Vec2::splat(8.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
        Player {
            // in radians
            angle: 0.001,
        },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player_transform, player)) = player_query.single_mut() {
        let mut direction = Vec2::ZERO;

        let rotation = Rot2::radians(player.angle);

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction = rotation * Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction = rotation * Vec2::X * -1.;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            player_transform.translation += direction.extend(0.) * 150. * time.delta_secs();
        }
    }
}

pub fn player_rotation_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.angle = rot_angle(player.angle, 0.025);
        }
        else if keyboard_input.pressed(KeyCode::KeyD) {
            player.angle = rot_angle(player.angle, -0.025);
        }
    }
}

fn compute_h_ray_len(player_pos: &Vec2, player_angle: f32) -> (Vec2, f32) {
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

fn compute_v_ray_len(player_pos: &Vec2, player_angle: f32) -> (Vec2, f32) {
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

pub fn draw_rays(
    mut gizmos: Gizmos,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((transform, player)) = player_query.single_mut() {
        let mut degree_offset = -32. * DEGREE;

        for r in 0..64 {
            let (ray_pos_h, dist_h) = compute_h_ray_len(
                &transform.translation.xy(),
                rot_angle(player.angle, degree_offset),
            );
            let (ray_pos_v, dist_v) = compute_v_ray_len(
                &transform.translation.xy(),
                rot_angle(player.angle, degree_offset),
            );

            let final_ray_pos: Vec2;
            let final_dist: f32;
            
            if dist_h > dist_v {
                final_ray_pos = ray_pos_v;
                final_dist = dist_v;
            } else {
                final_ray_pos = ray_pos_h;
                final_dist = dist_h;
            }

            gizmos.line_2d(transform.translation.xy(), final_ray_pos, Color::from(RED));

            degree_offset += DEGREE;

            // draw 3d scene
            let mut line_height: f32 = (64.*512.) / final_dist;
            let line_off = 160. - (line_height / 8.);   
            if line_height > 512. {
                line_height = 512.;
            }

            let test = 1024. - r as f32*8.;
            gizmos.line_2d(Vec2::new(test, line_off), Vec2::new(test, line_off + line_height), Color::from(RED));
        }


    }
}

fn rot_angle(angle: f32, rotation: f32) -> f32 {
    let mut new_angle: f32 = angle + rotation;

    if new_angle > 2. * PI {
        new_angle -= 2. * PI;
    } else if new_angle < 0. {
        new_angle += 2. * PI;
    }

    return new_angle;
}
