use bevy::prelude::Quat;

use crate::game::playground::{laser::components::{LaserBundle, LaserLength, Direction}, components::{Orientation, WorldPosition}};

pub fn get_laser_bundle(level: &str) -> Option<Vec<LaserBundle>> {
    match level {
        "starting" => {
            Some(
                vec![
                    LaserBundle {
                        position: WorldPosition {x: 1246.0, y: 945.0},
                        orientation: Orientation(Quat::from_rotation_z(Direction::Horizontal.into())), //angle is 0.0 or PI/2.0 ; 
                        length: LaserLength(146.0)
                    },
                ]
            )
        },
        _ => {None}
    }
}