use crate::game::{playground::{scenery::components::{SceneryBundle, Bounds}, components::Path}, components::Level};

pub fn get_scenery_bundle(level: &Level) -> Option<SceneryBundle> {
    match level {
        Level::Starting => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/factory.png".to_string()),
                },
            )
        },
        _ => {None}
    }
}