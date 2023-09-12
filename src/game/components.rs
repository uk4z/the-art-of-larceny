use bevy::prelude::*;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
    Loading,
}

pub struct ScoreEvent {
    pub comment: String,
}

#[derive(Resource, Debug)]
pub enum Level {
    Starting,
    Mock,
}