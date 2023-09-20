use crate::game::{playground::{security::components::{Active, SecurityBundle}, components::{ReachDistance, WorldPosition}}, components::Level};


pub fn get_security_bundle(level: &Level) -> Option<SecurityBundle> {
    match level {
        Level::Starting => {
            Some(
                SecurityBundle {
                    position: WorldPosition {x: 2534.0, y: 1411.0}, 
                    active: Active(true),
                    reach: ReachDistance(40.0),
                },
            )
        },
    }
}