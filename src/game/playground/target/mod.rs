pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;
use components::*;
use crate::AppState;
use crate::game::SimulationState;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_target.in_schedule(OnEnter(SimulationState::Loading)))
            .add_systems(
                (
                    signal_target_zone, 
                    load_target_unlock, 
                ) 
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_target.in_schedule(OnExit(AppState::Game)));
    }
}


pub fn interaction_allowed_for_target (
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Target>)>,
    target_q: Query<(&WorldPosition, &ReachDistance), (With<Target>, Without<Player>)>,
) -> bool {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        if let Ok((target_position, target_reach)) = target_q.get_single() {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*target_position));
            if distance <= player_reach.0+target_reach.0 {
                return true
            }
        }
    }
    false
}

pub fn is_target_unlock(
    timer: &UnlockTimer,
) -> bool {
    if timer.0.finished() {
        true
    } else {
        false
    }
    

}