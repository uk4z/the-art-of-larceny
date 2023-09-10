pub mod systems;
pub mod components;

use bevy::prelude::*;
use crate::AppState;
use crate::game::SimulationState;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;
use components::Footage;

use systems::*;
pub struct FootagePlugin;

impl Plugin for FootagePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_footage.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    signal_footage, 
                    suppress_footage, 
                ) 
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
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