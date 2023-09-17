use bevy::{prelude::{Component, Resource}, time::Timer};

#[derive(Component, Debug)]
pub struct ScoreMenu;

#[derive(Component, Debug)]
pub struct LeaveButton;

#[derive(Component, Debug)]
pub struct ItemBoard;

#[derive(Component, Debug)]
pub struct TimeBoard;

#[derive(Component, Debug)]
pub struct TotalScore;

#[derive(Component, Debug)]
pub struct ScoreValue(pub u64);

#[derive(Resource, Debug)]
pub struct ScoreTimer{
    pub timer: Timer,
    pub index: usize, 
}

#[derive(Component, Debug)]
pub struct SubScores;

#[derive(Component, Debug)]
pub struct ElapsedTime(pub u64);