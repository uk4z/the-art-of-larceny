pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;

use crate::game::components::SimulationState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // Systems
            .add_systems(
                (interact_with_resume_button, interact_with_exit_button)
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            // OnExit State System
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}