use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::Layer;
use crate::game::playground::is_visible;
use super::components::*;
use crate::game::playground::components::{WorldPosition, Orientation, ReachDistance};
use crate::game::playground::scenery::get_scenery_scale_from_window;
use crate::game::playground::player::components::Player;
use crate::game::board::components::Helper;
use super::interaction_allowed_for_item;
use crate::game::playground::components::{Name, Value};


pub fn spawn_item (
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

     commands.spawn(
        (SpriteBundle{
            texture: asset_server.load("items/Note.png"),
            transform: Transform::from_xyz(500.0, 500.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),       
        ..default()
        }, 
        ItemBundle { 
            position: WorldPosition {x: 890.0, y: 433.0,},
            orientation: Orientation(Quat::IDENTITY),
            reach: ReachDistance(40.0),
            name: Name("Exchange rate eur/dol:".to_string()),
            value: Value(1.2),
        },
        Item, 
    ));
}

pub fn signal_item (
    item_visibility_q: Query<&Visibility, With<Item>>,
    mut help_q: Query<&mut Text, With<Helper>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Item>)>,
    item_q: Query<(&WorldPosition, &ReachDistance), (With<Item>, Without<Player>)>, 
) {
    for visibility in item_visibility_q.iter() {
        if is_visible(visibility) {
            if let Ok(mut text) = help_q.get_single_mut() {
                if interaction_allowed_for_item(&player_q, &item_q) {
                    text.sections[0].value = "Press E to take the item".to_string();
                }
            }
        }
    };
}

pub fn take_item (
    mut item_visibility_q: Query<&mut Visibility, With<Item>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Item>)>,
    item_q: Query<(&WorldPosition, &ReachDistance), (With<Item>, Without<Player>)>,
    keyboard_input: ResMut<Input<KeyCode>>,
) {
    if interaction_allowed_for_item(&player_q, &item_q) {
        item_visibility_q.for_each_mut(|mut visibility| {
            if keyboard_input.just_pressed(KeyCode::E) && is_visible(&*visibility) {
                *visibility = Visibility::Hidden;
            }
        });
        
    }
}