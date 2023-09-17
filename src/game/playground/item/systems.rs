use bevy::prelude::*;
use crate::components::Layer;
use crate::game::components::{Level, ItemCount};
use crate::game::bundle::item::get_item_bundle;
use crate::game::playground::is_visible;
use super::components::*;
use crate::game::playground::components::{WorldPosition, ReachDistance, Orientation};
use crate::game::playground::player::components::Player;
use crate::game::board::components::Helper;
use super::interaction_allowed_for_item;


pub fn spawn_item (
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    level: Res<Level>
) {
    let scale = 30.0/200.0; 

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
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Item>)>,
    item_q: Query<(Entity, &WorldPosition, &ReachDistance), (With<Item>, Without<Player>)>,
    mut count: ResMut<ItemCount>, 
    keyboard_input: ResMut<Input<KeyCode>>,
    mut commands: Commands,

) {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        for (entity, item_position, item_reach) in item_q.iter() {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*item_position));
            if distance <= player_reach.0+item_reach.0 && keyboard_input.pressed(KeyCode::E)  {
                    commands.entity(entity).despawn(); 
                    count.0 += 1; 
            }
        }
    }
}

pub fn rotate_item(
    mut orientation_q: Query<&mut Orientation, With<Item>>,
) {

    orientation_q.for_each_mut(|mut orientation| { 
        orientation.0 =  orientation.0.mul_quat(Quat::from_rotation_z(0.01)); 
    }); 
}