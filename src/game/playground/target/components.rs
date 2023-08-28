use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, ReachDistance};

#[derive(Component, Debug)]
pub struct Target;

#[derive(Component, Debug)]
pub struct UnlockTimer(pub Timer);

#[derive(Bundle, Debug)]
pub  struct TargetBundle {
    pub position: WorldPosition,
    pub reach: ReachDistance,
    pub unlock_timer: UnlockTimer,
}
