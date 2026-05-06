use bevy::prelude::*;

mod systems;
use systems::*;

pub struct SceneRendererPlugin;

impl Plugin for SceneRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_scene);
    }
}