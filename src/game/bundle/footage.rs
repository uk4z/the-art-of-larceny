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
    }
}