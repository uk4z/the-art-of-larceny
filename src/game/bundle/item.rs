use bevy::prelude::Quat;

use crate::game::{playground::{item::components::ItemBundle, components::{WorldPosition, ReachDistance, Orientation, Path}}, components::Level};

pub fn get_item_bundle(level: &Level) -> Option<Vec<ItemBundle>> {
    match level {
        Level::Factory => {
            Some(
                vec![
                    ItemBundle { 
                        position: WorldPosition {x: 3279.0, y: 610.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },

                    ItemBundle { 
                        position: WorldPosition {x: 3279.0, y: 1674.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },

                    ItemBundle { 
                        position: WorldPosition {x: 2193.0, y: 232.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },

                    ItemBundle { 
                        position: WorldPosition {x: 113.0, y: 190.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                ]
            )
        },
        Level::Tutorial => {
            Some(
                vec![
                    ItemBundle { 
                        position: WorldPosition {x: 1578.0, y: 65.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 1648.0, y: 65.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 1718.0, y: 65.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 1788.0, y: 65.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 1858.0, y: 65.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                ]
            )
        },
        Level::Warehouse => {
            Some(
                vec![
                    ItemBundle { 
                        position: WorldPosition {x: 614.0, y: 1050.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 2099.0, y: 1653.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 2899.0, y: 513.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 3218.0, y: 2008.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 1862.0, y: 350.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition {x: 1806.0, y: 961.0,},
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                ]
            )
        },
        Level::MillerHouse => {
            Some(
                vec![
                    ItemBundle { 
                        position: WorldPosition { x:  3117.0 , y:  1897.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition { x:  178.0 , y:  1629.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition { x:  1585.0 , y:  1519.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition { x:  89.0 , y:  1270.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition { x:  1105.0 , y:  1270.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition { x:  1267.0 , y:  397.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                    ItemBundle { 
                        position: WorldPosition { x:  1822.0 , y:  80.0 },
                        orientation: Orientation(Quat::IDENTITY),
                        reach: ReachDistance(40.0),
                        path: Path("items/diamond.png".to_string()),
                    },
                ]
                
            )
        },
    }
}