use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Scenery;

#[derive(Component, Debug)]
pub struct Bounds(pub Vec<Vec<i32>>);

#[derive(Bundle, Debug)]
pub struct SceneryBundle {
    pub bounds: Bounds
}

#[derive(Resource, Debug)]
pub struct BoundsResource{
    pub handle: Handle<Image>,
}