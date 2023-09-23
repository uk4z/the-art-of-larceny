use crate::game::{playground::{footage::components::{FootageBundle, Available}, components::{WorldPosition, ReachDistance}}, components::Level};


pub fn get_footage_bundle(level: &Level) -> Option<FootageBundle> {
    match level {
        Level::Factory => {
            Some(
                FootageBundle {
                    position: WorldPosition {
                        x: 2534.0,
                        y: 1171.0,
                    },
                    reach: ReachDistance(40.0),
                    available: Available(true),
                }
            )
        },
        Level::Tutorial => {
            Some(
                FootageBundle {
                    position: WorldPosition {
                        x: 1610.0,
                        y: 327.0,
                    },
                    reach: ReachDistance(40.0),
                    available: Available(true),
                }
            )
        },
        Level::Warehouse => {
            Some(
                FootageBundle {
                    position: WorldPosition {
                        x: 1443.0,
                        y: 1449.0,
                    },
                    reach: ReachDistance(40.0),
                    available: Available(true),
                }
            )
        },
        Level::MillerHouse => {
            None
        },
        Level::Maze => {
            Some(
                FootageBundle {
                    position: WorldPosition {
                        x: -100.0,
                        y: -100.0,
                    },
                    reach: ReachDistance(40.0),
                    available: Available(true),
                }
            )
        },
        Level::Office => {
            Some(
                FootageBundle {
                    position: WorldPosition { x:  2913.0 , y:  1642.0 },
                    reach: ReachDistance(40.0),
                    available: Available(true),
                }
            )
        },
    }
}