use bevy::prelude::Quat;

use crate::game::{playground::{laser::components::{LaserBundle, LaserLength, Direction}, components::{Orientation, WorldPosition}}, components::Level};

pub fn get_laser_bundle(level: &Level) -> Option<Vec<LaserBundle>> {
    match level {
        Level::Starting => {
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