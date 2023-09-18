use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::WorldPosition;
use super::player::components::{PlayerPace, Player};
use super::scenery::{
    components::Scenery,
    SCENERY_SIZE,
};

use super::CHARACTER_SIZE;

use crate::get_scale_reference;
use crate::components::Layer;
use super::components::Orientation;
pub fn confine_position(
    mut positions_q: Query<(&mut WorldPosition, &PlayerPace)>,
) {
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

pub fn world_to_screen(
    mut interactable_q: Query<(&mut Transform, &WorldPosition, Option<&Orientation>), (Without<Scenery>, Without<Player>)>,
    mut player_q: Query<(&mut Transform, &WorldPosition, &Orientation), With<Player>>,
    mut scenery_q: Query<&mut Transform, (With<Scenery>, Without<Player>)>,
    window_q: Query<&Window, With<PrimaryWindow>>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Ok(mut scenery_t) = scenery_q.get_single_mut() {
        if let Ok((mut player_transform, player_position, player_orientation)) = player_q.get_single_mut() {
            player_transform.rotation = player_orientation.0;

            let scenery_direction = (Vec3::new(SCENERY_SIZE.0/2.0, SCENERY_SIZE.1/2.0, Layer::Scenery.into()) - Vec3::from(*player_position))*scale_reference;
            scenery_t.translation = scenery_direction + player_transform.translation; 
            scenery_t.translation.z = Layer::Scenery.into(); 

            interactable_q.for_each_mut(|(mut transform, position, orientation)| {
                let direction = (Vec3::from(*position) - Vec3::from(*player_position))*scale_reference;
                transform.translation = player_transform.translation + direction; 
                transform.translation.z = Layer::Interactable.into(); 

                if let Some(rotation) = orientation {
                    transform.rotation = rotation.0;
                }
            });
        }
        
    }
}