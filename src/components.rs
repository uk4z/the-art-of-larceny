use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum Layer {
    UI, 
    Scenery, 
    Interactable, 
    Camera,
}

impl Into<f32> for Layer {
    fn into(self) -> f32 {
        match self {
            Layer::Scenery => {
                0.0
            },
            Layer::Interactable => {
                10.0
            }
            Layer::UI => {
                20.0
            },
            Layer::Camera => {
                999.9
            }
        }
    }
}