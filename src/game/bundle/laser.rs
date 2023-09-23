use bevy::prelude::Quat;

use crate::game::{playground::{laser::components::{LaserBundle, LaserLength, Direction}, components::{Orientation, WorldPosition}}, components::Level};

pub fn get_laser_bundle(level: &Level) -> Option<Vec<LaserBundle>> {
    match level {
        Level::Factory => {
            Some(
                vec![
                    LaserBundle {
                        position: WorldPosition {x: 596.0, y: 1016.0},
                        orientation: Orientation(Quat::from_rotation_z(Direction::Horizontal.into())), //angle is 0.0 or PI/2.0 ; 
                        length: LaserLength(118.0),
                    },
                    LaserBundle {
                        position: WorldPosition {x: 736.0, y: 172.0},
                        orientation: Orientation(Quat::from_rotation_z(Direction::Vertical.into())), //angle is 0.0 or PI/2.0 ; 
                        length: LaserLength(114.0),
                    },
                ]
            )
        },
        Level::Tutorial => {
            Some(
                vec![
                    LaserBundle {
                        position: WorldPosition {x: 1575.0, y: 1230.0},
                        orientation: Orientation(Quat::from_rotation_z(Direction::Horizontal.into())), //angle is 0.0 or PI/2.0 ; 
                        length: LaserLength(284.0),
                    },
                ]
            )
        },
        Level::Warehouse => {
            Some(
                vec![
                    LaserBundle {
                        position: WorldPosition {x: 2510.0, y: 1120.0},
                        orientation: Orientation(Quat::from_rotation_z(Direction::Horizontal.into())), //angle is 0.0 or PI/2.0 ; 
                        length: LaserLength(311.0),
                    },
                    LaserBundle {
                        position: WorldPosition {x: 1574.0, y: 731.0},
                        orientation: Orientation(Quat::from_rotation_z(Direction::Horizontal.into())), //angle is 0.0 or PI/2.0 ; 
                        length: LaserLength(118.0),
                    },
                ]
            )
        },
        Level::MillerHouse => {
            None
        },
        Level::Maze => {
            None
        },
    }
}