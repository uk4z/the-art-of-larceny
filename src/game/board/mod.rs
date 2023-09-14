pub mod systems;
pub mod components;

use bevy::prelude::*;

use systems::*;

use crate::AppState;

use self::components::CurrencyLocked;

use super::SimulationState;

#[derive(Debug)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrencyLocked(false))
            .add_system(spawn_helper_menu.in_schedule(OnEnter(SimulationState::Running))
            )
            .add_system(spawn_stealth_icon.in_schedule(OnEnter(SimulationState::Loading)))
            .add_systems(
                (
                    clean_helper,
                    unlock_animation,
                    update_icon,
                ) 
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_helper_menu.in_schedule(OnExit(SimulationState::Running)))
            .add_system(despawn_stealth_icon.in_schedule(OnExit(AppState::Game)));
    }
}