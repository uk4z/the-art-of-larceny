use std::time::Duration;

use bevy::time::{Timer, TimerMode};

use crate::game::{playground::{target::components::{TargetBundle, UnlockTimer}, components::{WorldPosition, ReachDistance}}, components::Level};


pub fn get_target_bundle(level: &Level) -> Option<TargetBundle> {
    match level {
        Level::Factory => {
            Some(
                TargetBundle {
                    position: WorldPosition {
                        x: 600.0,
                        y: 702.0,
                    },
                    reach: ReachDistance(40.0),
                    unlock_timer: UnlockTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)),
                },
            )
        },
        Level::Tutorial => {
            Some(
                TargetBundle {
                    position: WorldPosition {
                        x: 1402.0,
                        y: 176.0,
                    },
                    reach: ReachDistance(40.0),
                    unlock_timer: UnlockTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)),
                },
            )
        },
        Level::Warehouse => {
            Some(
                TargetBundle {
                    position: WorldPosition {
                        x: 1423.0,
                        y: 1306.0,
                    },
                    reach: ReachDistance(40.0),
                    unlock_timer: UnlockTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)),
                },
            )
        },
        Level::MillerHouse => {
            Some(
                TargetBundle {
                    position: WorldPosition {
                        x: 2064.0,
                        y: 945.0,
                    },
                    reach: ReachDistance(40.0),
                    unlock_timer: UnlockTimer(Timer::new(Duration::from_secs(2), TimerMode::Once)),
                },
            )
        },
    }
}