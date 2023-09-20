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
            .add_systems(OnEnter(SimulationState::Paused), spawn_pause_menu)
            // Systems
            .add_systems(Update, (
                    interact_with_resume_button, 
                    interact_with_exit_button
            ))
            // OnExit State System
            .add_systems(OnExit(SimulationState::Paused), despawn_pause_menu);
    }
}