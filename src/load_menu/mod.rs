pub mod components;
pub mod systems;
pub mod level_story;

use systems::*;

use bevy::prelude::*;

use crate::game::SimulationState;

pub struct LoadMenuPlugin;

impl Plugin for LoadMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_load_menu.in_schedule(OnEnter(SimulationState::Loading)))
            // Systems
            .add_system(interact_with_start_button.in_set(OnUpdate(SimulationState::Loading)))
            // OnExit State System
            .add_system(despawn_load_menu.in_schedule(OnExit(SimulationState::Loading)));
    }
}