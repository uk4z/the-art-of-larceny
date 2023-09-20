use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SelectButton;

#[derive(Component, Debug)]
pub struct QuitButton;

#[derive(Component, Debug)]
pub struct LevelMenu;

#[derive(Component, Debug)]
pub struct LevelLabel;

#[derive(Component, Debug)]
pub struct MainImage;

#[derive(Component, Debug)]
pub struct LevelImage;

#[derive(Component, Debug)]
pub struct BestScore;

#[derive(Component, Debug)]
pub struct BestTime;


#[derive(Component, Reflect, Default, Debug, Clone)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
pub struct Best {
    pub time: u64,
    pub score: u64,
}

#[derive(Component, Debug)]
pub struct LoadedScore;

#[derive(Component, Debug)]
pub struct BackgroundMusic;
