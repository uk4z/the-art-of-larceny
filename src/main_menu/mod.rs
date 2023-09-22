pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;

use crate::{AppState, game::{components::Level, playground::scenery::components::BoundsResource}};


pub const MAIN_SIZE: (f32, f32) = (3260.0, 2240.0);
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Level::Tutorial)
            //.register_type::<Best>()
            .insert_resource(BoundsResource{handle: None})

            // OnEnter State Systems
            .add_systems( OnEnter(AppState::MainMenu), (
                    spawn_main_menu,
                    spawn_level_menu,
                    spawn_main_image,
                    spawn_music, 
            ))
            // Systems
            .add_systems(Update, (
                    interact_with_play_button, 
                    interact_with_quit_button, 
                    interact_with_select_button, 
                    //update_level_image_on_resize, 
                    switch_level, 
                    display_level_title,
                    update_level_image,
                    update_main_image_on_resize,
                    update_main_menu_on_resize,
            ))
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), (
                    despawn_main_image,
                    despawn_level_image, 
                    despawn_main_menu,
                    despawn_level_menu,
                    despawn_music, 
            ));
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