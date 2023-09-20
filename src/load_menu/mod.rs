pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;

use crate::game::components::SimulationState;

pub struct LoadMenuPlugin;

impl Plugin for LoadMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(SimulationState::Loading), spawn_load_menu)
            // Systems
            .add_systems(Update, interact_with_start_button)
            // OnExit State System
            .add_systems(OnExit(SimulationState::Loading), despawn_load_menu);
    }
}