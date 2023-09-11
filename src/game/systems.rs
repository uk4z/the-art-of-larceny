use bevy::prelude::*;

use crate::{game::SimulationState, AppState};

use super::playground::components::GameOver;

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if simulation_state.0 == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.0 == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
            println!("Simulation Running.");
        }
    }
}

pub fn handle_game_over(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_over_event: EventReader<GameOver>,
) {
    for _ in game_over_event.iter() {
        app_state_next_state.set(AppState::ScoreMenu);
        println!("Entered AppState::ScoreMenu");
    }
}