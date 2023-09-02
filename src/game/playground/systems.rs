use bevy::prelude::*;
use bevy::window::WindowResized;

use super::components::WorldPosition;
use super::player::components::{Player, PlayerPace};
use super::guard::components::Guard;
use super::scenery::{
    components::Scenery,
    SCENERY_SIZE,
    get_scenery_scale_from_window,
    get_scenery_position_from_window,
};
use super::CHARACTER_SIZE;

use crate::components::Layer;
use super::components::Orientation;
use super::get_world_origin;

pub fn confine_position(
    mut positions_q: Query<(&mut WorldPosition, &PlayerPace)>,
) {
    println!("{:?}", positions_q);
    let height_limit = SCENERY_SIZE.1-CHARACTER_SIZE/2.0; 
    let width_limit = SCENERY_SIZE.0-CHARACTER_SIZE/2.0; 

    let (x_max, y_max) = (width_limit, height_limit);
    let (x_min, y_min) = (CHARACTER_SIZE/2.0, CHARACTER_SIZE/2.0);

    positions_q.for_each_mut(|(mut position,_)| {
        if position.x > x_max {
            position.x = x_max;
        }
        if position.x < x_min {
            position.x = x_min;
        }
        if position.y > y_max {
            position.y = y_max;
        }
        if position.y < y_min {
            position.y = y_min;
        }
    });
}

pub fn update_scale(
    mut resize_event: EventReader<WindowResized>, 
    mut world_q: Query<(&mut Transform, &WorldPosition), Without<Scenery>>,
    mut scenery_q: Query<&mut Transform, With<Scenery>>,
) {
    for resized_window in resize_event.iter() {
        if let Ok(mut scenery_t) = scenery_q.get_single_mut() {
            let new_width = resized_window.width;
            let new_height = resized_window.height;
            let new_scale = get_scenery_scale_from_window(&new_width, &new_height);
            let (new_x, new_y) = get_scenery_position_from_window(&new_width, &new_height);

            world_q.for_each_mut(|(mut transform, position)| {
                let z_layer = transform.translation.z;
                let window_position = Vec3::new(new_x, new_y, z_layer)-Vec3::from(*position);

                transform.translation = window_position;
                transform.scale = Vec3::new(new_scale, new_scale, 1.0)

            });
            scenery_t.translation = Vec3::new(new_x, new_y, Layer::Scenery.into());
            scenery_t.scale = Vec3::new(new_scale, new_scale, 1.0);
        }
    }
    
}

pub fn world_to_screen(
    mut interactable_q: Query<(&mut Transform, &WorldPosition, Option<&Orientation>), Without<Scenery>>,
    scenery_q: Query<&mut Transform, With<Scenery>>,
) {
    if let Ok(scenery_t) = scenery_q.get_single() {
        let scenery_position = scenery_t.translation - Vec3::new(0.0, 0.0, Layer::Scenery.into());
        let scale = scenery_t.scale.x;

        interactable_q.for_each_mut(|(mut transform, position, orientation)| {
            let origin = get_world_origin(scale, (scenery_position.x, scenery_position.y));
            let in_screen_position = origin + Vec3::from(*position)*scale + Vec3::new(0.0, 0.0, Layer::Interactable.into());
            transform.translation = in_screen_position; 

            if let Some(rotation) = orientation {
                transform.rotation = rotation.0;
            }
        });
    }
}