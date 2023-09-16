use crate::game::{playground::{security::components::{Active, SecurityBundle, Intrusion}, components::{ReachDistance, WorldPosition}}, components::Level};


pub fn get_security_bundle(level: &Level) -> Option<SecurityBundle> {
    match level {
        Level::Starting => {
            Some(
                SecurityBundle {
                    position: WorldPosition {x: 2408.0, y: 1291.0},
                    intrusion: Intrusion(false),
                    active: Active(true),
                    reach: ReachDistance(40.0),
                },
            )
        },
        _ => {None}
    }
}