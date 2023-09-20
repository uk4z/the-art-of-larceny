use bevy::prelude::*;

use crate::game::playground::components::Path;

#[derive(Component, Debug)]
pub struct Scenery;

#[derive(Component, Debug)]
pub struct Bounds(pub Vec<Vec<i32>>);

#[derive(Bundle, Debug)]
pub struct SceneryBundle {
    pub bounds: Bounds,
    pub path: Path,
}

#[derive(Resource, Debug)]
pub struct BoundsResource{
    pub handle: Option<Handle<Image>>,
}