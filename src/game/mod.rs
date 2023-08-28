pub mod playground;
pub mod board; 

use bevy::prelude::*;

use self::playground::PlaygroundPlugin;
use self::board::BoardPlugin;

#[derive(Debug)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlaygroundPlugin)
            .add_plugin(BoardPlugin);
    }
}