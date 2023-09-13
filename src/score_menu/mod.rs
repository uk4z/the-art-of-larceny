pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;

use crate::game::components::SimulationState;

pub struct ScoreMenuPlugin;

impl Plugin for ScoreMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_score_menu.in_schedule(OnEnter(SimulationState::Score)))
            // Systems
            .add_systems(
                (interact_with_restart_button, interact_with_leave_button, handle_score_event)
                    .in_set(OnUpdate(SimulationState::Score)),
            )
            // OnExit State System
            .add_system(despawn_score_menu.in_schedule(OnExit(SimulationState::Score)));
    }
}