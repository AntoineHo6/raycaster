use bevy::prelude::*;

use crate::{SceneGizmos, raycast_data::resource::RaycastData};
use bevy::color::palettes::basic::RED;

pub fn render_scene(raycast_data: Res<RaycastData>, mut gizmos: Gizmos<SceneGizmos>) {
    for i in 0..raycast_data.ray_count as usize {
        let mut line_height: f32 = (64. * 512.) / raycast_data.dist[i];
        if line_height > 512. {
            line_height = 512.;
        }

        let test = 1024. - i as f32 * 8.;

        gizmos.line_2d(
            Vec2::new(test, 0.),
            Vec2::new(test, line_height),
            Color::from(RED),
        );
    }
}
