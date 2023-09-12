use crate::game::{playground::{footage::components::{FootageBundle, Available}, components::{WorldPosition, ReachDistance}}, components::Level};


pub fn get_footage_bundle(level: &Level) -> Option<FootageBundle> {
    match level {
        Level::Starting => {
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