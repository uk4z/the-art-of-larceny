use crate::game::{playground::{footage::components::{FootageBundle, Available}, components::{WorldPosition, ReachDistance}}, components::Level};


pub fn get_footage_bundle(level: &Level) -> Option<FootageBundle> {
    match level {
        Level::Starting => {
            Some(
                FootageBundle {
                    position: WorldPosition {
                        x: 2395.0,
                        y: 1110.0,
                    },
                    reach: ReachDistance(60.0),
                    available: Available(true),
                }
            )
        },
        _ => {None}
    }
}