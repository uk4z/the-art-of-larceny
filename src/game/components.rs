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
    pub value: u64, 
}

#[derive(Resource, Debug)]
pub enum Level {
    Starting,
}

#[derive(Resource, Debug)]
pub struct ItemCount(pub u8);

#[derive(Resource, Debug)]
pub struct GameTime(pub Instant);