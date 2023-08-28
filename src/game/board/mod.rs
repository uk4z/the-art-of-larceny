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
            .add_system(lock_animation)
            .add_system(unlock_target)
            .add_system(button_system)
            .add_system(show_item_found)
            .insert_resource(CurrencyLocked(false));
    }
}