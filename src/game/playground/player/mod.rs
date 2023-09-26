pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;
use crate::{game::{playground::WORLD_SCALE, SimulationState, components::KeyBoard}, AppState};

const METERS_PER_SECOND: f32 = 2.6;//In meters 
pub const DISTANCE_PER_SECOND: f32 = METERS_PER_SECOND*WORLD_SCALE;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Loading), spawn_player)
            .add_systems(Update, (
                    motion_handler, 
                    set_player_pace,
                    move_player, 
                    neutralise_guard, 
                    update_stealth_on_intrusion,
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnEnter(SimulationState::Score), stop_player_footsteps)
            .add_systems(OnEnter(SimulationState::Paused), pause_player_footsteps)
            .add_systems(OnExit(AppState::Game), (
                    despawn_player,
                    despawn_corpse,
            ));
    }
}

pub fn get_player_direction(
    keyboard_input: Res<Input<KeyCode>>,
    keyboard: Res<KeyBoard>, 
) -> Vec3 {
    let mut direction = Vec3::ZERO; 

    if keyboard_input.pressed(keyboard.up.unwrap()) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(keyboard.down.unwrap()) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if keyboard_input.pressed(keyboard.right.unwrap()) {
        direction += Vec3::new(1.0, 0.0, 0.0); 
    }
    if keyboard_input.pressed(keyboard.left.unwrap()) {
        direction += Vec3::new(-1.0, 0.0, 0.0); 
    }

    direction.normalize_or_zero()
}