use bevy::prelude::*;

mod player; 
use player::PlayerPlugin;

mod systems;
use systems::*;

const MAP: [u8; 64] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 1, 1, 1, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1 ,1 ,1,
]; 

fn main() {
    App::new()
        .add_systems(Startup, spawn_map)
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .run();
}
