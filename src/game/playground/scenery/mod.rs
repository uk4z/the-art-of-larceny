pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;

use crate::{AppState, game::SimulationState};

pub const SCENERY_SIZE: (f32, f32) = (2240.0, 1280.0); //In pixel size

pub struct SceneryPlugin;

impl Plugin for SceneryPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems( (
                spawn_scenery,
                spawn_bounds_resource,
                )
                .in_schedule(OnEnter(SimulationState::Loading))
            )
            .add_system(
                set_bounds
                .in_set(OnUpdate(SimulationState::Loading))
            )
            .add_system(despawn_scenery.in_schedule(OnExit(AppState::Game)));
    }
}


pub fn get_scenery_scale_from_window(
    window_w: &f32, 
    window_h: &f32,
) -> f32 {
    let w_scale = window_w/SCENERY_SIZE.0;
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
    let x = window_w/2.0;
    let y = window_h/2.0;
    (x, y)
}