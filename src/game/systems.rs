use bevy::prelude::*;

use crate::game::{SimulationState, playground::player::components::Stealth};

use super::{playground::{components::{GameOver, Ambiance}, extraction::components::LevelCompleted, guard::components::Guard}, ScoreEvent, components::{ItemCount, GameTime}};

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
    ambiance_sink: Query<&AudioSink, With<Ambiance>>, 
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(sink) = ambiance_sink.get_single() {
            sink.toggle(); 
        }
        
        if *simulation_state.get() == SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
        }
        if *simulation_state.get() == SimulationState::Paused {
            simulation_state_next_state.set(SimulationState::Running);
        }
    }
}

pub fn handle_game_over(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut game_over_event: EventReader<GameOver>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    for _ in game_over_event.iter() {
        score_event.send(ScoreEvent { comment: "You lost !".to_string(), value: 0});
        simulation_state_next_state.set(SimulationState::Score);
    }
}

pub fn handle_level_complete(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut level_complete_event: EventReader<LevelCompleted>,
    mut score_event: EventWriter<ScoreEvent>,
    time: Res<GameTime>,
    count: Res<ItemCount>,  
    guard_q: Query<Entity, With<Guard>>,
) {
    for level in level_complete_event.iter() {
        let stealth = level.stealth;


        let gems_score = count.0 as u64 * 25000;
        let nb_guards = guard_q.iter().len() as u64; 
        let guard_score = nb_guards * 25000; 

        let target_score: u64 = 100000; 

        let time_score = 100000*((3600-time.0.elapsed().as_secs())/3600);

        let stealth_coefficient = match stealth {
            Stealth::None => {
                1
            }, 
            Stealth::Begineer => {
                2
            },
            _ => {
                4
            }
        };

        let value = (gems_score + time_score + target_score + guard_score)*stealth_coefficient; 
        let total_score = format!(" {}", value);

        score_event.send(ScoreEvent {comment: total_score, value});
        simulation_state_next_state.set(SimulationState::Score);
    }
}