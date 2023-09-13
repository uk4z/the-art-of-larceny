use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};
use rand::prelude::*;

use super::components::*;
use crate::components::Layer;
use crate::game::playground::player::components::{Stealth, Player};
use crate::game::playground::scenery::{get_scenery_scale_from_window, SCENERY_SIZE};
use crate::game::playground::target::components::{UnlockTimer, Target};
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::target::interaction_allowed_for_target;

pub fn spawn_stealth_icon(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    commands.spawn(
        (SpriteBundle{
            texture: asset_server.load("stealth_icon/ninja.png"),
            transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),
        ..default()
        },
        StealthIcon,
        WorldPosition {x: 135.0, y: 125.0},
    ));

}

pub fn despawn_stealth_icon(
    mut commands: Commands, 
    stealth_icon_q: Query<Entity, With<StealthIcon>>,
) {
    for entity in stealth_icon_q.iter() {
        commands.entity(entity).despawn();
    };
    
}

pub fn spawn_helper_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column, 
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::End, 
                align_items: AlignItems::Center,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
            ..default()
        },
        HelperMenu,
    )).with_children(|helper|{
        helper.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                ..default()
            },
            visibility: Visibility::Visible,
            ..default()
        }).with_children(|text_division|{
            text_division.spawn((TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    }),
                    background_color: Color::BLACK.into(),
                    ..default()
            },
            Helper,
            ));
        });
    });
}

pub fn despawn_helper_menu(
    mut commands: Commands, 
    helper_menu_q: Query<Entity, With<HelperMenu>>,
) {
    for entity in helper_menu_q.iter() {
        commands.entity(entity).despawn_recursive();
    };
    
}

pub fn clean_helper (
    mut helper_q: Query<&mut Text, With<Helper>>,
) {
    //This function is called at the begining of each frame to clean the helper section of any given text. 
    //Therefore, the BoardPlugin has to be added before the PlaygroundPlugin in the LevelPlugin.
    if let Ok(mut text) = helper_q.get_single_mut() {
        text.sections[0].value = "".to_string();
    }
}


pub fn unlock_animation (
    mut password_q: Query<&mut Text, With<Helper>>,
    timer_q: Query<&UnlockTimer, With<Target>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Target>)>,
    target_q: Query<(&WorldPosition, &ReachDistance), (With<Target>, Without<Player>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if interaction_allowed_for_target(player_q, target_q) {
        if let Ok(mut text) = password_q.get_single_mut() {
            if let Ok(timer) = timer_q.get_single() {
                if keyboard_input.pressed(KeyCode::E) && !timer.0.finished(){
                    let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
                    let mut rng = thread_rng();
        
                    let number = (timer.0.percent_left()*100.0/20.0) as usize; 
        
                    let mut random_password = "".to_string();
                    for _ in 0..5 {
                        let index: usize = rng.gen_range(0..data.len()-1);
                        random_password.push_str(&data[index..index+1]);
                    }
                    let correct_password = "clear".to_string();
        
                    if number <= 0 {
                        text.sections[0].value = correct_password; 
                    } else if number >= 5 {
                        text.sections[0].value = random_password; 
                    } else {
                        text.sections[0].value = format!{"{}{}", &random_password[..number], &correct_password[number..]};
                    }
                }
            }
        } 
    }
    
}

/* pub fn show_item_found(
    item_q: Query<(&components::Name, &components::Value, &Visibility), With<Item>>, 
    mut board_item_q: Query<&mut Text, With<ItemBoard>>,
) {
    let mut items: Vec<String> = item_q.iter().map(|(name, value, visibility)| {
        let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
        let mut rng = thread_rng();
        let mut random_text = "".to_string();
        for _ in 0..4 {
            let index: usize = rng.gen_range(0..data.len()-1);
            random_text.push_str(&data[index..index+1]);
        }
        let item = match visibility {
            Visibility::Hidden => {format!("{}: {}", name.0, value.0)},
            _ => {format!("{}: {}", name.0, random_text)},
        };
        return item
    }).collect();

    board_item_q.for_each_mut(|mut text| {
        text.sections[0].value = match items.pop(){
            Some(item) => {item},
            None => {"".to_string()},
        };
    })
} */

pub fn update_icon(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stealth_q: Query<&Stealth, With<Player>>,
    stealth_icon_q: Query<Entity, With<StealthIcon>>,
) {
    if let Ok(stealth) = stealth_q.get_single() {
        if let Ok(icon) = stealth_icon_q.get_single() {
            commands.entity(icon).despawn();

            let path = match *stealth {
                Stealth::Begineer => {
                    "stealth_icon/camera.png"
                }
                Stealth::Engineer => {
                    "stealth_icon/ninja.png"
                }
                Stealth::Ghost => {
                    "stealth_icon/ninja.png"
                }
                Stealth::None => {
                    "stealth_icon/eye.png"
                }
            };

            let window = window_q.get_single().unwrap();
            let width = window.width();
            let height = window.height();
            let scale = get_scenery_scale_from_window(&width, &height);

            let x_step = (width - scale*SCENERY_SIZE.0)/2.0;
            let y_step = (height - scale*SCENERY_SIZE.1)/2.0;
            let position = Vec3::new(x_step, y_step, 0.0) + Vec3::new(135.0, 125.0, 0.0)*scale;

            commands.spawn(
                (SpriteBundle{
                    texture: asset_server.load(path),
                    transform: Transform::from_xyz(position.x, position.y, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),
                ..default()
                },
                StealthIcon,
                WorldPosition {x: 135.0, y: 125.0},
            ));
            
        }
    }
}