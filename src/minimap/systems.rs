use bevy::prelude::*;

use crate::constants::MAP;

pub fn spawn_minimap(mut commands: Commands) {
    for x in 0..8 {
        for y in 0..8 {
            let x_coord = -(512. / 2.0) + (x as f32 * 64.) + 32.;
            let y_coord = (512. / 2.0) - (y as f32 * 64.) - 32.;

            if MAP[y * 8 + x] == 1 {
                commands.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::splat(62.)),
                        ..default()
                    },
                    Transform::from_xyz(x_coord, y_coord, 0.),
                ));
            } else {
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