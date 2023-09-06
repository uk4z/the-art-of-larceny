use bevy::prelude::*;

use crate::game::playground::components::{WorldPosition, ReachDistance};

#[derive(Component, Debug)]
pub struct Footage;


#[derive(Component, Debug)]
pub struct Available(pub bool);

#[derive(Bundle, Debug)]
pub  struct FootageBundle {
    pub position: WorldPosition,
    pub reach: ReachDistance,
    pub available: Available, 
}
