use bevy::audio::{PlaybackMode, VolumeLevel, Volume};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use crate::get_scale_reference;
use crate::game::components::{Level, KeyBoard};
use crate::game::bundle::target::get_target_bundle;
use crate::game::playground::components::{WorldPosition, ReachDistance};

use super::components::{Target, UnlockTimer};
use super::is_target_unlock;
use crate::game::playground::player::components::Player;
use crate::game::board::components::Helper;
use super::interaction_allowed_for_target;

pub fn spawn_target (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,     
    window_q: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,  
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Some(bundle) = get_target_bundle(&level) {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
                transform: Transform::from_xyz(0.0, 0.0, 4.0)
                    .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),
                material: materials.add(ColorMaterial::from(Color::PURPLE)), 
                visibility: Visibility::Hidden,
                ..default()
            },
            bundle,
            Target,
            AudioBundle {
                source: asset_server.load("sounds/hacking.ogg"),
                settings: PlaybackSettings { 
                    mode: PlaybackMode::Loop, 
                    volume: Volume::Relative(VolumeLevel::new(0.5)), 
                    speed: 1.0, paused: true }
            },
        ));
    }
    
}

pub fn despawn_target(
    mut commands: Commands,
    entity_q: Query<Entity, With<Target>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn signal_target_zone (
    mut help_q: Query<&mut Text, With<Helper>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Target>)>,
    target_q: Query<(&WorldPosition, &ReachDistance), (With<Target>, Without<Player>)>,
    timer_q: Query<&UnlockTimer, With<Target>>, 
) {
    if let Ok(mut text) = help_q.get_single_mut() {
        if interaction_allowed_for_target(player_q, target_q) {
            if let Ok(timer) = timer_q.get_single() {
                if !is_target_unlock(timer) {
                    text.sections[0].value = "Interact to unlock the target".to_string();
                }
            }
        }
    }
}


pub fn load_target_unlock (
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    keyboard: Res<KeyBoard>, 
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Target>)>,
    target_q: Query<(&WorldPosition, &ReachDistance), (With<Target>, Without<Player>)>,
    mut timer_q: Query<&mut UnlockTimer, With<Target>>,
) {
    if interaction_allowed_for_target(player_q, target_q) {
        if let Ok(mut timer) = timer_q.get_single_mut() {
            if keyboard_input.just_pressed(keyboard.interact.unwrap()) && !timer.0.finished() { 
                timer.0.reset();
            } else if keyboard_input.pressed(keyboard.interact.unwrap()) {
                timer.0.tick(time.delta());
            } 
        }
    }
}

pub fn animate_sound(
    audio_q: Query<&AudioSink, With<Target>>,
    timer_q: Query<&UnlockTimer, With<Target>>,
    keyboard_input: Res<Input<KeyCode>>,
    keyboard: Res<KeyBoard>, 
) {
    if let Ok(timer) = timer_q.get_single() {
        if let Ok(sink) = audio_q.get_single() {
            if timer.0.finished() {
                sink.stop(); 
            }
            if timer.0.percent() > 0.0  {
                if keyboard_input.pressed(keyboard.interact.unwrap()) {
                    sink.play();
                }
                if keyboard_input.just_released(keyboard.interact.unwrap()) {
                    sink.pause(); 
                }
            }
            
        }
    }

}