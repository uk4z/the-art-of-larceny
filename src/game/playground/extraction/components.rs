use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, ReachDistance};

#[derive(Component, Debug)]
pub struct Extraction;


#[derive(Bundle, Debug)]
pub  struct ExtractionBundle {
    pub position: WorldPosition,
    pub reach: ReachDistance,
}
