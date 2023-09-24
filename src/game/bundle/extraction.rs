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
        Level::MillerHouse => {
            Some(
                ExtractionBundle {
                    position: WorldPosition {x: 946.0, y: 102.0},
                    reach: ReachDistance(40.0),
                }
            )
        },
        Level::Maze => {
            Some(
                ExtractionBundle {
                    position: WorldPosition { x:  204.0 , y:  122.0 },
                    reach: ReachDistance(40.0),
                }
            )
        },
        Level::Office => {
            Some(
                ExtractionBundle {
                    position: WorldPosition { x:  1640.0 , y: 2210.0 },
                    reach: ReachDistance(40.0),
                }
            )
        },
        Level::Canyon => {
            Some(
                ExtractionBundle {
                    position: WorldPosition { x:  2274.0 , y:  64.0 },
                    reach: ReachDistance(40.0),
                }
            )
        },
    }
}