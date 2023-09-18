pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;

use crate::{AppState, game::SimulationState};

pub const SCENERY_SIZE: (f32, f32) = (3360.0, 2240.0); //In pixel size

pub struct SceneryPlugin;

impl Plugin for SceneryPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_scenery.in_schedule(OnEnter(SimulationState::Loading))
            )
            .add_system(
                set_bounds
                .in_set(OnUpdate(SimulationState::Loading))
            )
            .add_system(despawn_scenery.in_schedule(OnExit(AppState::Game)));
    }
}