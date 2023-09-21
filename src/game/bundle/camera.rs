use bevy::prelude::*;
use std::f32::consts::PI;
use crate::game::{playground::{
    camera::{components::{CameraBundle, CameraPosition, CameraPattern, FOVLength}, systems::ROTATION_CORRECTION}, 
    components::{WorldPosition, Orientation}}, components::Level};




pub fn get_camera_bundle(level: &Level) -> Option<Vec<CameraBundle>> {
    match level {
        Level::Starting => {
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
    }
}