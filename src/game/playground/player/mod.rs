pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;
use crate::{game::{playground::WORLD_SCALE, SimulationState}, AppState};

const METERS_PER_SECOND: f32 = 2.6;//In meters 
pub const DISTANCE_PER_SECOND: f32 = METERS_PER_SECOND*WORLD_SCALE;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.in_schedule(OnEnter(SimulationState::Loading)))
            .add_systems(
                (
                    motion_handler, 
                    set_player_pace,
                    move_player, 
                    neutralise_guard, 
                ) 
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    despawn_player,
                    despawn_corpse,
                ).in_schedule(OnExit(AppState::Game)));
    }
}

pub fn get_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
) -> Vec3 {
    let mut direction = Vec3::ZERO; 

    if keyboard_input.pressed(KeyCode::Z) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0); 
    }
    if keyboard_input.pressed(KeyCode::Q) {
        direction += Vec3::new(-1.0, 0.0, 0.0); 
    }

    direction.normalize_or_zero()
}