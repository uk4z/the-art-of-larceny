pub mod components;
pub mod systems;

use bevy_ui_borders::BordersPlugin;
use systems::*;

use bevy::prelude::*;

use crate::{AppState, game::components::Level};

pub const MAIN_SIZE: (f32, f32) = (3260.0, 2240.0);
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BordersPlugin)
            .insert_resource(Level::Starting)
            // OnEnter State Systems
            .add_systems(
                (
                    spawn_main_menu,
                    spawn_level_menu,
                    spawn_main_image,
                ).in_schedule(OnEnter(AppState::MainMenu)))
            // Systems
            .add_systems(
                (
                    interact_with_play_button, 
                    interact_with_quit_button, 
                    interact_with_select_button, 
                    //update_level_image_on_resize, 
                    switch_level, 
                    display_level_title,
                    update_level_image,
                    update_main_image_on_resize,
                    update_main_menu_on_resize,
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_systems(
                (   
                    despawn_main_image,
                    despawn_level_image, 
                    despawn_main_menu,
                    despawn_level_menu,
                ).in_schedule(OnExit(AppState::MainMenu)));
    }
}

pub fn get_main_scale_from_window(
    window_w: &f32, 
    window_h: &f32,
) -> f32 {
    let w_scale = window_w/MAIN_SIZE.0;
    let h_scale = window_h/MAIN_SIZE.1;
    if w_scale > h_scale {
        h_scale
    } else {
        w_scale
    }
}