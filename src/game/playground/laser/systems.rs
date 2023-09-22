use bevy::audio::{Volume, PlaybackMode, VolumeLevel};
use bevy::prelude::*; 
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

use super::{components::*, laser_extremum, get_extremum_values};
use crate::get_scale_reference;
use crate::components::Layer;
use crate::game::components::Level;
use crate::game::bundle::laser::get_laser_bundle;
use crate::game::playground::components::{WorldPosition, Orientation, GameOver};
use crate::game::playground::footage::components::{Available, Footage};
use crate::game::playground::player::components::Player;
use crate::game::playground::security::components::{Security, Intrusion, Active, Device};

pub fn spawn_laser(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
    window_q: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Some(lasers) = get_laser_bundle(&level) {
        for bundle in lasers {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(bundle.length.0, 4.0, 0.0))).into(),
                    transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into())
                            .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.0, 1.0, 0.0, 0.6))), 
                    ..default()
                },
                bundle, 
                Laser,
                AudioBundle {
                    source: asset_server.load("sounds/laser.ogg"),
                    settings: PlaybackSettings { 
                        mode: PlaybackMode::Once, 
                        volume: Volume::Relative(VolumeLevel::new(1.0)), 
                        speed: 1.0, paused: true }
                },
            ));
        }
    }
}

pub fn despawn_laser(
    mut commands: Commands,
    entity_q: Query<Entity, With<Laser>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}


pub fn burn_player(
    mut game_over: EventWriter<GameOver>,
    lasers_q: Query<(&WorldPosition, &Orientation, &LaserLength, &AudioSink), With<Laser>>, 
    mut player_q: Query<&WorldPosition, With<Player>>,
    mut intrusion_event: EventWriter<Intrusion>,
    active_q: Query<&Active, With<Security>>, 
    footage_q: Query<&Available, With<Footage>>, 
){
    if let Ok(player_pos) = player_q.get_single_mut() {
        for (laser_pos, orientation, length, sink) in lasers_q.iter() {
            let extremum = laser_extremum(laser_pos, orientation, length);
            
            let x_axis: Vec<f32> = extremum.iter().map(|point| {point.x}).collect();
            let y_axis: Vec<f32> = extremum.iter().map(|point| {point.y}).collect();

            let (min_x, max_x) = get_extremum_values(x_axis);
            let (min_y, max_y) = get_extremum_values( y_axis); 

            if player_pos.x >= min_x && player_pos.x <= max_x && player_pos.y >= min_y && player_pos.y <= max_y {
                if let Ok(active) = active_q.get_single() {
                    if let Ok(available) = footage_q.get_single() {
                        if active.0 && available.0 { 
                            intrusion_event.send(Intrusion(Device::Laser));
                            sink.play(); 
                            game_over.send(GameOver); 
                        }
                    }
                    
                }
            }
        }
    }
}