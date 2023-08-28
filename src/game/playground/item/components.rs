use bevy::prelude::*; 
use crate::game::playground::components::{WorldPosition, ReachDistance, Orientation};
use crate::game::playground::components::{Name, Value};

#[derive(Debug, Component)]
pub struct Item; 

#[derive(Debug, Bundle)]
pub struct ItemBundle {
    pub position: WorldPosition, 
    pub reach: ReachDistance,
    pub orientation: Orientation, 
    pub name: Name,
    pub value: Value, 
}