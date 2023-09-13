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
            .add_system(spawn_laser.in_schedule(OnEnter(SimulationState::Loading)))
            .add_systems(
                (
                    alert_security,  
                ) 
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_laser.in_schedule(OnExit(AppState::Game)));
    }
}

pub fn laser_extremum(
    position: &WorldPosition,
    orientation: &Orientation,
    length: &LaserLength,
) -> Vec<Vec3> {
    let ref_vector = orientation.0.mul_vec3(Vec3::X*length.0/2.0);
    let up_vector = orientation.0.mul_quat(Quat::from_rotation_z(PI/2.0)).normalize().mul_vec3(Vec3::X*2.0);

    
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