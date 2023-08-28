use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BoardUI;

#[derive(Component, Debug)]
pub struct Helper;

#[derive(Component, Debug)]
pub struct LockedButton;

#[derive(Component, Debug)]
pub struct UnlockedButton;


#[derive(Component, Debug)]
pub struct LoadingBar;

#[derive(Component, Debug)]
pub struct OpenTarget;

#[derive(Resource, Debug)]
pub struct CurrencyLocked(pub bool);

#[derive(Component, Debug)]
pub struct ItemBoard;
