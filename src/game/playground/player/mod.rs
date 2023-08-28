pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;
use crate::game::playground::WORLD_SCALE;

const METERS_PER_SECOND: f32 = 2.6;//In meters 
pub const DISTANCE_PER_SECOND: f32 = METERS_PER_SECOND*WORLD_SCALE;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(motion_handler)
            .add_system(set_player_pace)
            .add_system(move_player);
    }
}

fn get_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
) -> Vec3 {
    let mut direction = Vec3::ZERO; 

    if keyboard_input.pressed(KeyCode::Up) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::new(1.0, 0.0, 0.0); 
    }
    if keyboard_input.pressed(KeyCode::Left) {
        direction += Vec3::new(-1.0, 0.0, 0.0); 
    }

    direction.normalize_or_zero()
}