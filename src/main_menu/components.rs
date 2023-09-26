use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SelectButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct CloseButton;


#[derive(Component, Debug)]
pub struct QuitButton;

#[derive(Component, Debug)]
pub struct LevelMenu;

#[derive(Component)]
pub struct SettingsMenu;

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

#[derive(Component, Debug)]
pub struct UpField;

#[derive(Component, Debug)]
pub struct DownField;

#[derive(Component, Debug)]
pub struct RightField;

#[derive(Component, Debug)]
pub struct LeftField;

#[derive(Component, Debug)]
pub struct RunField;

#[derive(Component, Debug)]
pub struct InteractField;

#[derive(Component, Debug)]
pub enum SettingKeyButton{
    Up, 
    Down, 
    Right, 
    Left, 
    Run, 
    Interact,
}

#[derive(Component, Debug)]
pub struct SelectValue;
