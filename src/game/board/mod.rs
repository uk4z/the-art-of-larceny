pub mod systems;
pub mod components;

use bevy::prelude::*;

use bevy_ui_borders::BordersPlugin;

use systems::*;

use self::components::CurrencyLocked;

#[derive(Debug)]
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BordersPlugin)
            .add_startup_system(spawn_board)
            .add_system(clean_helper)
            .add_system(unlock_animation)
            .add_system(unlock_target)
            .add_system(button_system)
            .add_system(show_item_found)
            .add_system(display_stealth)
            .add_system(resize_intel_menu)
            .add_system(handle_intel_visibility)
            .add_system(display_intel_label)
            .add_system(switch_section)
            .insert_resource(CurrencyLocked(false));
    }
}