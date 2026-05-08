use bevy::prelude::*;

mod constants;
mod utils;

mod player;
use player::PlayerPlugin;

mod systems;
use systems::*;

mod minimap;
use minimap::MinimapPlugin;

mod scene_renderer;
use scene_renderer::SceneRendererPlugin;

mod raycast_data;
use raycast_data::RaycastDataPlugin;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct RayGizmos;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SceneGizmos;

fn main() {
    App::new()
        .add_systems(Startup, spawn_cameras)
        .init_gizmo_group::<RayGizmos>() // <--- Register here!
        .init_gizmo_group::<SceneGizmos>() // <--- Register here!
        .add_systems(Startup, setup_gizmo_styles)
        .add_plugins(RaycastDataPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(MinimapPlugin)
        .add_plugins(SceneRendererPlugin)
        .run();
}

fn setup_gizmo_styles(mut config_store: ResMut<GizmoConfigStore>) {
    // Configure Ray style
    let (ray_config, _) = config_store.config_mut::<RayGizmos>();
    ray_config.line.width = 1.0;

    // Configure Player style
    let (player_config, _) = config_store.config_mut::<SceneGizmos>();
    player_config.line.width = 8.0;
}
