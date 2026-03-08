use bevy::prelude::*;
use bevy::color::palettes::basic::RED;

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
    .run();
}

fn setup(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set the window res
    let mut window = windows.single_mut().unwrap();
    window.resolution.set(512.0, 512.0);
    window.resizable = false;

    // Spawn camera
    commands.spawn(Camera2d);

    // Spawn walls
    for (y, row) in MAP.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 1 {
                let x_coord = x as f32 * 64. - (window.width() / 2.0) + 32.;
                let y_coord = y as f32 * 64. - (window.height() / 2.0) + 32.;
                commands.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::splat(64.)),
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
        Transform::from_xyz(0., 0., 0.),
        Player{},

    ));
}

#[derive(Component)]
pub struct Player {
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            player_transform.translation += direction * 200. * time.delta_secs();
        }
    }
}