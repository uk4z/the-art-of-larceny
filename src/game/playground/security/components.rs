use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, ReachDistance};

#[derive(Component, Debug)]
pub struct Security; 

#[derive(Component, Debug)]
pub struct Intrusion(pub bool);

#[derive(Component, Debug)]
pub struct Active(pub bool); 

#[derive(Bundle, Debug)]
pub struct SecurityBundle {
    pub position: WorldPosition,
    pub intrusion: Intrusion,
    pub active: Active,
    pub reach: ReachDistance, 
}

