pub mod systems;
pub mod components;

use bevy::prelude::*;
use crate::AppState;
use crate::game::SimulationState;
use crate::game::board::systems::clean_helper;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;
use crate::game::playground::extraction::components::Extraction;

use self::components::LevelCompleted;
use self::systems::*;
pub struct ExtractionPlugin;

impl Plugin for ExtractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LevelCompleted>()
            .add_systems(OnEnter(SimulationState::Loading), spawn_extraction)
            .add_systems(Update, (
                signal_extraction.after(clean_helper), 
                end_level, 
                reveal_extraction,
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_extraction);
    }
}

pub fn interaction_allowed_for_extraction (
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Extraction>)>,
    extraction_q: Query<(&WorldPosition, &ReachDistance), (With<Extraction>, Without<Player>)>,
) -> bool {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        if let Ok((extraction_position, extraction_reach)) = extraction_q.get_single() {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*extraction_position));
            if distance <= player_reach.0+extraction_reach.0 {
                return true
            }
        }
    }
    false
}