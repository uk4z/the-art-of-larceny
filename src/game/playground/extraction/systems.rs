use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::audio::{Volume, PlaybackMode, VolumeLevel};


use crate::get_scale_reference;
use crate::components::Layer;
use crate::game::components::{Level, KeyBoard};
use crate::game::bundle::extraction::get_extraction_bundle;
use crate::game::playground::components::{WorldPosition, ReachDistance};
use crate::game::playground::target::components::{UnlockTimer, Target};

use super::components::{Extraction, LevelCompleted};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::board::components::Helper;
use super::interaction_allowed_for_extraction;
use crate::game::playground::target::is_target_unlock;

pub fn spawn_extraction (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
    window_q: Query<&Window, With<PrimaryWindow>>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 
    if let Some(bundle) = get_extraction_bundle(&level) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
                transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into())
                    .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                visibility: Visibility::Hidden,
                ..default()
            },
            bundle,
            Extraction,
        ));
    }
    
}

pub fn despawn_extraction(
    mut commands: Commands,
    entity_q: Query<Entity, With<Extraction>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn signal_extraction (
    mut help_q: Query<&mut Text, With<Helper>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Extraction>)>,
    extraction_q: Query<(&WorldPosition, &ReachDistance), (With<Extraction>, Without<Player>)>,
    timer_q: Query<&UnlockTimer, With<Target>>, 
) {
    if let Ok(mut text) = help_q.get_single_mut() {
        if interaction_allowed_for_extraction(player_q, extraction_q) {
            if let Ok(timer) = timer_q.get_single() {
                if is_target_unlock(timer) {
                    text.sections[0].value = "Interact to extract the target".to_string();
                }
            }
        }
    }
}

pub fn end_level(
    keyboard_input: Res<Input<KeyCode>>, 
    keyboard: Res<KeyBoard>, 
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Extraction>)>,
    extraction_q: Query<(&WorldPosition, &ReachDistance), (With<Extraction>, Without<Player>)>,
    stealth_q: Query<&Stealth, With<Player>>,
    mut level_event: EventWriter<LevelCompleted>,
    target_q: Query<&UnlockTimer, With<Target>>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    if interaction_allowed_for_extraction(player_q, extraction_q) {
        if let Ok(timer) = target_q.get_single() {
            if is_target_unlock(timer) {
                if keyboard_input.just_pressed(keyboard.interact.unwrap()) {
                    commands.spawn((
                        AudioBundle {
                            source: asset_server.load("sounds/win.ogg"),
                            settings: PlaybackSettings { 
                                mode: PlaybackMode::Despawn, 
                                volume: Volume::Relative(VolumeLevel::new(1.0)), 
                                speed: 1.0, 
                                paused: false,}
                        }, 
                    ));

                    if let Ok(stealth) = stealth_q.get_single() {
                        level_event.send(LevelCompleted {stealth: stealth.clone()})
                    }   
                }
            }
        }
    }
}

pub fn reveal_extraction(
    mut extraction_q: Query<&mut Visibility, With<Extraction>>,
    target_q: Query<&UnlockTimer, With<Target>>,
) {
    if let Ok(timer) = target_q.get_single() {
        if is_target_unlock(timer) {
            if let Ok(mut visible) = extraction_q.get_single_mut() {
                *visible = Visibility::Visible;
            }
        }
    }
}



