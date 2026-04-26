use bevy::prelude::*;

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

pub fn spawn_map(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set the window res
    let mut window = windows.single_mut().unwrap();
    window.resolution.set(512.0, 512.0);
    window.resizable = false;

    // Spawn camera
    commands.spawn(Camera2d);

    // // Spawn map
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
}