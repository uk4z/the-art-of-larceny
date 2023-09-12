use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};

use super::components::*;
use super::{get_player_direction, DISTANCE_PER_SECOND};

use crate::components::Layer;
use crate::game::board::components::IntelMenu;
use crate::game::components::Level;
use crate::game::bundle::player::get_player_bundle;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion};
use crate::game::playground::scenery::components::{Bounds, Scenery};
use crate::game::playground::scenery::{get_scenery_scale_from_window, SCENERY_SIZE};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    level: Res<Level>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    if let Some(bundle) = get_player_bundle(&level) {
        commands.spawn(
            (SpriteBundle{
                texture: asset_server.load("player/static.png"),
                transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),       
            ..default()
            }, 
            bundle,
            Player, 
        ));
    }
    
}

pub fn despawn_player(
    mut commands: Commands,
    entity_q: Query<Entity, With<Player>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut WorldPosition, &mut Orientation, &PlayerPace), With<Player>>,
    intel_q: Query<&Visibility, With<IntelMenu>>, 
    bounds_q: Query<&Bounds, With<Scenery>>,

) {
    if let Ok(visibility) = intel_q.get_single() {
        match *visibility {
            Visibility::Hidden => {
                if let Ok((mut position, mut orientation, pace)) = player_q.get_single_mut() { 
                    let direction = get_player_direction(keyboard_input);
            
                    //update position
                    let speed = match pace {
                        PlayerPace::Run => {
                            <PlayerPace as Into<f32>>::into(PlayerPace::Run)*DISTANCE_PER_SECOND*time.delta_seconds()
                        },
                        PlayerPace::Walk => {
                            <PlayerPace as Into<f32>>::into(PlayerPace::Walk)*DISTANCE_PER_SECOND*time.delta_seconds()
                        },
                    };
                    let translation: Vec3 = direction*speed;

                    if let Ok(bounds) = bounds_q.get_single() {
                        let (x, y) = ((position.x+translation.x) as usize, (SCENERY_SIZE.1-(position.y+translation.y)) as usize);
                        
                        let mut move_player = true; 
                        if !bounds.0.is_empty() {
                            for width in 0..20 {
                                for height in 0..20 {
                                    move_player = move_player 
                                                && bounds.0[y+height][x+width] == 0 
                                                && bounds.0[y+height][x-width] == 0
                                                && bounds.0[y-height][x-width] == 0 
                                                && bounds.0[y-height][x-width] == 0 ;
                                }
                            }
                        }

                        if move_player {
                            position.x += translation.x; 
                            position.y += translation.y;
                        }
            
                    } 
            
                    //update orientation
                    if direction.length() > 0.0 {
                        orientation.0 = Quat::from_rotation_arc(Vec3::X, direction);
                    }
                    
                };
            },
            _ => {},
        }
    }
    
}


pub fn set_player_pace(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<&mut PlayerPace, With<Player>>,
) {
    if let Ok(mut player_pace) = player_q.get_single_mut() {
        let pace = player_pace.as_mut();
        if keyboard_input.pressed(KeyCode::LShift) {
            *pace = PlayerPace::Run;
        } else {
            *pace = PlayerPace::Walk;
        }
    }
}

pub fn motion_handler(
    intel_q: Query<&Visibility, With<IntelMenu>>, 
    mut player_q: Query<(&mut Handle<Image> ,&mut AnimatedMotion, &mut Transform, &PlayerPace), With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if let Ok(visibility) = intel_q.get_single() {
        match *visibility {
            Visibility::Hidden => {
            
                if let Ok((mut texture, mut animated, mut transform, pace)) 
                        = player_q.get_single_mut() {
            
                    if keyboard_input.any_pressed([KeyCode::Z, KeyCode::Q, KeyCode::S, KeyCode::D]) {
                        *texture = asset_server.load("player/motion.png");
                        match pace {
                            PlayerPace::Run => {
                                animated.run_timer.tick(time.delta());
                                if animated.run_timer.finished() {
                                    transform.scale.y = -transform.scale.y;
                                }
                            },
                            PlayerPace::Walk => {
                                animated.walk_timer.tick(time.delta());
                                if animated.walk_timer.finished() {
                                    transform.scale.y = -transform.scale.y;
                                }
                            },
                        }
                    } else {
                        *texture = asset_server.load("player/static.png")
                    }
                }
            }, 
            _ => {}
        }
    }
    
}
