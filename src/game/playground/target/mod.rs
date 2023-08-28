pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;
use components::*;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_target)
            .add_system(signal_target_zone)
            .add_system(load_target_unlock);
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