use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, ReachDistance};

#[derive(Component, Debug)]
pub struct Security; 

#[derive(Event, Debug)]
pub struct Intrusion(pub Device);

#[derive(Component, Debug)]
pub enum Device {
    Camera, 
    Laser, 
}

#[derive(Component, Debug)]
pub struct Active(pub bool); 

#[derive(Bundle, Debug)]
pub struct SecurityBundle {
    pub position: WorldPosition,
    pub active: Active,
    pub reach: ReachDistance, 
}

