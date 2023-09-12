use crate::game::playground::{extraction::components::ExtractionBundle, components::{WorldPosition, ReachDistance}};

pub fn get_extraction_bundle(level: &str) -> Option<ExtractionBundle> {
    match level {
        "starting" => {
            Some(
                ExtractionBundle {
                    position: WorldPosition {
                        x: 2075.0,
                        y: 405.0,
                    },
                    reach: ReachDistance(40.0),
                }
            )
        },
        _ => {None}
    }
}