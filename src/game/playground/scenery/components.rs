use bevy::prelude::*;

use crate::game::playground::components::Path;

#[derive(Component, Debug)]
pub struct Scenery;

#[derive(Component, Debug)]
pub struct Bounds(pub Vec<Vec<i32>>);

#[derive(Component, Debug)]
pub struct ScenerySize{
    pub width: f32, 
    pub height: f32
}

#[derive(Bundle, Debug)]
pub struct SceneryBundle {
    pub bounds: Bounds,
    pub path: Path,
    pub size: ScenerySize, 
}

#[derive(Resource, Debug)]
pub struct BoundsResource{
    pub handle: Option<Handle<Image>>,
}