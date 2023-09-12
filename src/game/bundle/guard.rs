use std::time::Duration;

use bevy::{prelude::Quat, time::{Timer, TimerMode}};

use crate::game::playground::{guard::components::{GuardBundle, GuardPace, Patrol, Waiting, ChaseStack, GuardState, FOVBundle}, components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance}};

pub fn get_guard_bundle(level: &str) -> Option<Vec<GuardBundle>> {
    match level {
        "starting" => {
            Some(
                vec![ 
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
                ]
            )
        },
        _ => {None}
    }
}


pub fn get_fov_bundle(level: &str) -> Option<Vec<FOVBundle>> {
    match level {
        "starting" => {
            Some(
                vec![
                    FOVBundle {
                        position: WorldPosition {
                            x: 500.0,
                            y: 50.0,
                        },
                        orientation: Orientation(Quat::IDENTITY),
                    },
                ]
            )
        },
        _ => {None}
    }
}