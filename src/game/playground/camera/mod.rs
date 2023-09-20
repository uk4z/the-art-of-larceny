pub mod components; 
pub mod systems; 

use bevy::prelude::*;
use systems::*;

use crate::{AppState, game::SimulationState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Loading), spawn_camera)
            .add_systems(Update, (
                alert_security, 
                update_fov_position, 
                rotate_cameras 
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_camera);
    }
}