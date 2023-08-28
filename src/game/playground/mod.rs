pub mod player;
pub mod target;
pub mod scenery;
pub mod systems;
pub mod components;
pub mod extraction;
pub mod item;

use bevy::prelude::*;

use systems::*;
use player::PlayerPlugin;
use scenery::{SceneryPlugin, SCENERY_SIZE};
use target::TargetPlugin;
use extraction::ExtractionPlugin;
use item::ItemPlugin;


pub const WORLD_SCALE: f32 = 80.0; //80 pixels = 1 m 
pub const CHARACTER_SIZE: f32 = 50.0; //In pixels

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(SceneryPlugin)
            .add_plugin(TargetPlugin)
            .add_plugin(ExtractionPlugin)
            .add_plugin(ItemPlugin)
            .add_system(confine_position)
            .add_system(update_scale)
            .add_system(world_to_screen);
    }
}


pub fn get_world_origin(
    resolution_scale: f32, 
    scenery_center: (f32, f32)
) -> Vec3 {
    let height = resolution_scale*SCENERY_SIZE.1;
    let width = resolution_scale*SCENERY_SIZE.0;

    Vec3::new(scenery_center.0-width/2.0, scenery_center.1-height/2.0, 0.0)
}

pub fn is_visible(
    visibility: &Visibility,
) -> bool {
    match visibility {
        Visibility::Hidden => {
            false
        },
        _ => {
            true
        }
    }
}