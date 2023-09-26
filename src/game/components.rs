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

#[derive(Event, Debug)]
pub struct ScoreEvent {
    pub comment: String,
    pub value: u64, 
}

#[derive(Resource, Debug)]
pub enum Level {
    Tutorial,
    MillerHouse, 
    Factory,
    Warehouse,
    Maze, 
    Office,
    Canyon, 
}

#[derive(Resource, Debug)]
pub struct ItemCount(pub u8);

#[derive(Resource, Debug)]
pub struct GameTime(pub Instant);

#[derive(Resource, Debug)]
pub struct KeyBoard {
    pub up: Option<KeyCode>,
    pub down: Option<KeyCode>, 
    pub right: Option<KeyCode>, 
    pub left: Option<KeyCode>, 
    pub run: Option<KeyCode>,
    pub interact: Option<KeyCode>, 
}
