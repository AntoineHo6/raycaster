use bevy::prelude::*;

mod constants;

mod player; 
use player::PlayerPlugin;

mod systems;
use systems::*;

mod minimap;
use minimap::MinimapPlugin;

mod scene_renderer;
use scene_renderer::SceneRendererPlugin;

fn main() {
    App::new()
        .add_systems(Startup, spawn_cameras)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(MinimapPlugin)
        .add_plugins(SceneRendererPlugin)
        .run();
}
