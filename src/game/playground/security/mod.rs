pub mod systems; 
pub mod components; 

use bevy::prelude::*;
use systems::*;

pub struct SecurityPlugin;

impl Plugin for SecurityPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_security);
    }
}