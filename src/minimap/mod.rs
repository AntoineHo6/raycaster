use bevy::prelude::*;

mod systems;
use systems::*;

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_minimap);
    }
}