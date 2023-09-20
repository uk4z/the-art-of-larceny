pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

use self::components::CurrencyLocked;

use super::{SimulationState, playground::target::systems::signal_target_zone};

#[derive(Debug)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrencyLocked(false))
            .add_systems(OnEnter(SimulationState::Running),spawn_helper_menu)
            .add_systems(OnEnter(SimulationState::Loading), spawn_stealth_icon)
            .add_systems(Update, (
                    clean_helper,
                    unlock_animation.after(signal_target_zone),
                    update_icon,
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(SimulationState::Running),despawn_helper_menu)
            .add_systems(OnExit(AppState::Game), despawn_stealth_icon);
    }
}