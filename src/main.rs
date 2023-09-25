pub mod systems;
pub mod game;
pub mod components;
pub mod main_menu;
pub mod pause_menu;
pub mod load_menu;
pub mod score_menu; 

use std::time::Duration;

use bevy::asset::ChangeWatcher;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{Window, WindowMode, PresentMode}; 
use bevy_embedded_assets::EmbeddedAssetPlugin;

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
            title: "The Art of Larceny".to_string(),
            mode: WindowMode::Fullscreen,
            resizable: true,
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    };


    App::new()
        .add_plugins(
            (
            DefaultPlugins
                .set(AssetPlugin {
                watch_for_changes: Some(ChangeWatcher{delay: Duration::from_secs(1)}),
                    ..Default::default()
                    },
                )
                .set(window_plugin)
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
                LogDiagnosticsPlugin::default(),
                FrameTimeDiagnosticsPlugin::default(),
            )
        )
        .add_state::<AppState>()
        .add_state::<SimulationState>()
        .add_plugins(
            (   
                MainMenuPlugin,
                LoadMenuPlugin,
                GamePlugin,
                PauseMenuPlugin,
                ScoreMenuPlugin
        ))
        .add_systems(Startup, spawn_setup)
        //.add_startup_system(start_bgm_music)
        .add_systems(Update, (
            debug_window_size,
            update_camera_position,
            start_level
        )) //To close the window when pressing 'ESC' key
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    None, 
}

pub const WINDOW_REFERENCE: (f32, f32) = (1920.0, 1080.0); 

pub fn get_scale_reference(
    width: &f32, 
    height: &f32,
) -> f32 {
    let w_scale = width/WINDOW_REFERENCE.0;
    let h_scale = height/WINDOW_REFERENCE.1;
    if w_scale > h_scale {
        h_scale
    } else {
        w_scale
    }
}