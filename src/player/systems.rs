use bevy::prelude::*;

use bevy::color::palettes::basic::RED;
use bevy::math::Rot2;
use std::f32::consts::PI;

use super::components::Player;


pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::from(RED),
            custom_size: Some(Vec2::splat(8.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
        Player{
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

            if player.angle > 2.*PI {
                player.angle -= 2.*PI;
            }
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.angle -= 0.025;

            if player.angle < 0. {
                player.angle += 2.*PI;
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

        gizmos.line_2d(
            start.xy(),
            end.xy(),
        Color::WHITE,
        );
    }
}

pub fn draw_rays(
    mut gizmos: Gizmos,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((transform, player)) = player_query.single_mut() {
        let a_tan = -1./player.angle.tan();
        
        let px = transform.translation.x;
        let py = transform.translation.y;

        let rx: f32;
        let ry: f32;

        // --- Check Horizontal lines ---

        if player.angle > PI {  // looking down
            ry = (py / 64.).floor() * 64.;
            rx = px + (py - ry) * a_tan;
        }
        else {  // looking up
            ry = 64. + (py / 64.).floor() * 64.; 
            rx = px + (py - ry) * a_tan;
        }

        // add check IF the angle is equal to 0 or PI
        
        


        
        gizmos.line_2d(
                transform.translation.xy(),
            Vec2::new(rx, ry),
        Color::WHITE,
            );
    }
}