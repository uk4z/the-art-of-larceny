use std::{time::Duration, f32::consts::PI};

use bevy::{prelude::Quat, time::{TimerMode, Timer}};

use crate::game::{playground::{components::{Orientation, WorldPosition, AnimatedMotion, ReachDistance}, player::components::{PlayerBundle, PlayerPace, Stealth}}, components::Level};


pub fn get_player_bundle(level: &Level) -> Option<PlayerBundle> {
    match level {
        Level::Factory => {
            Some(
                PlayerBundle { 
                    position: WorldPosition {
                        x: 3208.0,
                        y: 1128.0,
                    },
                    orientation: Orientation(Quat::IDENTITY),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )
        },
        Level::Tutorial => {
            Some(
                PlayerBundle { 
                    position: WorldPosition {
                        x: 678.0,
                        y: 112.0,
                    },
                    orientation: Orientation(Quat::from_rotation_z(PI/2.0)),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )
        },
        Level::Warehouse => {
            Some(
                PlayerBundle { 
                    position: WorldPosition {
                        x: 290.0,
                        y: 541.0,
                    },
                    orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )
        },
        Level::MillerHouse => {
            Some(
                PlayerBundle { 
                    position: WorldPosition {
                        x: 204.0,
                        y: 196.0,
                    },
                    orientation: Orientation(Quat::from_rotation_z(PI)),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )
        },
        Level::Maze => {
            Some(
                PlayerBundle { 
                    position: WorldPosition { x: 35.0, y: 1886.0 },
                    orientation: Orientation(Quat::from_rotation_z(0.0)),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )       
        },
        Level::Office => {
            Some(
                PlayerBundle { 
                    position: WorldPosition { x:  1640.0 , y: 2210.0 },
                    orientation: Orientation(Quat::from_rotation_z(0.0)),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )       
        },
        Level::Canyon => {
            Some(
                PlayerBundle { 
                    position: WorldPosition { x:  3186.0 , y:  27.0 },
                    orientation: Orientation(Quat::from_rotation_z(PI/2.0)),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(40.0),
                    stealth: Stealth::Ghost,
                },
            )       
        },
    }
}