pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;
use components::*;
use crate::AppState;
use crate::game::SimulationState;
use crate::game::board::systems::clean_helper;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Loading), spawn_target)
            .add_systems(Update, (
                    signal_target_zone.after(clean_helper), 
                    animate_sound.before(load_target_unlock), 
                    load_target_unlock, 
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_target);
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