use crate::game::{playground::{scenery::components::{SceneryBundle, Bounds, ScenerySize}, components::Path}, components::Level};

pub fn get_scenery_bundle(level: &Level) -> Option<SceneryBundle> {
    match level {
        Level::Factory => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/factory.png".to_string()),
                    size: ScenerySize{width: 3360.0, height: 2240.0}
                },
            )
        },
        Level::Tutorial => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/tutorial.png".to_string()),
                    size: ScenerySize{width: 2000.0, height: 2000.0}
                },
            )
        },
        Level::Warehouse => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/warehouse.png".to_string()),
                    size: ScenerySize{width: 3360.0, height: 2240.0}
                },
            )
        },
        Level::MillerHouse => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/millerhouse.png".to_string()),
                    size: ScenerySize{width: 3360.0, height: 2240.0}
                },
            )
        },
        Level::Maze => {
            Some(
                SceneryBundle {
                    bounds: Bounds(Vec::new()),
                    path: Path("levels/backgrounds/maze.png".to_string()),
                    size: ScenerySize{width: 3360.0, height: 2240.0}
                },
            )
        },
    }
}