use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::game::playground::components::{WorldPosition, ReachDistance};
use super::interaction_allowed_for_security;
use super::components::*;
use crate::game::board::components::Helper;
use crate::game::playground::player::components::Player;
use crate::game::playground::camera::components::Camera;
use crate::game::playground::laser::components::Laser;

pub fn spawn_security(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
            transform: Transform::from_xyz(1400.0, 250.0, 4.0),
            material: materials.add(ColorMaterial::from(Color::NAVY)),
            visibility: Visibility::Hidden,
            ..default()
        },
        SecurityBundle {
            position: WorldPosition {x: 710.0, y: 975.0},
            intrusion: Intrusion(false),
            active: Active(true),
            reach: ReachDistance(40.0),
        },
        Security,
    )); 
}

pub fn signal_security (
    mut help_q: Query<&mut Text, With<Helper>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Security>)>,
    security_q: Query<(&WorldPosition, &ReachDistance), (With<Security>, Without<Player>)>, 
    visibility_q: Query<&Active, With<Security>>,
) {
    if let Ok(active) = visibility_q.get_single() {
        if active.0 {
            if let Ok(mut text) = help_q.get_single_mut() {
                if interaction_allowed_for_security(&player_q, &security_q) {
                    text.sections[0].value = "Press D to desactivate the security".to_string();
                }
            }
        } else {
            if let Ok(mut text) = help_q.get_single_mut() {
                if interaction_allowed_for_security(&player_q, &security_q) {
                    text.sections[0].value = "Press D to activate the security".to_string();
                }
            }
        }
    };
}

pub fn desactivate_security (
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Security>)>,
    security_q: Query<(&WorldPosition, &ReachDistance), (With<Security>, Without<Player>)>, 
    mut active_q: Query<&mut Active, With<Security>>,
    keyboard_input: Res<Input<KeyCode>>, 
) {
    if interaction_allowed_for_security(&player_q, &security_q) {
        if keyboard_input.just_pressed(KeyCode::D) {
            if let Ok(mut active) = active_q.get_single_mut() {
                active.0 = !active.0; 
                
            }
        }
    }
}

pub fn update_visibility(
    active_q: Query<&Active, With<Security>>,
    mut cameras_q: Query<&mut Visibility, (With<Camera>, Without<Laser>)>,
    mut lasers_q: Query<&mut Visibility, (With<Laser>, Without<Camera>)>,
) {
    if let Ok(active) = active_q.get_single() {
        if !active.0 {
            cameras_q.for_each_mut(|mut visibility|{
                *visibility = Visibility::Hidden; 
            });
            lasers_q.for_each_mut(|mut visibility|{
                *visibility = Visibility::Hidden; 
            });
        } else {
            cameras_q.for_each_mut(|mut visibility|{
                *visibility = Visibility::Visible; 
            });
            lasers_q.for_each_mut(|mut visibility|{
                *visibility = Visibility::Visible; 
            });
        }
        
    }
    
        
}