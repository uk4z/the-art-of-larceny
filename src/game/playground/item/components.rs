use bevy::prelude::*; 
use crate::game::playground::components::{WorldPosition, ReachDistance, Orientation, Path};

#[derive(Debug, Component)]
pub struct Item; 

#[derive(Debug, Bundle)]
pub struct ItemBundle {
    pub position: WorldPosition, 
    pub reach: ReachDistance,
    pub orientation: Orientation, 
    pub path: Path, 
}