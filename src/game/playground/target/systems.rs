use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::utils::Duration;

use crate::game::playground::components::{WorldPosition, ReachDistance};

use super::components::{Target, TargetBundle, UnlockTimer};
use super::is_target_unlock;
use crate::game::playground::player::components::Player;
use crate::game::board::components::{Helper, IntelMenu, Section, Vault, Instruction, ItemContent};
use super::interaction_allowed_for_target;

pub fn spawn_target (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::new(40.0))).into(),
            transform: Transform::from_xyz(1376.0, 640.0, 4.0),
            material: materials.add(ColorMaterial::from(Color::PURPLE)), 
            visibility: Visibility::Hidden,
            ..default()
        },
        TargetBundle {
            position: WorldPosition {
                x: 1266.0,
                y: 1123.0,
            },
            reach: ReachDistance(40.0),
            unlock_timer: UnlockTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)),
        },
        Target,
    ));
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
                    text.sections[0].value = "Press U to unlock the target".to_string();
                }
            }
        }
    }
}


pub fn load_target_unlock (
    mut intel_q: Query<(&mut Visibility, &mut Section), With<IntelMenu>>,
    mut vault_q: Query<&mut Visibility, (With<Vault>, Without<IntelMenu>)>,
    mut instruction_q: Query<&mut Visibility, (With<Instruction>, Without<Vault>, Without<IntelMenu>)>,
    mut item_q: Query<&mut Visibility, (With<ItemContent>, Without<Vault>, Without<IntelMenu>, Without<Instruction>)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Target>)>,
    target_q: Query<(&WorldPosition, &ReachDistance), (With<Target>, Without<Player>)>,
    mut timer_q: Query<&mut UnlockTimer, With<Target>>,
) {
    if interaction_allowed_for_target(player_q, target_q) {
        if let Ok((mut visibility, mut section)) = intel_q.get_single_mut() {
            if let Ok(mut vault_vis) = vault_q.get_single_mut() {
                if let Ok(mut instruction_vis) = instruction_q.get_single_mut() {
                    if let Ok(mut item_vis) = item_q.get_single_mut() {
                        if let Ok(mut timer) = timer_q.get_single_mut() {
                            if keyboard_input.just_pressed(KeyCode::U) {
                                timer.0.reset();
                                *vault_vis = Visibility::Inherited;
                                *visibility = Visibility::Visible;
                                *section = Section::Vault;
                                *instruction_vis = Visibility::Hidden; 
                                *item_vis = Visibility::Hidden; 
                            } else if keyboard_input.pressed(KeyCode::U) {
                                timer.0.tick(time.delta());
                            } else if keyboard_input.just_released(KeyCode::U) {
                                if !timer.0.finished() {
                                    timer.0.reset(); 
                                } 
                            }
                        }
                    }
                }
            }
        }
    }
}