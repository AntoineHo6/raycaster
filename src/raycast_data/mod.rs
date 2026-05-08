use bevy::prelude::*;

pub mod resource;
use resource::*;

mod systems;
use systems::*;

pub struct RaycastDataPlugin;

impl Plugin for RaycastDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RaycastData>()
        .add_systems(Update, update_rays);
    }
}