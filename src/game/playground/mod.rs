pub mod player;
pub mod target;
pub mod scenery;
pub mod systems;
pub mod components;
pub mod extraction;
pub mod item;
pub mod guard;
pub mod camera;
pub mod security;
pub mod laser;
pub mod footage;


use bevy::prelude::*;

use systems::*;
use player::PlayerPlugin;
use scenery::{SceneryPlugin, SCENERY_SIZE};
use target::TargetPlugin;
use extraction::ExtractionPlugin;
use item::ItemPlugin;
use guard::GuardPlugin;
use camera::CameraPlugin;
use security::SecurityPlugin;
use laser::LaserPlugin;

use components::GameOver;

use crate::get_scale_reference;

use self::{footage::FootagePlugin, components::WorldPosition};



pub const WORLD_SCALE: f32 = 80.0; //80 pixels = 1 m 
pub const CHARACTER_SIZE: f32 = 50.0; //In pixels

pub struct PlaygroundPlugin;

impl Plugin for PlaygroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
                (
                    PlayerPlugin,
                    SceneryPlugin,
                    TargetPlugin,
                    ExtractionPlugin, 
                    ItemPlugin,
                    GuardPlugin,
                    SecurityPlugin,
                    CameraPlugin,
                    LaserPlugin,
                    FootagePlugin,
            ))
            .add_event::<GameOver>()
            .add_systems(PostUpdate, (
                    confine_position, 
                    world_to_screen,
            ));

    }
}


pub fn get_world_origin(
    resolution_scale: f32, 
    scenery_center: (f32, f32)
) -> Vec3 {
    let height = resolution_scale*SCENERY_SIZE.1;
    let width = resolution_scale*SCENERY_SIZE.0;

    Vec3::new(scenery_center.0-width/2.0, scenery_center.1-height/2.0, 0.0)
}

pub fn is_visible(
    visibility: &Visibility,
) -> bool {
    match visibility {
        Visibility::Hidden => {
            false
        },
        _ => {
            true
        }
    }
}


pub fn orientate_angle_with_vector(angle: f32, vector: Vec3) -> f32 {
    if vector.y >= 0.0 {
        angle
    } else  {
        -angle
    }
}


pub fn get_translation_from_world_position(
    position: &WorldPosition, 
    player_position: &WorldPosition, 
    window_size: &(f32, f32), 
) -> Vec3 {
    let center_window = Vec3::new(window_size.0/2.0, window_size.1/2.0, 0.0); 
    let scale_reference = get_scale_reference(&window_size.0, &window_size.1);

    let direction = Vec3::from(*position) - Vec3::from(*player_position)*scale_reference;

    center_window + direction
}