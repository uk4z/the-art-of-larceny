use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};

use super::components::Scenery;
use super::{get_scenery_position_from_window, get_scenery_scale_from_window};
use crate::components::Layer;


pub fn spawn_scenery(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();

    let (x, y) = get_scenery_position_from_window(&window.width(), &window.height());

    let scale = get_scenery_scale_from_window(&window.width(), &window.height());
    commands.spawn(
        (SpriteBundle{
            texture: asset_server.load("levels/backgrounds/test.png"),
            transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(scale, scale, Layer::Scenery.into())),
        ..default()
        },
        Scenery,
    ));
}

