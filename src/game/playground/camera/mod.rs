pub mod components; 
pub mod systems; 

use bevy::prelude::*;
use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(alert_security)
            .add_system(update_fov_position)
            .add_system(rotate_cameras);
    }
}