use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::audio::{Volume, PlaybackMode, VolumeLevel};


use std::f32::consts::PI;
use super::{components::*, obstacle_in_fov};
use super::{patrol_direction, chase_direction, search_direction};
use crate::get_scale_reference;
use crate::game::components::Level;
use crate::game::bundle::guard::get_guard_bundle;
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::playground::player::DISTANCE_PER_SECOND;

use crate::components::Layer;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance, GameOver};
use crate::game::playground::scenery::components::{Bounds, Scenery, ScenerySize};


const FOV_RANGE: f32 = 250.0; 
const HEAR_LIMIT: f32 = 400.0; 

pub fn spawn_guard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
    window_q: Query<&Window, With<PrimaryWindow>>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    let scale = 1.2; 
    //spawn_guard
    if let Some(guards) = get_guard_bundle(&level) {
        for bundle in guards {
            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load("guard/static.png"),
                    transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into())
                        .with_scale(Vec3::new(scale*scale_reference, scale*scale_reference, 1.0)),       
                ..default()
                }, 
                bundle,
                Guard,
                AudioBundle {
                    source: asset_server.load("sounds/footstep.ogg"),
                    settings: PlaybackSettings {
                        mode: PlaybackMode::Loop, 
                        volume: Volume::Relative(VolumeLevel::new(0.3)), 
                        speed: 1.0, paused: true}
                },
            ));
        }
    }
    
    
}

pub fn despawn_guard(
    mut commands: Commands,
    entity_q: Query<Entity, With<Guard>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn alert_guard (
    mut guards_q: Query<(&mut GuardState, &WorldPosition, &Orientation, &mut GuardPace), With<Guard>>, 
    mut player_q: Query<(&WorldPosition, &mut Stealth), With<Player>>,
    bounds_q: Query<(&Bounds, &ScenerySize), With<Scenery>>,
    asset_server: Res<AssetServer>, 
    mut commands: Commands,
){
    if let Ok((player_pos, mut stealth)) = player_q.get_single_mut() {
        if let Ok((bounds, size)) = bounds_q.get_single() {
            for (mut state, guard_pos, orientation, mut pace) in guards_q.iter_mut() {
                if !obstacle_in_fov(player_pos, guard_pos, bounds, size) {
                    let target_vector = Vec3::from(*player_pos)-Vec3::from(*guard_pos);
                    let fov_vector = orientation.0.mul_vec3(Vec3::X);
                    let angle = Quat::from_rotation_arc(target_vector, fov_vector).angle_between(Quat::IDENTITY);
                    let distance = target_vector.length();
                    match *state {
                        GuardState::Chasing => {},
                        _ => {
                            if angle < PI/4.0 && FOV_RANGE >= distance {
                                *state = GuardState::Chasing;
                                *pace = GuardPace::Run;
                                *stealth = Stealth::None;

                                commands.spawn(
                                    AudioBundle {
                                        source: asset_server.load("sounds/scream.ogg"),
                                        settings: PlaybackSettings {
                                            mode: PlaybackMode::Despawn, 
                                            volume: Volume::Relative(VolumeLevel::new(1.0)), 
                                            speed: 1.0, paused: false}
                                    }
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn disalert_guard (
    mut guards_q: Query<(&mut GuardState, &WorldPosition, &Orientation), With<Guard>>, 
    player_q: Query<&WorldPosition, With<Player>>,
    bounds_q: Query<(&Bounds, &ScenerySize), With<Scenery>>,
){
    if let Ok(player_pos) = player_q.get_single() {
        if let Ok((bounds, size)) = bounds_q.get_single() {
            for (mut state, guard_pos, orientation,) in guards_q.iter_mut() {
                let target_vector = Vec3::from(*player_pos)-Vec3::from(*guard_pos);
                let fov_vector = orientation.0.mul_vec3(Vec3::X);
                let angle = Quat::from_rotation_arc(target_vector, fov_vector).angle_between(Quat::IDENTITY);
                let distance = target_vector.length();
                match *state {
                    GuardState::Chasing => {

                        if !(angle < PI/4.0 && FOV_RANGE >= distance) || obstacle_in_fov(player_pos, guard_pos, bounds, size) {
                            *state = GuardState::Searching(*player_pos);

                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

pub fn catch_player (
    player_q: Query<(&AudioSink, &WorldPosition, &ReachDistance), (With<Player>, Without<Guard>)>,
    mut guard_q: Query<(&WorldPosition, &ReachDistance, &GuardState), (With<Guard>, Without<Player>)>,
    mut game_over: EventWriter<GameOver>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    if let Ok((sink, player_position, player_reach)) = player_q.get_single() {
        guard_q.for_each_mut(|(guard_position, guard_reach, state)| {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*guard_position));
            if distance <= player_reach.0+guard_reach.0 {
                match *state {
                    GuardState::Chasing => {
                        game_over.send(GameOver);
                        sink.pause();
                        commands.spawn(
                            AudioBundle {
                                source: asset_server.load("sounds/guard_punch.ogg"),
                                settings: PlaybackSettings { 
                                    mode: PlaybackMode::Despawn, 
                                    volume: Volume::Relative(VolumeLevel::new(0.5)), 
                                    speed: 1.0, paused: false}
                            }
                        );
                    }, 
                    _ => {}
                }
            }
        });
    }
}

pub fn move_guard(
    time: Res<Time>,
    player_q: Query<&WorldPosition, (With<Player>, Without<Guard>)>,
    mut guard_q: Query<(&mut WorldPosition, &mut Patrol, &mut Orientation, &GuardPace, &mut GuardState), (With<Guard>, Without<Player>)>,
    bounds_q: Query<(&Bounds, &ScenerySize), With<Scenery>>,
) { 
    guard_q.for_each_mut(|
        (mut position, mut patrol, mut orientation, 
            pace, mut state,
        )| { 

        let mut is_chasing = false;
        let direction = match *state {
            GuardState::Patrolling => {
                patrol_direction(&mut *patrol, &*position)
            },
            GuardState::Chasing => {
                is_chasing = true; 
                if let Ok(player_pos) = player_q.get_single() {
                    chase_direction(player_pos, &*position)
                } else {
                    Vec3::ZERO
                }
            }, 
            GuardState::Searching(target) => {
                search_direction(&target, &*position)
            },
            GuardState::Returning => {
                Vec3::ZERO
            },
            GuardState::Waiting(_) => {
                Vec3::ZERO
            }
        };
  
        //update position
        let speed = match pace {
            GuardPace::Run => {
                <GuardPace as Into<f32>>::into(GuardPace::Run)*DISTANCE_PER_SECOND*time.delta_seconds()
            },
            GuardPace::Walk => {
                <GuardPace as Into<f32>>::into(GuardPace::Walk)*DISTANCE_PER_SECOND*time.delta_seconds()
            },
        };
        let translation: Vec3 = direction*speed;

        if (!(patrol.is_waiting_position() && patrol.patrol_position_reached(*position))) || is_chasing {
            if let Ok((bounds, size)) = bounds_q.get_single() {
                let (x, y) = ((position.x+translation.x) as usize, (size.height-(position.y+translation.y)) as usize);
                            
                let mut move_guard = true; 
                if !bounds.0.is_empty() {
                    for width in 0..20 {
                        for height in 0..20 {
                            move_guard = move_guard 
                                        && bounds.0[y+height][x+width] == 0 
                                        && bounds.0[y+height][x-width] == 0
                                        && bounds.0[y-height][x-width] == 0 
                                        && bounds.0[y-height][x-width] == 0 ;
                        }
                    }
                }
                
                if move_guard {
                    position.x += translation.x; 
                    position.y += translation.y;
                } else {
                    match *state {
                        GuardState::Searching(_) => {
                            *state = GuardState::Waiting(Timer::from_seconds(2.0, TimerMode::Once)); 
                        }
                        _ => {}
                    }
                }
            }
        }

        //update orientation
        if direction.length() > 0.0 && !(patrol.is_waiting_position() && patrol.patrol_position_reached(*position)) {
            orientation.0 = Quat::from_rotation_arc(Vec3::X, direction);
        }
        
    });
}

pub fn update_waiting_timer(
    mut guard_q: Query<&mut GuardState, With<Guard>>,
    time: Res<Time>
) {
    guard_q.for_each_mut(|mut state|{
        match state.clone() {
            GuardState::Waiting(mut timer) => {
                if timer.finished() {
                    *state = GuardState::Returning;
                } else {
                    timer.tick(time.delta());
                    *state = GuardState::Waiting(timer);
                }
            },
            _ => {}
        }
    });
}

pub fn update_patrols_positions(
    past_time: Res<Time>,
    mut guard_q: Query<(&WorldPosition, &mut Patrol, &GuardState), With<Guard>>,
) {
    guard_q.for_each_mut(|(position, mut patrol, state)| {
        match *state {
            GuardState::Patrolling => {
                if patrol.patrol_position_reached(*position) {
                    if patrol.is_waiting_position() {
                        let waiting = patrol.get_waiting();
                        waiting.time.tick(past_time.delta());
                        if waiting.time.finished() {
                            patrol.next_waiting();
                            patrol.next_position();
                        }
                    } else {
                        patrol.next_position();
                    }
                }
            }
            _ => {}
        }
    });
}

pub fn update_chase_stack (
    mut guard_q: Query<(&mut ChaseStack, &mut WorldPosition, &mut GuardState, &mut Orientation, &mut GuardPace), With<Guard>>,
) {
    guard_q.for_each_mut(|(
        mut stack, mut position, mut state, 
        mut orientation, mut pace
    )| {
        match *state {
            GuardState::Chasing => {
                stack.0.push((*position, *orientation));
            },
            GuardState::Searching(target) => {
                let distance = (Vec3::from(target) - Vec3::from(*position)).length();
                if distance < POSITION_REACH {
                    *state = GuardState::Waiting(Timer::from_seconds(2.0, TimerMode::Once));
                } else {
                    stack.0.push((*position, *orientation));
                }
            },
            GuardState::Returning => {
                if stack.0.len() == 1 {
                    let (pos, stack_orientation) = stack.0.pop().unwrap() ;
                    *position = pos;
                    orientation.0 =  stack_orientation.0;
                } else if let Some((pos, stack_orientation)) = stack.0.pop() {
                    *position = pos;
                    orientation.0 =  stack_orientation.0.mul_quat(Quat::from_rotation_z(PI));  
                } else {
                    *state = GuardState::Patrolling;
                    *pace = GuardPace::Walk;
                };
            },
            _ => {}
            
        }
    });
}

pub fn guard_motion_handler(
    mut guard_q: Query<(&mut Handle<Image> ,&mut AnimatedMotion, &mut Transform, &GuardPace, &mut Patrol, &WorldPosition), With<Guard>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    guard_q.for_each_mut(|(
        mut texture, 
        mut animated, 
        mut transform, 
        pace, 
        mut patrol,
        position,
    )| { 
        if !(patrol.is_waiting_position() && patrol.patrol_position_reached(*position)) {
            match pace {
                GuardPace::Run => {
                    animated.run_timer.tick(time.delta());
                    if animated.run_timer.finished() {
                        transform.scale.y = -transform.scale.y;
                    }
                },
                GuardPace::Walk => {
                    animated.walk_timer.tick(time.delta());
                    if animated.walk_timer.finished() {
                        transform.scale.y = -transform.scale.y;
                    }
                },
            };
            *texture = asset_server.load("guard/motion.png");
        } else {
            *texture = asset_server.load("guard/static.png"); 
        }
    });
}

pub fn guard_sound_handler(
    mut guard_q: Query<(&GuardState, &GuardPace, &mut Patrol, &WorldPosition, &AudioSink), With<Guard>>,
) {
    guard_q.for_each_mut(|( 
        state, 
        pace, 
        mut patrol,
        position,
        sink,
    )| { 
        match *state {
            GuardState::Waiting(_) => {
                sink.pause()
            } 
            _ => {sink.play()}
        }

        if !(patrol.is_waiting_position() && patrol.patrol_position_reached(*position)) {
            match pace {
                GuardPace::Run => {
                    sink.set_speed(2.8); 
                    sink.set_volume(0.5); 
                },
                GuardPace::Walk => {
                    sink.set_speed(1.2); 
                    sink.set_volume(0.3); 
                },
            };
            sink.play();
        } else {
            sink.pause()
        }
    });
}


pub fn handle_sound_distance(
    guard_q: Query<(&AudioSink, &WorldPosition), With<Guard>>,
    player_q: Query<&WorldPosition, (With<Player>, Without<Guard>)>,
) {
    if let Ok(player_pos) = player_q.get_single() {
        guard_q.for_each(|(sink, guard_pos)| {
            let distance = Vec3::from(*guard_pos).distance(Vec3::from(*player_pos)); 
            
            if distance > HEAR_LIMIT {
                sink.pause(); 
            }
        }); 
    }
}


pub fn bounds_guard(
    mut guard_q: Query<(&Orientation, &mut WorldPosition, &GuardState), With<Guard>>,
    bounds_q: Query<(&Bounds, &ScenerySize), With<Scenery>>,
) {
    if let Ok((bounds, size)) = bounds_q.get_single() {
        guard_q.for_each_mut(|(orientation, mut position, state)| {
            match *state {
                GuardState::Chasing => {
                    let right = orientation.0.mul_quat(Quat::from_rotation_z(-PI/2.0)).normalize().mul_vec3(Vec3::X);
                    let left = orientation.0.mul_quat(Quat::from_rotation_z(PI/2.0)).normalize().mul_vec3(Vec3::X);
        
                    for i in 1..4 {
                        let rr = orientation.0.mul_quat(Quat::from_rotation_z(-(i as f32)*PI/6.0)).normalize().mul_vec3(Vec3::X)*50.0;  
                        if obstacle_in_fov(&WorldPosition { x: position.x + rr.x, y: position.y + rr.y }, &position, bounds, size) {
                            let projection = rr.project_onto(right).normalize();
                            position.x += -projection.x; 
                            position.y +=  -projection.y; 
                        }
        
                        let ll = orientation.0.mul_quat(Quat::from_rotation_z((i as f32)*PI/6.0)).normalize().mul_vec3(Vec3::X)*50.0; 
        
                        if obstacle_in_fov(&WorldPosition { x: position.x + ll.x, y: position.y + ll.y }, &position, bounds, size) {
                            let projection = ll.project_onto(left).normalize();
                            position.x += -projection.x; 
                            position.y +=  -projection.y; 
                        }
                        
                    }
                },
                _ => {}
            }
        
        });
    }
}

pub fn avoid_guard_collisions(
    mut guard_q: Query<&mut WorldPosition, With<Guard>>, 
) {
    let positions: Vec<Mut<'_, WorldPosition>> = guard_q.iter_mut().collect();
    let length = positions.len();
    let mut mirror = Vec::new();
    let mut mutable_mirror = Vec::new();

    for position in positions {
        mirror.push(position.clone()); 
        mutable_mirror.push(position);
    }  
    
    for ref_index in 0..length {
        let ref_pos = mirror.get(ref_index).unwrap();
        for i in (ref_index+1)..length {
            let pos = mutable_mirror.get_mut(i).unwrap(); 

            let direction = Vec3::from(**pos) - Vec3::from(*ref_pos);

            let ratio = 80.0 /*hitbox length*/ / direction.length();  
            if ratio > 1.0  {
                let new_direction = direction + direction.normalize(); 
                **pos = WorldPosition {x: ref_pos.x+new_direction.x, y: ref_pos.y+new_direction.y};
            }
        }
    }
    
}

pub fn stop_footsteps(
    guard_q: Query<&AudioSink, With<Guard>>,
) {
    guard_q.for_each(|sink| {
        sink.stop();
    }); 
}

pub fn pause_footsteps(
    guard_q: Query<&AudioSink, With<Guard>>,
) {
    guard_q.for_each(|sink| {
        sink.pause();
    }); 
}