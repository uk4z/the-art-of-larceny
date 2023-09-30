pub mod components;
pub mod systems;

use systems::*;

use bevy::prelude::*;

use crate::{AppState, game::{components::{Level, KeyBoard}, playground::scenery::components::BoundsResource}};

use self::components::{SelectValue, UpField, RightField, DownField, LeftField, RunField, InteractField};


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
                    spawn_settings_menu, 
                    spawn_music, 
                    spawn_select_value, 
            ))
            // Systems
            .add_systems(Update, (
                    interact_with_play_button, 
                    interact_with_quit_button, 
                    interact_with_select_button, 
                    interact_with_settings_button, 
                    interact_with_close_button, 
                    interact_with_settings_key_button, 
                    select_level_on_return,
                    //update_level_image_on_resize, 
                    switch_level, 
                    display_level_title,
                    update_level_image,
                    update_main_image_on_resize,
                    update_main_menu_on_resize,
                    handle_key_selection,
                    handle_selection_display, 
                    update_keycode_on_selection,
            ))
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), (
                    despawn_main_image,
                    despawn_level_image, 
                    despawn_main_menu,
                    despawn_level_menu,
                    despawn_settings_menu,
                    despawn_music, 
                    despawn_select_value, 
                    despawn_settings_menu, 
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

pub fn handle_selection_display(
    keyboard: ResMut<KeyBoard>, 
    mut upfield_q: Query<&mut Text, With<UpField>>,
    mut downfield_q: Query<&mut Text, (Without<UpField>, With<DownField>)>,
    mut rightfield_q: Query<&mut Text, (Without<UpField>, Without<DownField>, With<RightField>)>,
    mut leftfield_q: Query<&mut Text, (Without<UpField>, Without<DownField>, Without<RightField>, With<LeftField>)>,
    mut runfield_q: Query<&mut Text, (Without<UpField>, Without<DownField>, Without<RightField>, Without<LeftField>, With<RunField>)>,
    mut interactfield_q: Query<&mut Text, (Without<UpField>, Without<DownField>, Without<RightField>, Without<LeftField>, Without<RunField>, With<InteractField>)>,
) {

    if !keyboard.up.is_none() {
        if let Ok(mut up_text) = upfield_q.get_single_mut() {
            up_text.sections[0].value = keycode_to_string(&keyboard.up.unwrap()); 
        }
    }

    if !keyboard.down.is_none() {
        if let Ok(mut down_text) = downfield_q.get_single_mut() {
            down_text.sections[0].value = keycode_to_string(&keyboard.down.unwrap()); 
        }
    }

    if !keyboard.right.is_none() {
        if let Ok(mut right_text) = rightfield_q.get_single_mut() {
            right_text.sections[0].value = keycode_to_string(&keyboard.right.unwrap()); 
        }
    }

    if !keyboard.left.is_none() {
        if let Ok(mut left_text) = leftfield_q.get_single_mut() {
            left_text.sections[0].value = keycode_to_string(&keyboard.left.unwrap()); 
        }
    }

    if !keyboard.run.is_none() {
        if let Ok(mut run_text) = runfield_q.get_single_mut() {
            run_text.sections[0].value = keycode_to_string(&keyboard.run.unwrap()); 
        }
    }

    if !keyboard.interact.is_none() {
        if let Ok(mut interact_text) = interactfield_q.get_single_mut() {
            interact_text.sections[0].value = keycode_to_string(&keyboard.interact.unwrap()); 
        }
    }
}

pub fn update_keycode_on_selection(
    visibility_q: Query<&Visibility, With<SelectValue>>, 
    keyboard_input: Res<Input<KeyCode>>, 
    mut keyboard: ResMut<KeyBoard>, 
) {
    if let Ok(visibility) = visibility_q.get_single() {
        match *visibility {
            Visibility::Visible => {
                for keycode in keyboard_input.get_just_pressed() { 
                    if keyboard.up.is_none() {
                        keyboard.up = Some(*keycode); 
                    }

                    if keyboard.down.is_none() { 
                        keyboard.down = Some(*keycode);
                    }

                    if keyboard.right.is_none() {
                        keyboard.right = Some(*keycode);
                    }

                    if keyboard.left.is_none() {
                        keyboard.left = Some(*keycode);
                    }

                    if keyboard.run.is_none() {
                        keyboard.run = Some(*keycode); 
                    }

                    if keyboard.interact.is_none() {
                        keyboard.interact = Some(*keycode);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn keycode_to_string(
    keycode: &KeyCode, 
) -> String {
    let key = match *keycode {
        KeyCode::A => {"A"},
        KeyCode::B => {"B"},
        KeyCode::C => {"C"},
        KeyCode::D => {"D"},
        KeyCode::E => {"E"},
        KeyCode::F => {"F"},
        KeyCode::G => {"G"},
        KeyCode::H => {"H"},
        KeyCode::I => {"I"},
        KeyCode::J => {"J"},
        KeyCode::K => {"K"},
        KeyCode::L => {"L"},
        KeyCode::M => {"M"},
        KeyCode::N => {"N"},
        KeyCode::O => {"O"},
        KeyCode::P => {"P"},
        KeyCode::Q => {"Q"},
        KeyCode::R => {"R"},
        KeyCode::S => {"S"},
        KeyCode::T => {"T"},
        KeyCode::U => {"U"},
        KeyCode::V => {"V"},
        KeyCode::W => {"W"},
        KeyCode::X => {"X"},
        KeyCode::Y => {"Y"},
        KeyCode::Z => {"Z"},
        KeyCode::Numpad0 => {"Num0"},
        KeyCode::Numpad1 => {"Num1"},
        KeyCode::Numpad2 => {"Num2"},
        KeyCode::Numpad3 => {"Num3"},
        KeyCode::Numpad4 => {"Num4"},
        KeyCode::Numpad5 => {"Num5"},
        KeyCode::Numpad6 => {"Num6"},
        KeyCode::Numpad7 => {"Num7"},
        KeyCode::Numpad8 => {"Num8"},
        KeyCode::Numpad9 => {"Num9"},
        KeyCode::AltLeft => {"AltLeft"}
        KeyCode::AltRight => {"AltRight"}
        KeyCode::Asterisk => {"*"}
        KeyCode::Apostrophe => {"'"}
        KeyCode::Comma => {","}
        KeyCode::ControlLeft => {"CtrlLeft"}
        KeyCode::Down => {"ArrowDown"},
        KeyCode::Up => {"ArrowUp"},
        KeyCode::Right => {"ArrowRight"},
        KeyCode::Left => {"ArrowLeft"},
        KeyCode::Key0 => {"0"},
        KeyCode::Key1 => {"1"},
        KeyCode::Key2 => {"2"},
        KeyCode::Key3 => {"3"},
        KeyCode::Key4 => {"4"},
        KeyCode::Key5 => {"5"},
        KeyCode::Key6 => {"6"},
        KeyCode::Key7 => {"7"},
        KeyCode::Key8 => {"8"},
        KeyCode::Key9 => {"9"},
        KeyCode::ShiftRight =>  {"ShiftRight"},
        KeyCode::ShiftLeft =>  {"ShiftLeft"},
        _ => {"Undeclared"}
    };

    key.to_string()
}