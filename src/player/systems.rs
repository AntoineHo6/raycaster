use bevy::prelude::*;

use bevy::color::palettes::basic::RED;
use bevy::math::Rot2;
use std::f32::consts::{FRAC_PI_2, PI};

use crate::MAP;

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

pub fn player_sight_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.angle += 0.025;

            if player.angle > 2. * PI {
                player.angle -= 2. * PI;
            }
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.angle -= 0.025;

            if player.angle < 0. {
                player.angle += 2. * PI;
            }
        }
    }
}

pub fn draw_player_sight(
    mut gizmos: Gizmos,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((player_transform, player)) = player_query.single_mut() {
        let start = player_transform.translation.xy();
        let length = 15.;

        let rotation = Rot2::radians(player.angle);
        let direction = rotation * Vec2::X;

        let end = start + (direction * length);

        gizmos.line_2d(start.xy(), end.xy(), Color::WHITE);
    }
}

fn compute_h_ray_len(player_pos: Vec2, player_angle: f32) -> (Vec2, f32) {
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

fn compute_v_ray_len(player_pos: Vec2, player_angle: f32) -> (Vec2, f32) {
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
        let (ray_pos_h, dist_h) = compute_h_ray_len(transform.translation.xy(), player.angle);
        let (ray_pos_v, dist_v) = compute_v_ray_len(transform.translation.xy(), player.angle);

        if dist_h > dist_v {
            gizmos.line_2d(transform.translation.xy(), ray_pos_v, Color::from(RED));
        } else {
            gizmos.line_2d(transform.translation.xy(), ray_pos_h, Color::from(RED));
        }
    }
}
