use bevy::color::palettes::tailwind::GREEN_950;
use bevy::{color::palettes::tailwind::GREEN_100, prelude::*};

use bevy::color::palettes::basic::RED;
use bevy::math::Rot2;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3, PI};

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
            angle: 0.,
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

pub fn draw_rays(
    mut gizmos: Gizmos,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((transform, mut player)) = player_query.single_mut() {
        let arc_tan = 1. / player.angle.tan();

        let player_x = transform.translation.x;
        let player_y = transform.translation.y;

        let mut ray_x: f32;
        let mut ray_y: f32;

        let offset_x: f32;
        let offset_y: f32;

        let mut map_pos: Vec2;
        let mut map_idx: f32;
        let mut dof: u32 = 0;

        let epsilon: f32 = 0.001;

        // --- Check Horizontal lines ---
        if player.angle == 0. || player.angle == PI {
            player.angle = 0.001;
            ray_x = player_x;
            ray_y = player_y;

            offset_y = -64.;
            offset_x = 0.0;
        } else if player.angle > PI {
            // looking down
            ray_y = (player_y / 64.).floor() * 64. - epsilon;
            ray_x = player_x + (ray_y - player_y) * arc_tan;

            offset_y = -64.;
            offset_x = offset_y * arc_tan;
        } else {
            // looking up
            ray_y = 64. + (player_y / 64.).floor() * 64. + epsilon;
            ray_x = player_x + (ray_y - player_y) * arc_tan;

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

        gizmos.line_2d(
            transform.translation.xy(),
            Vec2::new(ray_x, ray_y),
            Color::from(RED),
        );
    }
}
