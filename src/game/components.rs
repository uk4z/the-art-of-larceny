use std::time::Instant;

use bevy::prelude::*;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    None,
    Running,
    Paused,
    Loading,
    Score, 
}

pub struct ScoreEvent {
    pub comment: String,
}

#[derive(Resource, Debug)]
pub enum Level {
    Starting,
    Mock,
}


#[derive(Resource, Debug)]
pub struct ItemCount(pub u8);

#[derive(Resource, Debug)]
pub struct GameTime(pub Instant);