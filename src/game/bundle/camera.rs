use bevy::prelude::*;
use std::f32::consts::PI;
use crate::game::playground::{
    camera::{components::{CameraBundle, CameraPosition, Rotate, CameraPattern, FOVLength}, systems::ROTATION_CORRECTION}, 
    components::{WorldPosition, Orientation}};




pub fn get_camera_bundle(level: &str) -> Option<Vec<CameraBundle>> {
    match level {
        "starting" => {
            Some(
                vec![
                    CameraBundle {
                        position: CameraPosition {x: 1044.0, y: 332.0},
                        fov_position: WorldPosition {x:1044.0, y: 332.0},
                        orientation: Orientation(Quat::from_rotation_z(0.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
                        pattern: CameraPattern::Arc((PI/5.0, 0.0, Rotate::Trigo)),
                        fov_length: FOVLength(140.0),
                    },
                ]
            )
        },
        _ => {None}
    }
}