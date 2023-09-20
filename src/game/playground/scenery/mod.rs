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
            .add_systems(OnEnter(SimulationState::Loading),spawn_scenery)
            .add_systems(Update,set_bounds)
            .add_systems(OnExit(AppState::Game), despawn_scenery);
    }
}