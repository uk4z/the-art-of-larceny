use bevy::prelude::*;

use crate::game::playground::{components::{WorldPosition, ReachDistance}, player::components::Stealth};

#[derive(Component, Debug)]
pub struct Extraction;


#[derive(Bundle, Debug)]
pub  struct ExtractionBundle {
    pub position: WorldPosition,
    pub reach: ReachDistance,
}

#[derive(Event, Debug)]
pub struct LevelCompleted {
    pub stealth: Stealth
}
