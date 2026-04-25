use bevy::prelude::*;
use bevy::color::palettes::basic::RED;
use bevy::math::Rot2;
use std::f64::consts::PI;

const MAP: [[u8; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1 ,1 ,1],
]; 

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .add_systems(Update, player_sight_controls)
    .add_systems(Update, draw_player_sight)
    .run();
}

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set the window res
    let mut window = windows.single_mut().unwrap();
    window.resolution.set(512.0, 512.0);
    window.resizable = false;

    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn map
    for (y, row) in MAP.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let x_coord = x as f32 * 64. - (window.width() / 2.0) + 32.;
            let y_coord = y as f32 * 64. - (window.height() / 2.0) + 32.;
            
            if cell == 1 {
                commands.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::splat(62.)),
                        ..default()
                    },
                    Transform::from_xyz(x_coord, y_coord, 0.),
                ));
            }
            else {
                commands.spawn((
                    Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(62.)),
                        ..default()
                    },
                    Transform::from_xyz(x_coord, y_coord, 0.),
                ));
            }
        }
    }
    
    // Spawn player
    commands.spawn((
        Sprite {
            color: Color::from(RED),
            custom_size: Some(Vec2::splat(8.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
        Player{
            dx: 0.,
            dy: 0.,
            angle: 0.,
        },
    ));
}

#[derive(Component)]
pub struct Player {
    pub dx: f32,
    pub dy: f32,
    pub angle: f32
}


pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut player_transform, _)) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            player_transform.translation += direction * 200. * time.delta_secs();
        }
    }
}

fn player_sight_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    if let Ok(mut player) = player_query.single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.angle += 0.025;

            if player.angle > 2.*PI as f32 {
                player.angle -= 2.*PI as f32;
            }
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.angle -= 0.025;

            if player.angle < 0. {
                player.angle += 2.*PI as f32;
            }
        }
    }
}

fn draw_player_sight(
    mut gizmos: Gizmos,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((player_transform, player)) = player_query.single_mut() {
        let start = player_transform.translation.xy();
        let length = 25.;

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
