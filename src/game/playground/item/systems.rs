use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::Layer;
use crate::game::components::Level;
use crate::game::bundle::item::get_item_bundle;
use crate::game::playground::is_visible;
use super::components::*;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::scenery::get_scenery_scale_from_window;
use crate::game::playground::player::components::Player;
use crate::game::board::components::Helper;
use super::interaction_allowed_for_item;


pub fn spawn_item (
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    level: Res<Level>
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    if let Some(items) = get_item_bundle(&level) {
        for bundle in items {
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load(bundle.path.0.clone()),
                    transform: Transform::from_xyz(500.0, 500.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),       
                ..default()
                }, 
                bundle, 
                Item, 
            ));
        }
    }
}

pub fn despawn_item(
    mut commands: Commands,
    entity_q: Query<Entity, With<Item>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
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