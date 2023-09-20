use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::audio::{PlaybackMode, VolumeLevel, Volume};

use crate::get_scale_reference;
use crate::components::Layer;
use crate::game::components::Level;
use crate::game::bundle::security::get_security_bundle;
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
    level: Res<Level>,    
    window_q: Query<&Window, With<PrimaryWindow>>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Some(bundle) = get_security_bundle(&level) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
                transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into())
                    .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),
                material: materials.add(ColorMaterial::from(Color::NAVY)),
                visibility: Visibility::Hidden,
                ..default()
            },
            bundle,
            Security,
        ));
    } 
}

pub fn despawn_security(
    mut commands: Commands,
    entity_q: Query<Entity, With<Security>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
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
                    text.sections[0].value = "Press E to desactivate the security".to_string();
                }
            }
        } else {
            if let Ok(mut text) = help_q.get_single_mut() {
                if interaction_allowed_for_security(&player_q, &security_q) {
                    text.sections[0].value = "Press E to activate the security".to_string();
                }
            }
        }
    };
}

pub fn toggle_security (
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Security>)>,
    security_q: Query<(&WorldPosition, &ReachDistance), (With<Security>, Without<Player>)>, 
    mut active_q: Query<&mut Active, With<Security>>,
    keyboard_input: Res<Input<KeyCode>>, 
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    if interaction_allowed_for_security(&player_q, &security_q) {
        if keyboard_input.just_pressed(KeyCode::E) {
            if let Ok(mut active) = active_q.get_single_mut() {
                if active.0 {
                    commands.spawn(
                        AudioBundle {
                            source: asset_server.load("sounds/security_off.ogg"),
                            settings: PlaybackSettings { 
                                mode: PlaybackMode::Despawn, 
                                volume: Volume::Relative(VolumeLevel::new(1.0)), 
                                speed: 1.5, paused: false }
                        },
                    );
                } else {
                    commands.spawn(
                        AudioBundle {
                            source: asset_server.load("sounds/security_on.ogg"),
                            settings: PlaybackSettings { 
                                mode: PlaybackMode::Despawn, 
                                volume: Volume::Relative(VolumeLevel::new(1.0)), 
                                speed: 1.5, paused: false}
                        },
                    );
                }
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