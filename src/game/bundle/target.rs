use std::time::Duration;

use bevy::time::{Timer, TimerMode};

use crate::game::{playground::{target::components::{TargetBundle, UnlockTimer}, components::{WorldPosition, ReachDistance}}, components::Level};


pub fn get_target_bundle(level: &Level) -> Option<TargetBundle> {
    match level {
        Level::Starting => {
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
        _ => {None}
    }
}