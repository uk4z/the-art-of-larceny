use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};
use bevy::utils::Duration;
use bevy::sprite::MaterialMesh2dBundle;

use std::f32::consts::PI;
use super::{components::*, obstacle_in_fov};
use super::{patrol_direction, chase_direction, search_direction};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::playground::player::DISTANCE_PER_SECOND;

use crate::components::Layer;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance, GameOver};
use crate::game::playground::scenery::components::{Bounds, Scenery};
use crate::game::playground::scenery::{get_scenery_scale_from_window, SCENERY_SIZE};


const FOV_RANGE: f32 = 220.0; 
const VISION_LENGTH: f32 = 150.0;

pub fn spawn_guard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    //spawn FOV
    for _ in 0..1 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(VISION_LENGTH, 3))).into(),
                transform: Transform::from_xyz(500.0, 50.0, 4.0),
                material: materials.add(ColorMaterial::from(Color::YELLOW)), 
                visibility: Visibility::Hidden,
                ..default()
            },
            FOVBundle {
                position: WorldPosition {
                    x: 500.0,
                    y: 50.0,
                },
                orientation: Orientation(Quat::IDENTITY),
            },
            FOV,
        ));
    }
   

    //spawn_guard
    for _ in 0..1 {
        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("guard/static.png"),
                transform: Transform::from_xyz(500.0, 50.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),       
            ..default()
            }, 
            GuardBundle { 
                position: WorldPosition {x: 717.0 , y: 849.0},
                orientation: Orientation(Quat::IDENTITY),
                pace: GuardPace::Walk,
                animation: AnimatedMotion {
                    walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                    run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                },
                reach: ReachDistance(20.0),
                patrol: Patrol {
                    positions: vec![WorldPosition {x: 717.0, y:849.0},
                        WorldPosition {x: 726.0, y:335.0},
                        WorldPosition {x: 382.0, y:345.0},
                        WorldPosition {x: 726.0, y:335.0},
                    ],
                    waitings: vec![
                        Waiting {
                            position: WorldPosition {x: 717.0, y:849.0},
                            time: Timer::from_seconds(2.0, TimerMode::Repeating),
                        },
                        Waiting {
                            position: WorldPosition {x: 382.0, y:345.0},
                            time: Timer::from_seconds(2.0, TimerMode::Repeating),
                        },
                    ],
                    position_index: 0, 
                    waiting_index: 0,
                },
                state: GuardState::Patrolling,
                chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
            },
            Guard,
        ));
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

pub fn despawn_fov(
    mut commands: Commands,
    entity_q: Query<Entity, With<FOV>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn alert_guard (
    mut guards_q: Query<(&mut GuardState, &WorldPosition, &Orientation, &mut GuardPace), With<Guard>>, 
    mut player_q: Query<(&WorldPosition, &mut Stealth), With<Player>>,
    bounds_q: Query<&Bounds, With<Scenery>>,
){
    if let Ok((player_pos, mut stealth)) = player_q.get_single_mut() {
        if let Ok(bounds) = bounds_q.get_single() {
            for (mut state, guard_pos, orientation, mut pace) in guards_q.iter_mut() {
                if !obstacle_in_fov(player_pos, guard_pos, bounds) {
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
    bounds_q: Query<&Bounds, With<Scenery>>,
){
    if let Ok(player_pos) = player_q.get_single() {
        if let Ok(bounds) = bounds_q.get_single() {
            for (mut state, guard_pos, orientation,) in guards_q.iter_mut() {
                let target_vector = Vec3::from(*player_pos)-Vec3::from(*guard_pos);
                let fov_vector = orientation.0.mul_vec3(Vec3::X);
                let angle = Quat::from_rotation_arc(target_vector, fov_vector).angle_between(Quat::IDENTITY);
                let distance = target_vector.length();
                match *state {
                    GuardState::Chasing => {
                        if !(angle < PI/4.0 && FOV_RANGE >= distance) || obstacle_in_fov(player_pos, guard_pos, bounds) {
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
    player_q: Query<(&WorldPosition, &ReachDistance), (With<Player>, Without<Guard>)>,
    mut guard_q: Query<(&WorldPosition, &ReachDistance), (With<Guard>, Without<Player>)>,
    mut game_over: EventWriter<GameOver>,
) {
    if let Ok((player_position, player_reach)) = player_q.get_single() {
        guard_q.for_each_mut(|(guard_position, guard_reach)| {
            let distance = Vec3::from(*player_position).distance(Vec3::from(*guard_position));
            if distance <= player_reach.0+guard_reach.0 {
                game_over.send(GameOver);
            }
        });
    }
}

pub fn update_fov(
    mut fov_q: Query<(&mut Orientation, &mut WorldPosition), (With<FOV>, Without<Guard>)>, 
    guard_q: Query<(&Orientation, &WorldPosition), (With<Guard>, Without<FOV>)>, 
) {
    let mut guard_items: Vec<(&Orientation, &WorldPosition)> = guard_q.iter().collect();

    fov_q.for_each_mut(|(mut orientation, mut position)| {
        if let Some((guard_orientation, guard_position)) = guard_items.pop() {
            orientation.0 = guard_orientation.0; 
            let origin = Vec3::from(*guard_position);
            let fov_position = origin + guard_orientation.0.mul_vec3(Vec3::X*VISION_LENGTH);
            *position = WorldPosition {x: fov_position.x, y:fov_position.y};
            orientation.0 = Quat::from_rotation_z(PI/2.0).mul_quat(guard_orientation.0);  
        }
    });
}



pub fn move_guard(
    time: Res<Time>,
    player_q: Query<&WorldPosition, (With<Player>, Without<Guard>)>,
    mut guard_q: Query<(&mut WorldPosition, &mut Patrol, &mut Orientation, &GuardPace, &mut GuardState), (With<Guard>, Without<Player>)>,
    bounds_q: Query<&Bounds, With<Scenery>>,
) { 
    guard_q.for_each_mut(|
        (mut position, mut patrol, mut orientation, 
            pace, mut state,
        )| { 
        let direction = match *state {
            GuardState::Patrolling => {
                patrol_direction(&mut *patrol, &*position)
            },
            GuardState::Chasing => {
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

        if !(patrol.is_waiting_position() && patrol.patrol_position_reached(*position)) {
            if let Ok(bounds) = bounds_q.get_single() {
                let (x, y) = ((position.x+translation.x) as usize, (SCENERY_SIZE.1-(position.y+translation.y)) as usize);
                            
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
    mut guard_q: Query<(&WorldPosition, &mut Patrol), With<Guard>>,
) {
    guard_q.for_each_mut(|(position, mut patrol)| {
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
                if let Some((pos, stack_orientation)) = stack.0.pop() {
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
            *texture = asset_server.load("guard/motion.png");
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
            }
        } else {
            *texture = asset_server.load("guard/static.png")
        }
    });
}
