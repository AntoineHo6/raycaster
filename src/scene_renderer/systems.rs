use bevy::{color::palettes::tailwind::{BLUE_950, RED_700}, prelude::*};

use crate::{
    SceneGizmos, player::components::Player, raycast_data::resource::RaycastData,
    utils::normalize_angle,
};
use bevy::color::palettes::basic::RED;

pub fn render_scene(
    raycast_data: Res<RaycastData>,
    mut gizmos: Gizmos<SceneGizmos>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut windows: Query<&mut Window>,
) {
    if let Ok(player) = player_query.single_mut() {
        let window = windows.single_mut().unwrap();

        for i in 0..raycast_data.ray_count as usize {
            let line_x_pos = window.width() - 4. - i as f32 * 8.;

            let angle_diff = normalize_angle(player.angle - raycast_data.angle[i]);

            let corrected_dist = raycast_data.dist[i] * angle_diff.cos(); // fix fisheye effect

            let mut line_height: f32 = (64. * window.height()) / corrected_dist;

            if line_height > window.height() {
                line_height = window.height();
            }

            let color;
            if raycast_data.hit_vert_line[i] {
                color = Color::from(RED);
            } else {
                color = Color::from(RED_700);
            }

            let line_offset = 0. - line_height / 2.;

            gizmos.line_2d(
                Vec2::new(line_x_pos, line_offset),
                Vec2::new(line_x_pos, line_height + line_offset),
                color,
            );

            gizmos.line_2d(
                Vec2::new(line_x_pos, line_offset),
                Vec2::new(line_x_pos, -256.),
                Color::from(BLUE_950),
            );
        }
    }
}
