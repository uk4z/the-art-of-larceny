use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};
use bevy::utils::Duration;


use super::components::{GuardBundle, Guard, Waiting, Patrol};
use super::get_guard_direction;
use crate::game::playground::player::components::PlayerPace;
use crate::game::playground::player::DISTANCE_PER_SECOND;

use crate::components::Layer;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance};
use crate::game::playground::scenery::get_scenery_scale_from_window;

pub fn spawn_guard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("guard/static.png"),
            transform: Transform::from_xyz(500.0, 50.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),       
        ..default()
        }, 
        GuardBundle { 
            position: WorldPosition {x: 500.0, y: 50.0,},
            orientation: Orientation(Quat::IDENTITY),
            pace: PlayerPace::Walk,
            animation: AnimatedMotion {
                walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
            },
            reach: ReachDistance(20.0),
            patrol: Patrol {
                positions: vec![WorldPosition {x: 50.0, y: 50.0},  
                        WorldPosition {x: 340.0, y: 160.0}, 
                        WorldPosition {x: 890.0, y: 1000.0}],
                waitings: vec![
                    Waiting {
                        position: WorldPosition { x: 340.0, y: 160.0 },
                        time: Timer::from_seconds(2.0, TimerMode::Repeating),
                    },
                ],
                position_index: 0, 
                waiting_index: 0,
            }
        },
        Guard,
    ));
}

pub fn move_guard(
    time: Res<Time>,
    mut guard_q: Query<(&mut WorldPosition, &mut Patrol, &mut Orientation, &PlayerPace), With<Guard>>,
) { 
    guard_q.for_each_mut(|(mut position, mut patrol, mut orientation, pace)| { 
        let direction = get_guard_direction(&mut *patrol, &*position);

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

        if !(patrol.is_waiting_position() && patrol.patrol_position_reached(*position)) {
            position.x += translation.x; 
            position.y += translation.y;
        }

        //update orientation
        if direction.length() > 0.0 && !(patrol.is_waiting_position() && patrol.patrol_position_reached(*position)) {
            orientation.0 = Quat::from_rotation_arc(Vec3::X, direction);
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

/* 
pub fn set_guard_pace(
    guard_q: Query<&mut PlayerPace, With<Guard>>,
) {
} */

pub fn guard_motion_handler(
    mut guard_q: Query<(&mut Handle<Image> ,&mut AnimatedMotion, &mut Transform, &PlayerPace, &mut Patrol, &WorldPosition), With<Guard>>,
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
            *texture = asset_server.load("guard/static.png")
        }
    });
}
