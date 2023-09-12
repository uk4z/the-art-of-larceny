use std::time::Duration;

use bevy::{prelude::Quat, time::{TimerMode, Timer}};

use crate::game::{playground::{components::{Orientation, WorldPosition, AnimatedMotion, ReachDistance}, player::components::{PlayerBundle, PlayerPace, Stealth}}, components::Level};


pub fn get_player_bundle(level: &Level) -> Option<PlayerBundle> {
    match level {
        Level::Starting => {
            Some(
                PlayerBundle { 
                    position: WorldPosition {
                        x: 2075.0,
                        y: 405.0,
                    },
                    orientation: Orientation(Quat::IDENTITY),
                    pace: PlayerPace::Walk,
                    animation: AnimatedMotion {
                        walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                        run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
                    },
                    reach: ReachDistance(20.0),
                    stealth: Stealth::Ghost,
                },
            )
        },
        _ => {None}
    }
}