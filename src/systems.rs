use bevy::{camera::Viewport, prelude::*};

pub fn spawn_cameras(mut commands: Commands, mut windows: Query<&mut Window>) {
    // Set the window res
    let mut window = windows.single_mut().unwrap();
    window.resolution.set(1024., 512.);
    window.resizable = false;

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(512, 512),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(512, 0),
                physical_size: UVec2::new(512, 512),
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(768.0, 0.0, 1.0),
    ));
}
