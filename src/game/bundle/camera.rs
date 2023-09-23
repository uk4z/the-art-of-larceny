use bevy::prelude::*;
use std::f32::consts::PI;
use crate::game::{playground::{
    camera::{components::{CameraBundle, CameraPosition, CameraPattern, FOVLength}, systems::ROTATION_CORRECTION}, 
    components::{WorldPosition, Orientation}}, components::Level};




pub fn get_camera_bundle(level: &Level) -> Option<Vec<CameraBundle>> {
    match level {
        Level::Tutorial => {
            Some(
                vec![
                    CameraBundle {
                        position: CameraPosition {x: 1181.0, y: 1413.0},
                        fov_position: WorldPosition {x:1181.0, y: 1413.0},
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(200.0),
                    },
                    CameraBundle {
                        position: CameraPosition {x: 1967.0, y: 172.0},
                        fov_position: WorldPosition {x:1967.0, y: 172.0},
                        orientation: Orientation(Quat::from_rotation_z(PI)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                ]
            )
        },
        Level::Factory => {
            Some(
                vec![
                    CameraBundle {
                        position: CameraPosition {x: 3315.0, y: 640.0},
                        fov_position: WorldPosition {x:3315.0, y: 640.0},
                        orientation: Orientation(Quat::from_rotation_z(PI)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(200.0),
                    },

                    CameraBundle {
                        position: CameraPosition {x: 3315.0, y: 1600.0},
                        fov_position: WorldPosition {x:3315.0, y: 1600.0},
                        orientation: Orientation(Quat::from_rotation_z(PI)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(200.0),
                    },

                    CameraBundle {
                        position: CameraPosition {x: 1750.0, y: 46.0},
                        fov_position: WorldPosition {x:1750.0, y: 46.0},
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                    
                    CameraBundle {
                        position: CameraPosition {x: 1865.0, y: 1771.0},
                        fov_position: WorldPosition {x:1865.0, y: 1771.0},
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },

                    CameraBundle {
                        position: CameraPosition {x: 745.0, y: 970.0},
                        fov_position: WorldPosition {x:745.0, y: 970.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(200.0),
                    },
                ]
            )
        },
        Level::Warehouse => {
            Some(
                vec![
                    CameraBundle {
                        position: CameraPosition {x: 2105.0, y: 742.0},
                        fov_position: WorldPosition {x:2105.0, y: 742.0},
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                    CameraBundle {
                        position: CameraPosition {x: 2355.0, y: 1682.0},
                        fov_position: WorldPosition {x:2355.0, y: 1682.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                    CameraBundle {
                        position: CameraPosition {x: 3314.0, y: 850.0},
                        fov_position: WorldPosition {x:3314.0, y: 850.0},
                        orientation: Orientation(Quat::from_rotation_z(PI)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                ]
            )
        },
        Level::MillerHouse => {
            None
        },
        Level::Maze => {
            Some(
                vec![
                    CameraBundle {
                        position: CameraPosition { x:  279.0 , y:  955.0 },
                        fov_position: WorldPosition { x:  279.0 , y:  955.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                ]
            )
        },
        Level::Office => {
            Some(
                vec![
                    CameraBundle {
                        position: CameraPosition { x:  1406.0 , y:  1654.0 },
                        fov_position: WorldPosition { x:  1406.0 , y:  1654.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                    CameraBundle {
                        position: CameraPosition { x:  1712.0 , y:  1565.0 },
                        fov_position: WorldPosition { x:  1712.0 , y:  1565.0 },
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                    CameraBundle {
                        position: CameraPosition { x:  1600.0 , y:  513.0 },
                        fov_position: WorldPosition { x:  1600.0 , y:  513.0 },
                        orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                    CameraBundle {
                        position: CameraPosition { x:  3077.0 , y:  46.0 },
                        fov_position: WorldPosition { x:  3077.0 , y:  46.0 },
                        orientation: Orientation(Quat::from_rotation_z(PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Static,
                        fov_length: FOVLength(300.0),
                    },
                ]
            )
        },
    }
}