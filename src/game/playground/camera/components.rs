use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, Orientation};

#[derive(Component, Debug)]
pub struct Camera; 

#[derive(Component, Debug, Clone, Copy)]
pub enum Rotate {
    Trigo, 
    AntiTrigo,
}

#[derive(Component, Debug)]
pub enum CameraPattern {
    Static, 
    Arc((f32,f32, Rotate)), 
}

#[derive(Component, Debug)]
pub struct FOVLength(pub f32);

#[derive(Component, Debug)]
pub struct CameraPosition {
    pub x: f32, 
    pub y: f32, 
}

#[derive(Bundle, Debug)]
pub struct CameraBundle {
    pub position: CameraPosition,
    pub fov_position: WorldPosition, 
    pub orientation: Orientation,
    pub pattern: CameraPattern,
    pub fov_length: FOVLength, 
}
