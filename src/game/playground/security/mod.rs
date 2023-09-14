pub mod systems; 
pub mod components; 

use bevy::prelude::*;
use systems::*;

use crate::AppState;
use crate::game::SimulationState;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;
use components::Security; 
pub struct SecurityPlugin;

impl Plugin for SecurityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_security.in_schedule(OnEnter(SimulationState::Loading)))
            .add_systems(
                (
                    signal_security, 
                    update_visibility,
                    desactivate_security 
                ) 
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_security.in_schedule(OnExit(AppState::Game)));
    }
}

pub fn interaction_allowed_for_security (
    player_q: &Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Security>)>,
    security_q: &Query<(&WorldPosition, &ReachDistance), (With<Security>, Without<Player>)>,
) -> bool {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        if let Ok((security_position, security_reach)) = security_q.get_single() {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*security_position));
            if distance <= player_reach.0+security_reach.0 {
                return true
            }
        }
    }
    false
}