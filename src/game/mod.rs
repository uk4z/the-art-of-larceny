pub mod playground;
pub mod board; 
pub mod systems;

use bevy::prelude::*;

use crate::AppState;

use self::playground::PlaygroundPlugin;
use self::board::BoardPlugin;
use systems::*;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugin(PlaygroundPlugin)
            .add_plugin(BoardPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(handle_game_over.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    Loading,
}