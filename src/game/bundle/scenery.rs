use crate::game::playground::{scenery::components::{SceneryBundle, Bounds}, components::Path};

pub fn get_scenery_bundle(level: &str) -> Option<SceneryBundle> {
    match level {
        "starting" => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/test.png".to_string()),
                },
            )
        },
        _ => {None}
    }
}