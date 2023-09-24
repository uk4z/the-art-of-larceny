use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::audio::{PlaybackMode, VolumeLevel, Volume};

use super::components::{WorldPosition, Ambiance};
use super::player::components::{PlayerPace, Player};
use super::scenery::components::ScenerySize;
use super::scenery::components::Scenery;

use super::CHARACTER_SIZE;

use crate::game::components::Level;
use crate::get_scale_reference;
use crate::components::Layer;
use super::components::Orientation;


pub fn confine_position(
    mut positions_q: Query<(&mut WorldPosition, &PlayerPace)>,
    size_q: Query<&ScenerySize, With<Scenery>>, 
) {
    if let Ok(size) = size_q.get_single() {
        let height_limit = size.height-CHARACTER_SIZE/2.0; 
        let width_limit = size.width-CHARACTER_SIZE/2.0; 
    
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
    
}

pub fn world_to_screen(
    mut interactable_q: Query<(&mut Transform, &WorldPosition, Option<&Orientation>), (Without<Scenery>, Without<Player>)>,
    mut player_q: Query<(&mut Transform, &WorldPosition, &Orientation), With<Player>>,
    mut scenery_q: Query<(&mut Transform, &ScenerySize), (With<Scenery>, Without<Player>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Ok((mut scenery_t, scenery_size)) = scenery_q.get_single_mut() {
        if let Ok((mut player_transform, player_position, player_orientation)) = player_q.get_single_mut() {
            player_transform.rotation = player_orientation.0;

            let scenery_direction = (Vec3::new(scenery_size.width/2.0, scenery_size.height/2.0, Layer::Scenery.into()) - Vec3::from(*player_position))*scale_reference;
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


pub fn start_ambiance(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
) {
    let path = match *level {
        Level::Factory =>  {"sounds/ambiance/factory_music.ogg"}, 
        Level::Canyon => {"sounds/ambiance/canyon_music.ogg"},
        Level::Maze => {"sounds/ambiance/maze_music.ogg"},
        Level::MillerHouse => {"sounds/ambiance/millerhouse_music.ogg"},
        Level::Office => {"sounds/ambiance/office_music.ogg"},
        Level::Tutorial => {"sounds/ambiance/factory_music.ogg"}, 
        Level::Warehouse => {"sounds/ambiance/factory_music.ogg"}, 
    };

    commands.spawn((
        AudioBundle {
            source: asset_server.load(path),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop, 
                volume: Volume::Relative(VolumeLevel::new(0.05)), 
                speed: 1.0, paused: false}
        },
        Ambiance,
    ));    
}


pub fn despawn_ambiance(
    mut commands: Commands, 
    music_q: Query<Entity, With<Ambiance>>, 
) {
    if let Ok(entity) = music_q.get_single() {
        commands.entity(entity).despawn(); 
    } 
}

pub fn stop_ambiance(
    ambiance_q: Query<&AudioSink, With<Ambiance>>, 
) {
    for sink in ambiance_q.iter() {
        sink.stop();
    }
}