use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Helper;


#[derive(Resource, Debug)]
pub struct CurrencyLocked(pub bool);

#[derive(Component, Debug)]
pub struct StealthIcon;

#[derive(Component, Debug)]
pub struct HelperMenu;