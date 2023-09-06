use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance};

#[derive(Component, Debug)]
pub struct Player;

#[derive(Debug, Component)]
pub enum PlayerPace {
    Walk, 
    Run, 
}

impl Into<f32> for PlayerPace {
    fn into(self) -> f32 {
        match self {
            PlayerPace::Run => {
                1.5
            },
            PlayerPace::Walk => {
                1.0
            }
        }
    }
}

#[derive(Component, Debug)]
pub enum Stealth {
    Ghost, 
    Engineer, 
    Begineer,
    None, 
}

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub position: WorldPosition,
    pub orientation: Orientation, 
    pub pace: PlayerPace,
    pub animation: AnimatedMotion,
    pub reach: ReachDistance,
    pub stealth: Stealth,
}