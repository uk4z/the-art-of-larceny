pub mod components;
pub mod systems; 

use bevy::prelude::*;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::player::components::Player;

use systems::*;
use components::Item;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_item)
            .add_system(signal_item)
            .add_system(take_item);
    }
}

pub fn interaction_allowed_for_item (
    player_q: &Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Item>)>,
    item_q: &Query<(&WorldPosition, &ReachDistance), (With<Item>, Without<Player>)>,
) -> bool {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        if let Ok((item_position, item_reach)) = item_q.get_single() {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*item_position));
            if distance <= player_reach.0+item_reach.0 {
                return true
            }
        }
    }
    false
}