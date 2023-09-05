use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, Orientation};
use std::f32::consts::PI;

#[derive(Component, Debug)]
pub struct Laser; 

#[derive(Component, Debug)]
pub struct LaserLength(pub f32); 

#[derive(Bundle, Debug)]
pub struct LaserBundle {
    pub position: WorldPosition,
    pub orientation: Orientation, 
    pub length: LaserLength,
}

pub enum Direction {
    Vertical, 
    Horizontal,
}

impl Into<f32> for Direction {
    fn into(self) -> f32 {
        match self {
            Direction::Horizontal => {
                0.0
            }, 
            Direction::Vertical => {
                PI/2.0
            }
        }
    }
}