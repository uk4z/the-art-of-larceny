use std::time::Duration;

use bevy::{prelude::Quat, time::{Timer, TimerMode}};
use std::f32::consts::PI; 

use crate::game::{playground::{guard::components::{GuardBundle, GuardPace, Patrol, Waiting, ChaseStack, GuardState, FOVBundle}, components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance}}, components::Level};

pub fn get_guard_bundle(level: &Level) -> Option<Vec<GuardBundle>> {
    match level {
        Level::Starting => {
            Some(
                vec![ 
                    GuardBundle { 
                        position: WorldPosition {x: 2626.0 , y: 512.0},
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![WorldPosition {x: 2626.0, y:512.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 2626.0, y:512.0},
                                    time: Timer::from_seconds(200.0, TimerMode::Repeating),
                                },
                            ],
                            position_index: 0, 
                            waiting_index: 0,
                        },
                        state: GuardState::Patrolling,
                        chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },
                
                
                    /* ---------------------------------------------------------------------- */
                    
                    GuardBundle { 
                        position: WorldPosition {x: 1409.0 , y: 1832.0},
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![WorldPosition {x: 1409.0 , y: 1832.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1409.0 , y: 1832.0},
                                    time: Timer::from_seconds(200.0, TimerMode::Repeating),
                                },
                            ],
                            position_index: 0, 
                            waiting_index: 0,
                        },
                        state: GuardState::Patrolling,
                        chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },

                    /* ---------------------------------------------------------------------- */
                
                    GuardBundle { 
                    position: WorldPosition {x: 2934.0 , y: 1832.0},
                    orientation: Orientation(Quat::from_rotation_z(PI/2.0)),
                    pace: GuardPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(20.0),
                    patrol: Patrol {
                        positions: vec![
                            WorldPosition {x: 2834.0 , y: 1832.0},
                            WorldPosition {x: 2731.0 , y: 2099.0},
                            WorldPosition {x: 2451.0 , y: 2099.0},
                            WorldPosition {x: 2311.0 , y: 1888.0},
                            WorldPosition {x: 2309.0 , y: 1888.0},
                            WorldPosition {x: 2451.0 , y: 2099.0},
                            WorldPosition {x: 2731.0 , y: 2099.0},
                        ],
                        waitings: vec![
                            Waiting {
                                position: WorldPosition {x: 2834.0 , y: 1832.0},
                                time: Timer::from_seconds(5.0, TimerMode::Repeating),
                            },
                            Waiting {
                                position: WorldPosition {x: 2309.0 , y: 1888.0},
                                time: Timer::from_seconds(5.0, TimerMode::Repeating),
                            },
                        ],
                        position_index: 0, 
                        waiting_index: 0,
                    },
                    state: GuardState::Patrolling,
                    chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },

                    /* ---------------------------------------------------------------------- */
                
                    GuardBundle { 
                        position: WorldPosition {x: 1430.0 , y: 2050.0},
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1430.0 , y: 2050.0},
                                WorldPosition {x: 1430.0 , y: 2148.0},
                                WorldPosition {x: 1882.0 , y: 2148.0},
                                WorldPosition {x: 1931.0 , y: 2043.0},
                                WorldPosition {x: 1882.0 , y: 2148.0},
                                WorldPosition {x: 1430.0 , y: 2148.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1430.0 , y: 2050.0},
                                    time: Timer::from_seconds(5.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 1931.0 , y: 2043.0},
                                    time: Timer::from_seconds(5.0, TimerMode::Repeating),
                                },
                            ],
                            position_index: 0, 
                            waiting_index: 0,
                        },
                        state: GuardState::Patrolling,
                        chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },

                    /* ---------------------------------------------------------------------- */
            
                    GuardBundle { 
                        position: WorldPosition {x: 2487.0 , y: 1299.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 2487.0 , y: 1299.0},
                                WorldPosition {x: 2201.0 , y: 1259.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 2487.0 , y: 1299.0},
                                    time: Timer::from_seconds(5.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition{x: 2201.0 , y: 1259.0},
                                    time: Timer::from_seconds(1.0, TimerMode::Repeating),
                                },
                            ],
                            position_index: 0, 
                            waiting_index: 0,
                        },
                        state: GuardState::Patrolling,
                        chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },  

                    /* ---------------------------------------------------------------------- */
            
                    GuardBundle { 
                        position: WorldPosition {x: 1525.0 , y: 1624.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1525.0 , y: 1624.0},
                                WorldPosition {x: 2090.0 , y: 1624.0},
                                WorldPosition {x: 2090.0 , y: 1270.0},
                                WorldPosition {x: 1525.0 , y: 1270.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 0.0 , y: 0.0}, // basically means there is no waiting position
                                    time: Timer::from_seconds(1.0, TimerMode::Repeating),
                                },
                            ],
                            position_index: 0, 
                            waiting_index: 0,
                        },
                        state: GuardState::Patrolling,
                        chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },  

                    /* ---------------------------------------------------------------------- */
            
                    GuardBundle { 
                        position: WorldPosition {x: 1508.0 , y: 682.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1508.0 , y: 682.0},
                                WorldPosition {x: 1936.0 , y: 682.0},
                                WorldPosition {x: 1936.0 , y: 180.0},
                                WorldPosition {x: 1508.0 , y: 180.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 0.0 , y: 0.0}, // basically means there is no waiting position
                                    time: Timer::from_seconds(1.0, TimerMode::Repeating),
                                },
                            ],
                            position_index: 0, 
                            waiting_index: 0,
                        },
                        state: GuardState::Patrolling,
                        chase_stack: ChaseStack(Vec::<(WorldPosition, Orientation)>::new()),
                    },  

                    /* ---------------------------------------------------------------------- */
            
                    GuardBundle { 
                        position: WorldPosition {x: 1261.0 , y: 1228.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1261.0 , y: 1228.0},
                                WorldPosition {x: 1268.0 , y: 1717.0},
                                WorldPosition {x: 715.0 , y: 1717.0},
                                WorldPosition {x: 715.0 , y: 1450.0},
                                WorldPosition {x: 910.0 , y: 1450.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1261.0 , y: 1228.0},
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
    }
}


pub fn get_fov_bundle(level: &Level) -> Option<Vec<FOVBundle>> {
    match level {
        Level::Starting => {
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
    }
}