pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::*;

use crate::{game::SimulationState, AppState};

use self::components::LaserLength;

use std::f32::{consts::PI, INFINITY};
use super::components::{WorldPosition, Orientation};

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SimulationState::Loading), spawn_laser)
            .add_systems(Update, (
                    burn_player,  
            ).run_if(in_state(SimulationState::Running))
            )
            .add_systems(OnExit(AppState::Game), despawn_laser);
    }
}

pub fn laser_extremum(
    position: &WorldPosition,
    orientation: &Orientation,
    length: &LaserLength,
) -> Vec<Vec3> {
    let ref_vector = orientation.0.mul_vec3(Vec3::X*length.0/2.0);
    let up_vector = orientation.0.mul_quat(Quat::from_rotation_z(PI/2.0)).normalize().mul_vec3(Vec3::X*35.0);

    
    vec![Vec3::from(*position)+(ref_vector+up_vector), 
        Vec3::from(*position)+(ref_vector-up_vector), 
        Vec3::from(*position)+(-ref_vector+up_vector), 
        Vec3::from(*position)+(-ref_vector-up_vector)]
}


fn get_extremum_values(list: Vec<f32>) -> (f32, f32) {
    let mut minimum = INFINITY;  
    let mut maximum = 0.0;
    for value in list.iter() {
        if minimum > *value {
            minimum = *value; 
        }
        if maximum < *value {
            maximum = *value;  
        }
    }
    (minimum, maximum)
}