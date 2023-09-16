pub mod systems;
pub mod game;
pub mod components;
pub mod main_menu;
pub mod pause_menu;
pub mod load_menu;
pub mod score_menu; 

use bevy::prelude::*;
use bevy::window::{Window, WindowMode, PresentMode}; 

use game::GamePlugin;
use game::components::SimulationState;
use main_menu::MainMenuPlugin;
use pause_menu::PauseMenuPlugin;
use load_menu::LoadMenuPlugin; 
use score_menu::ScoreMenuPlugin;
use systems::*;


fn main() {

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "The art of larceny: rogue's riches".to_string(),
            mode: WindowMode::Windowed,
            resize_constraints: WindowResizeConstraints { min_width: 1680.0, min_height: 720.0, max_width: 2440.0, max_height: 1280.0 },
            resizable: true,
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    };


    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_state::<AppState>()
        .add_state::<SimulationState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(LoadMenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(PauseMenuPlugin)
        .add_plugin(ScoreMenuPlugin)
        .add_startup_system(spawn_setup)
        .add_system(debug_window_size) //To close the window when pressing 'ESC' key
        .add_system(request_resize)
        .add_system(update_camera_position)
        .add_system(start_level)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    None, 
}