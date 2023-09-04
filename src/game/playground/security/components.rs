use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Security; 

#[derive(Component, Debug)]
pub struct Intrusion(pub bool); 

#[derive(Bundle, Debug)]
pub struct SecurityBundle {
    pub intrusion: Intrusion
}

