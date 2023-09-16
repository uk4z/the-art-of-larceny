pub mod playground;
pub mod board; 
pub mod systems;
pub mod bundle;
pub mod components;


use std::time::Instant;

use bevy::prelude::*;

use crate::AppState;

use self::playground::PlaygroundPlugin;
use self::board::BoardPlugin;
use systems::*;
use components::*;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ScoreEvent>()
            .insert_resource(ItemCount(0))
            .insert_resource(GameTime(Instant::now()))
            .add_plugin(PlaygroundPlugin)
            .add_plugin(BoardPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(handle_game_over.run_if(in_state(AppState::Game)))
            .add_system(handle_level_complete.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}