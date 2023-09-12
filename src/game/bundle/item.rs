use bevy::prelude::Quat;

use crate::game::playground::{item::components::ItemBundle, components::{WorldPosition, ReachDistance, Name, Value, Orientation, Path}};

pub fn get_item_bundle(level: &str) -> Option<Vec<ItemBundle>> {
    match level {
        "starting" => {
            Some(
                vec![
                    ItemBundle { 
                        position: WorldPosition {x: 890.0, y: 433.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        name: Name("Exchange rate eur/dol:".to_string()),
                        value: Value(1.2),
                        path: Path("items/Note.png".to_string()),
                    },
                ]
            )
        },
        _ => {None}
    }
}