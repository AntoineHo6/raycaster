use bevy::prelude::*;

use crate::RayGizmos;
use crate::constants::MAP;
use crate::raycast_data::resource::RaycastData;
use crate::utils::{rot_angle, screen_pos_to_grid_pos};
use bevy::color::palettes::basic::RED;
use bevy::math::Rot2;

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
            let p_new_pos = player_transform.translation + direction.extend(0.) * 150. * time.delta_secs();

            let p_new_grid_pos = screen_pos_to_grid_pos(p_new_pos.xy());

            let map_idx = p_new_grid_pos.y * 8. + p_new_grid_pos.x;

            if MAP[map_idx as usize] == 0 {
                player_transform.translation = p_new_pos;
            }
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
        } else if keyboard_input.pressed(KeyCode::KeyD) {
            player.angle = rot_angle(player.angle, -0.025);
        }
    }
}

pub fn draw_rays(
    mut gizmos: Gizmos<RayGizmos>,
    mut player_query: Query<&mut Transform, With<Player>>,
    raycast_data: Res<RaycastData>,
) {
    if let Ok(transform) = player_query.single_mut() {
        for i in 0..raycast_data.ray_count as usize {
            gizmos.line_2d(
                transform.translation.xy(),
                raycast_data.hit_pos[i],
                Color::from(RED),
            );
        }
    }
}
