use bevy::prelude::*;

use crate::game::{SimulationState, playground::player::components::Stealth};

use super::{playground::{components::GameOver, extraction::components::LevelCompleted}, ScoreEvent};

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
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut game_over_event: EventReader<GameOver>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    for _ in game_over_event.iter() {
        score_event.send(ScoreEvent { comment: "You lost !".to_string()});
        simulation_state_next_state.set(SimulationState::Score);
        println!("Entered AppState::ScoreMenu");
    }
}

pub fn handle_level_complete(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut level_complete_event: EventReader<LevelCompleted>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    for level in level_complete_event.iter() {
        let stealth = level.stealth;

        let comment: String = match stealth {
            Stealth::Begineer => {
                let stealth_comment = "Your footprints will have to suppressed. You are not a manager, it is not your job to add some more work to the task \
                                    at hand. It is okay to be seen, but it is not to let your mistakes be handled by other. You can do better!";
                stealth_comment.to_string()
            },
            Stealth::Engineer => {
                let stealth_comment = "Great recovery. You have been spotted by one of the security devices but managed to suppress the footage quickly \
                            enough. Missions require brain and you seem to have engineering skills. Good job!";
                stealth_comment.to_string()
            },
            Stealth::Ghost => {
                let stealth_comment = "No one has seen you. Perfectly silent, you got through your mission with great composure and managed \
                    to get out without alerting the guards. As if you were a ghost. Good job!";

                stealth_comment.to_string()
            },
            Stealth::None => {
                "You have been seen you dumbass!. Now you will have blood on you hands. Poor guys, they were not supposed to be shortlived.".to_string()
            }
        };

        score_event.send(ScoreEvent { comment});
        simulation_state_next_state.set(SimulationState::Score);
        println!("Entered AppState::ScoreMenu");
    }
}