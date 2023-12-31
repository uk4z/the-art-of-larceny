use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::audio::{Volume, PlaybackMode, VolumeLevel};


use crate::get_scale_reference;
use crate::game::components::{Level, KeyBoard};
use crate::game::bundle::footage::get_footage_bundle;
use crate::game::playground::components::{WorldPosition, ReachDistance};


use super::components::{Footage, Available};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::board::components::Helper;
use super::interaction_allowed_for_footage;

pub fn spawn_footage (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,  
) {

    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 
    if let Some(bundle) = get_footage_bundle(&level) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
                transform: Transform::from_xyz(0.0, 0.0, 4.0).with_scale(Vec3::new(scale_reference,  scale_reference, 1.0)),
                material: materials.add(ColorMaterial::from(Color::FUCHSIA)),
                visibility: Visibility::Hidden,
                ..default()
            },
            bundle,
            Footage,
            AudioBundle {
                source: asset_server.load("sounds/rewind.ogg"),
                settings: PlaybackSettings { 
                    mode: PlaybackMode::Once, 
                    volume: Volume::Relative(VolumeLevel::new(0.5)), 
                    speed: 1.0, paused: true}
            },
        ));
    }
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
                    text.sections[0].value = "Interact to suppress the footage".to_string();
                }
            }
        }
    }
}

pub fn suppress_footage(
    keyboard_input: Res<Input<KeyCode>>, 
    keyboard: Res<KeyBoard>, 
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Footage>)>,
    footage_q: Query<(&WorldPosition, &ReachDistance), (With<Footage>, Without<Player>)>,
    sink_q: Query<&AudioSink, With<Footage>>, 
    mut available_q: Query<&mut Available, With<Footage>>,
    mut stealth_q: Query<&mut Stealth, With<Player>>,
) {
    if interaction_allowed_for_footage(player_q, footage_q) {
        if keyboard_input.just_pressed(keyboard.interact.unwrap()) {
            if let Ok(mut available) = available_q.get_single_mut() {
                if let Ok(mut stealth) = stealth_q.get_single_mut() {
                    available.0 = false;
                    if let Ok(sink) = sink_q.get_single() {
                        sink.play(); 
                    } 
                    match *stealth {
                        Stealth::Begineer => {
                            *stealth = Stealth::Engineer;
                        },
                        _ => {},
                    };
                }
            }
        }
    }
}