use crate::game::playground::{footage::components::{FootageBundle, Available}, components::{WorldPosition, ReachDistance}};


pub fn get_footage_bundle(level: &str) -> Option<FootageBundle> {
    match level {
        "starting" => {
            Some(
                FootageBundle {
                    position: WorldPosition {
                        x: 1800.0,
                        y: 817.0,
                    },
                    reach: ReachDistance(40.0),
                    available: Available(true),
                }
            )
        },
        _ => {None}
    }
}