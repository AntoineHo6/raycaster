use bevy::prelude::*;

mod player; 
use player::PlayerPlugin;

mod systems;
use systems::*;

fn main() {
    App::new()
        .add_systems(Startup, spawn_map)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .run();
}
