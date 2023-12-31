use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use crate::AppState;
use crate::main_menu::components::MainMenu;

use super::components::Layer;

pub fn debug_window_size(
    mut resize_event: EventReader<WindowResized>,
) {
    for window in resize_event.iter() {
        println!("width = {}, height = {}", window.width, window.height);
    }
}


pub fn spawn_setup(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, Layer::Camera.into()), //Origin-Bottom left;Y-Up;X-right
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0    , 0.0, 0.0)),
            ..default()
        },
        ..default()
    },
    RenderLayers::from_layers(&[0, 1, 2]),
    ));
}


pub fn update_camera_position(
    mut resize_event: EventReader<WindowResized>,
    mut transform_q: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mut transform) = transform_q.get_single_mut() {
        for window in resize_event.iter() {
            transform.translation = Vec3::new(window.width/2.0, window.height/2.0, Layer::Camera.into())      
        }
    } 
}

pub fn start_level(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut visibility_q: Query<&mut Visibility, With<MainMenu>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) { 
        if *app_state.get() != AppState::Game {
            if let Ok(mut visibility) = visibility_q.get_single_mut() {
                app_state_next_state.set(AppState::Game);
                *visibility = Visibility::Hidden;
            }
        }
    }   
}