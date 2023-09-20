pub mod systems;
pub mod components;

use bevy::prelude::*;
use crate::AppState;
use crate::game::SimulationState;
use crate::game::board::systems::clean_helper;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;
use components::Footage;

use systems::*;
pub struct FootagePlugin;

impl Plugin for FootagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Loading), spawn_footage)
            .add_systems(Update, (
                signal_footage.after(clean_helper), 
                suppress_footage, 
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_footage);
    }
}

pub fn interaction_allowed_for_footage (
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Footage>)>,
    footage_q: Query<(&WorldPosition, &ReachDistance), (With<Footage>, Without<Player>)>,
) -> bool {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        if let Ok((footage_position, footage_reach)) = footage_q.get_single() {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*footage_position));
            if distance <= player_reach.0+footage_reach.0 {
                return true
            }
        }
    }
    false
}