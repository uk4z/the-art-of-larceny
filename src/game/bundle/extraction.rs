use crate::game::{playground::{extraction::components::ExtractionBundle, components::{WorldPosition, ReachDistance}}, components::Level};

pub fn get_extraction_bundle(level: &Level) -> Option<ExtractionBundle> {
    match level {
        Level::Factory => {
            Some(
                ExtractionBundle {
                    position: WorldPosition {x: 2910.0, y: 217.0},
                    reach: ReachDistance(40.0),
                }
            )
        },
        Level::Tutorial => {
            Some(
                ExtractionBundle {
                    position: WorldPosition {x: 750.0, y: 1626.0},
                    reach: ReachDistance(40.0),
                }
            )
        },
        Level::Warehouse => {
            Some(
                ExtractionBundle {
                    position: WorldPosition {x: 290.0, y: 220.0},
                    reach: ReachDistance(40.0),
                }
            )
        },
    }
}