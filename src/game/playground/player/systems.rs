use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};

use super::components::*;
use super::{get_player_direction, DISTANCE_PER_SECOND};

use crate::get_scale_reference;
use crate::components::Layer;
use crate::game::components::Level;
use crate::game::bundle::player::get_player_bundle;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance};
use crate::game::playground::guard::components::{GuardState, Guard};
use crate::game::playground::scenery::components::{Bounds, Scenery};
use crate::game::playground::scenery::SCENERY_SIZE;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    level: Res<Level>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Some(bundle) = get_player_bundle(&level) {
        commands.spawn(
            (SpriteBundle{
                texture: asset_server.load("player/static.png"),
                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, Layer::Interactable.into())
                        .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),       
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
    bounds_q: Query<&Bounds, With<Scenery>>,

) {
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
    mut player_q: Query<(&mut Handle<Image> ,&mut AnimatedMotion, &mut Transform, &PlayerPace), With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>
) {
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
}

pub fn neutralise_guard (
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Guard>)>,
    mut guard_q: Query<(Entity, &WorldPosition, &Orientation, &ReachDistance, &GuardState), (With<Guard>, Without<Player>)>,
    window_q: Query<&Window, With<PrimaryWindow>>, 
) {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        guard_q.for_each_mut(|(entity, guard_position, orientation, guard_reach, guard_state)| {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*guard_position));
            if distance <= player_reach.0+guard_reach.0 {
                match *guard_state {
                    GuardState::Chasing => {},
                    _ => {
                        commands.entity(entity).despawn(); 

                        let scale = 1.2;
                    
                        let window = window_q.get_single().unwrap(); 
                        let scale_reference = get_scale_reference(&window.width(), &window.height()); 

                        commands.spawn(
                            (SpriteBundle{
                                texture: asset_server.load("guard/dead.png"), 
                                transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into())
                                    .with_scale(Vec3::new(scale*scale_reference, scale*scale_reference, 1.0))
                                    .with_rotation(orientation.0),       
                            ..default()
                            }, 
                            WorldPosition {x: guard_position.x, y: guard_position.y}, 
                            Corpse, 
                        ));
                    }
                }
            }
        });
    }
}


pub fn despawn_corpse(
    mut commands: Commands, 
    entity_q: Query<Entity, With<Corpse>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}