use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Scenery;

#[derive(Component, Debug)]
pub struct Bounds(pub Vec<Vec<i32>>);

#[derive(Bundle, Debug)]
pub struct SceneryBundle {
    pub bounds: Bounds
}

#[derive(Debug)]
pub struct BoundsEvent{
    pub handle: Handle<Image>,
}