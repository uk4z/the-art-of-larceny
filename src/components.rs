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
                1.0
            }
            Layer::UI => {
                2.0
            },
            Layer::Camera => {
                999.9
            }
        }
    }
}