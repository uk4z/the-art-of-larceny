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
            .add_system(spawn_board.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    clean_helper,
                    unlock_animation,
                    unlock_target,
                    button_system,
                    show_item_found,
                    display_stealth,
                    resize_intel_menu,
                    handle_intel_visibility,
                    display_intel_label,
                    switch_section,
                ) 
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}