use std::time::Duration;

use bevy::{prelude::Quat, time::{Timer, TimerMode}};
use std::f32::consts::PI; 

use crate::game::{playground::{guard::components::{GuardBundle, GuardPace, Patrol, Waiting, ChaseStack, GuardState}, components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance}}, components::Level};

pub fn get_guard_bundle(level: &Level) -> Option<Vec<GuardBundle>> {
    match level {
        Level::Tutorial => {
            Some(
                vec![ 
                    GuardBundle { 
                        position: WorldPosition {x: 867.0 , y: 469.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![WorldPosition {x: 867.0, y:469.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 867.0, y: 469.0},
                                    time: Timer::from_seconds(10000.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 1034.0 , y: 1672.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![WorldPosition {x: 1034.0, y:1672.0},
                                WorldPosition {x: 1503.0, y:1668.0},
                                WorldPosition {x: 1503.0, y:1931.0},
                                WorldPosition {x: 1034.0, y:1931.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 0.0, y: 0.0},
                                    time: Timer::from_seconds(1000.0, TimerMode::Repeating),
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
        Level::Factory => {
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
        Level::Warehouse => {
            Some(
                vec![ 
                    GuardBundle { 
                        position: WorldPosition {x: 689.0 , y: 495.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 689.0, y:496.0},
                                WorldPosition {x: 747.0, y:642.0},
                                WorldPosition {x: 1010.0, y:650.0},
                                WorldPosition {x: 872.0, y:916.0},
                                WorldPosition {x: 823.0, y:683.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 689.0, y: 496.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 872.0, y: 916.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 549.0 , y: 1642.0},
                        orientation: Orientation(Quat::from_rotation_z(10.0*PI/6.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 549.0, y:1642.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 549.0, y: 1642.0},
                                    time: Timer::from_seconds(10000.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 1239.0 , y: 1900.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1239.0, y:1900.0},
                                WorldPosition {x: 523.0, y:1947.0},
                                WorldPosition {x: 321.0, y:1850.0},
                                WorldPosition {x: 354.0, y:1502.0},
                                WorldPosition {x: 710.0, y:1383.0},
                                WorldPosition {x: 681.0, y:1241.0},
                                WorldPosition {x: 827.0, y:1742.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1239.0, y:1900.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 681.0, y: 1241.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 1537.0 , y: 1960.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1537.0, y:1960.0},
                                WorldPosition {x: 2206.0, y:1945.0},
                                WorldPosition {x: 2507.0, y:1739.0},
                                WorldPosition {x: 2206.0, y:1945.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1537.0, y:1960.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 2507.0, y:1739.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 2719.0 , y: 1320.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 2719.0 , y: 1320.0},
                                WorldPosition {x: 2517.0, y: 1246.0},
                                WorldPosition {x: 3003.0, y: 1246.0},
                                WorldPosition {x: 2982.0, y: 970.0},
                                WorldPosition {x: 2557.0, y: 979.0},
                                WorldPosition {x: 2982.0, y: 970.0},
                                WorldPosition {x: 3217.0, y: 1300.0},
                                WorldPosition {x: 3228.0, y: 1674.0},
                                WorldPosition {x: 2857.0, y: 1666.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 0.0, y:0.0},
                                    time: Timer::from_seconds(1000.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 2298.0 , y: 137.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 2298.0 , y: 137.0},
                                WorldPosition {x: 2734.0, y: 111.0},
                                WorldPosition {x: 2762.0, y: 292.0},
                                WorldPosition {x: 2512.0, y: 393.0},
                                WorldPosition {x: 2273.0, y: 370.0},
                                WorldPosition {x: 1914.0, y: 144.0},
                                WorldPosition {x: 1612.0, y: 163.0},
                                WorldPosition {x: 1550.0, y: 553.0},
                                WorldPosition {x: 1954.0, y: 545.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 2762.0, y: 292.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 1764.0 , y: 832.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1764.0 , y: 832.0},
                                WorldPosition {x: 2128.0, y: 838.0},
                                WorldPosition {x: 2122.0, y: 1407.0},
                                WorldPosition {x: 2122.0, y: 1207.0},
                                WorldPosition {x: 1919.0, y: 1230.0},
                                WorldPosition {x: 2122.0, y: 1207.0},
                                WorldPosition {x: 2128.0, y: 838.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1764.0, y: 832.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 2122.0, y: 1407.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 1919.0, y: 1230.0},
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
        Level::MillerHouse => {
            Some(
                vec![ 
                    GuardBundle { 
                        position: WorldPosition {x: 585.0 , y: 121.0},
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 585.0, y:121.0},
                                WorldPosition {x: 605.0, y:666.0},
                                WorldPosition {x: 223.0, y:666.0},
                                WorldPosition {x: 605.0, y:666.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 585.0, y: 121.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 223.0, y: 666.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 376.0 , y: 1050.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 376.0 , y: 1050.0},
                                WorldPosition {x: 264.0, y: 1049.0},
                                WorldPosition {x: 315.0, y: 1240.0},
                                WorldPosition {x: 149.0, y: 1240.0},
                                WorldPosition {x: 264.0, y: 1049.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 376.0, y: 1050.0},
                                    time: Timer::from_seconds(3.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 149.0, y: 1240.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 797.0 , y: 1050.0},
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 797.0 , y: 1050.0},
                                WorldPosition {x: 928.0, y: 1049.0},
                                WorldPosition {x: 755.0, y: 1588.0},
                                WorldPosition {x: 597.0, y: 1563.0},
                                WorldPosition {x: 597.0, y: 1693.0},
                                WorldPosition {x: 597.0, y: 1563.0},
                                WorldPosition {x: 755.0, y: 1588.0},
                                WorldPosition {x: 928.0, y: 1049.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 797.0, y: 1050.0},
                                    time: Timer::from_seconds(3.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 755.0, y: 1588.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 597.0, y: 1693.0},
                                    time: Timer::from_seconds(3.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 332.0 , y: 1851.0},
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 332.0 , y: 1851.0},
                                WorldPosition {x: 684.0, y: 1851.0},
                                WorldPosition {x: 825.0, y: 1950.0},
                                WorldPosition {x: 1360.0, y: 1950.0},
                                WorldPosition {x: 1298.0, y: 1580.0},
                                WorldPosition {x: 1364.0, y: 1351.0},
                                WorldPosition {x: 1298.0, y: 1580.0},
                                WorldPosition {x: 1360.0, y: 1950.0},
                                WorldPosition {x: 825.0, y: 1950.0},
                                WorldPosition {x: 684.0, y: 1851.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 332.0, y: 1851.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 1360.0, y: 1950.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 1364.0, y: 1351.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 1829.0 , y: 1667.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1829.0 , y: 1667.0},
                                WorldPosition {x: 2277.0, y: 1667.0},
                                WorldPosition {x: 2467.0, y: 1771.0},
                                WorldPosition {x: 2458.0, y: 1977.0},
                                WorldPosition {x: 1829.0, y: 1977.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 0.0, y: 0.0},
                                    time: Timer::from_seconds(1000.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 2548.0 , y: 1565.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 2548.0 , y: 1565.0},
                                WorldPosition {x: 2548.0, y: 449.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 2548.0 , y: 1565.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 2548.0, y: 449.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 2765.0 , y: 1794.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 2765.0 , y: 1794.0},
                                WorldPosition {x: 2961.0, y: 1741.0},
                                WorldPosition {x: 3004.0 , y: 1523.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 2765.0 , y: 1794.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 3004.0 , y: 1523.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 3297.0 , y: 1090.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 3297.0 , y: 1090.0},
                                WorldPosition {x: 3294.0, y: 549.0},
                                WorldPosition {x: 3071.0 , y: 535.0},
                                WorldPosition {x: 3055.0 , y: 310.0},
                                WorldPosition {x: 3071.0 , y: 535.0},
                                WorldPosition {x: 3294.0 , y: 549.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 3297.0 , y: 1090.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 3055.0 , y: 310.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 2738.0 , y: 215.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 2738.0 , y: 215.0},
                                WorldPosition {x: 1206.0, y: 179.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 2738.0 , y: 215.0},
                                    time: Timer::from_seconds(3.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 1206.0, y: 179.0},
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
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
                        position: WorldPosition {x: 1901.0 , y: 600.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition {x: 1901.0 , y: 600.0},
                                WorldPosition {x: 2339.0, y: 806.0},
                                WorldPosition {x: 2339.0, y: 1004.0},
                                WorldPosition {x: 2280.0, y: 1045.0},
                                WorldPosition {x: 2339.0, y: 1004.0},
                                WorldPosition {x: 2339.0, y: 806.0},
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition {x: 1901.0 , y: 600.0},
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition {x: 2280.0, y: 1045.0},
                                    time: Timer::from_seconds(3.0, TimerMode::Repeating),
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
        Level::Maze => {
            Some(
                vec![ 
                    GuardBundle { 
                        position: WorldPosition { x:  666.0 , y:  1367.0 },
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  666.0 , y:  1367.0 },
                                WorldPosition { x:  813.0 , y:  1666.0 },
                                WorldPosition { x:  1198.0 , y:  1987.0 },
                                WorldPosition { x:  573.0 , y:  1936.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  666.0 , y:  1367.0 },
                                    time: Timer::from_seconds(1.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  1198.0 , y:  1987.0 },
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
                        position: WorldPosition { x:  1206.0 , y:  1670.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  1206.0 , y:  1670.0 },
                                WorldPosition { x:  1190.0 , y:  1374.0 },
                                WorldPosition { x:  944.0 , y:  1380.0 },
                                WorldPosition { x:  953.0 , y:  1007.0 },
                                WorldPosition { x:  600.0 , y:  1002.0 },
                                WorldPosition { x:  953.0 , y:  1007.0 },
                                WorldPosition { x:  1190.0 , y:  1374.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  1206.0 , y:  1670.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  944.0 , y:  1380.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  600.0 , y:  1002.0 },
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
                        position: WorldPosition { x:  1374.0 , y:  2056.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  1374.0 , y:  2056.0 },
                                WorldPosition { x:  1502.0 , y:  2057.0 },
                                WorldPosition { x:  1501.0 , y:  1993.0 },
                                WorldPosition { x:  1645.0 , y:  1994.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  1374.0 , y:  2056.0 },
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  1645.0 , y:  1994.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition { x:  3084.0 , y:  2031.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  3084.0 , y:  2031.0 },
                                WorldPosition { x:  3123.0 , y:  1999.0 },
                                WorldPosition { x:  3129.0 , y:  1856.0 },  
                                WorldPosition { x:  3193.0 , y:  1842.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  3084.0 , y:  2031.0 },
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  3193.0 , y:  1842.0 },
                                    time: Timer::from_seconds(4.0, TimerMode::Repeating),
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
                        position: WorldPosition { x:  2827.0 , y:  718.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  2827.0 , y:  718.0 },
                                WorldPosition { x:  2346.0 , y:  718.0 },
                                WorldPosition { x:  2271.0 , y:  827.0 },
                                WorldPosition { x:  1870.0 , y:  837.0 },
                                WorldPosition { x:  1765.0 , y:  729.0 },
                                WorldPosition { x:  1637.0 , y:  729.0 },
                                WorldPosition { x:  1765.0 , y:  729.0 },
                                WorldPosition { x:  1870.0 , y:  837.0 },
                                WorldPosition { x:  2271.0 , y:  827.0 },
                                WorldPosition { x:  2346.0 , y:  718.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  2827.0 , y:  718.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  1637.0 , y:  729.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition { x:  2132.0 , y:  461.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  2132.0 , y:  461.0 },
                                WorldPosition { x:  2132.0 , y:  378.0 },
                                WorldPosition { x:  1775.0 , y:  247.0 }, 
                                WorldPosition { x:  1647.0 , y:  247.0 },
                                WorldPosition { x:  1775.0 , y:  247.0 }, 
                                WorldPosition { x:  2132.0 , y:  378.0 },
                                WorldPosition { x:  2132.0 , y:  461.0 },
                                WorldPosition { x:  2132.0 , y:  378.0 },
                                WorldPosition { x:  2354.0 , y:  235.0 },
                                WorldPosition { x:  2483.0 , y:  235.0 },
                                WorldPosition { x:  2354.0 , y:  235.0 },
                                WorldPosition { x:  2132.0 , y:  378.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  2132.0 , y:  461.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  1647.0 , y:  247.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  2132.0 , y:  461.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
                                },
                                Waiting {
                                    position: WorldPosition { x:  2483.0 , y:  235.0 },
                                    time: Timer::from_seconds(2.0, TimerMode::Repeating),
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
                        position: WorldPosition { x:  1296.0 , y:  202.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI)),
                        pace: GuardPace::Walk,
                        animation: AnimatedMotion {
                            walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                        },
                        reach: ReachDistance(20.0),
                        patrol: Patrol {
                            positions: vec![
                                WorldPosition { x:  1296.0 , y:  202.0 },
                                WorldPosition { x:  1183.0 , y:  301.0 },
                                WorldPosition { x:  883.0 , y:  304.0 },
                                WorldPosition { x:  730.0 , y:  464.0 },
                                WorldPosition { x:  550.0 , y:  263.0 },
                                WorldPosition { x:  1183.0 , y:  301.0 },
                            ],
                            waitings: vec![
                                Waiting {
                                    position: WorldPosition { x:  1296.0 , y:  202.0 },
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