pub mod components; 
pub mod systems; 

use bevy::prelude::*;
use systems::*;

use crate::{AppState, game::SimulationState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_camera.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    alert_security, 
                    update_fov_position, 
                    rotate_cameras,
                ) 
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}