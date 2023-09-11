use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::game::playground::components::{WorldPosition, ReachDistance};

use super::components::{Footage, FootageBundle, Available};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::board::components::Helper;
use super::interaction_allowed_for_footage;

pub fn spawn_footage (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
            transform: Transform::from_xyz(2300.0, 817.0, 4.0),
            material: materials.add(ColorMaterial::from(Color::FUCHSIA)),
            visibility: Visibility::Hidden,
            ..default()
        },
        FootageBundle {
            position: WorldPosition {
                x: 1800.0,
                y: 817.0,
            },
            reach: ReachDistance(40.0),
            available: Available(true),
        },
        Footage,
    ));
}

pub fn despawn_footage(
    mut commands: Commands,
    entity_q: Query<Entity, With<Footage>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn signal_footage (
    mut help_q: Query<&mut Text, With<Helper>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Footage>)>,
    footage_q: Query<(&WorldPosition, &ReachDistance), (With<Footage>, Without<Player>)>,
    available_q: Query<&Available, With<Footage>>, 
) {
    if let Ok(mut text) = help_q.get_single_mut() {
        if interaction_allowed_for_footage(player_q, footage_q) {
            if let Ok(available) = available_q.get_single() {
                if available.0 {
                    text.sections[0].value = "Press S to suppress the footage".to_string();
                }
            }
        }
    }
}

pub fn suppress_footage(
    keyboard_input: Res<Input<KeyCode>>, 
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Footage>)>,
    footage_q: Query<(&WorldPosition, &ReachDistance), (With<Footage>, Without<Player>)>,
    mut available_q: Query<&mut Available, With<Footage>>,
    mut stealth_q: Query<&mut Stealth, With<Player>>,
) {
    if interaction_allowed_for_footage(player_q, footage_q) {
        if keyboard_input.just_pressed(KeyCode::S) {
            if let Ok(mut available) = available_q.get_single_mut() {
                if let Ok(mut stealth) = stealth_q.get_single_mut() {
                    available.0 = false;
                    match *stealth {
                        Stealth::Begineer => {
                            *stealth = Stealth::Engineer;
                        },
                        _ => {},
                    };
                    println!("suppressed"); 
                }
            }
        }
    }
}