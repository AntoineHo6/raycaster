use bevy::{camera::Viewport, prelude::*};

pub fn setup_window(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().unwrap();
    window.resolution.set(1024., 512.);
    window.resizable = false;
}

pub fn spawn_cameras(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set the window res
    let window = windows.single_mut().unwrap();

    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.width() as u32 / 2, window.height() as u32),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            viewport: Some(Viewport {
                physical_position: UVec2::new(window.width() as u32 / 2, 0),
                physical_size: UVec2::new(window.width() as u32 / 2, window.height() as u32),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(window.width() * 0.75, 0., 1.0),
    ));
}
