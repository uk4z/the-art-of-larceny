pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;


pub const SCENERY_SIZE: (f32, f32) = (2240.0, 1280.0); //In pixel size

pub struct SceneryPlugin;

impl Plugin for SceneryPlugin{
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_scenery);
    }
}


pub fn get_scenery_scale_from_window(
    window_w: &f32, 
    window_h: &f32,
) -> f32 {
    let w_scale = 0.75*window_w/SCENERY_SIZE.0;
    let h_scale = window_h/SCENERY_SIZE.1;
    if w_scale > h_scale {
        h_scale
    } else {
        w_scale
    }
}

pub fn get_scenery_position_from_window(
    window_w: &f32, 
    window_h: &f32, 
) -> (f32, f32) {
    let x = (window_w*0.75)/2.0;
    let y = window_h/2.0;
    (x, y)
}