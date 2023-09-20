use bevy::audio::{PlaybackMode, VolumeLevel};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::audio::Volume; 


use crate::get_scale_reference;
use crate::components::Layer;
use crate::game::components::Level;
use crate::game::bundle::camera::get_camera_bundle;
use crate::game::playground::components::{WorldPosition, Orientation};
use crate::game::playground::footage::components::{Available, Footage};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::playground::security::components::{Intrusion, Security, Active, Device};
use super::components::Camera;

use super::components::*;
use std::f32::consts::PI;

pub const ROTATION_CORRECTION: f32 = PI/2.0;


pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>,
    window_q: Query<&Window, With<PrimaryWindow>>, 
    asset_server: Res<AssetServer>, 
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height()); 

    if let Some(cameras) = get_camera_bundle(&level) {
        for bundle in cameras {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(bundle.fov_length.0, 3))).into(),
                    transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into())
                            .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.0, 1.0, 0.0, 0.6))), 
                    ..default()
                },
                bundle, 
                Camera,
                AudioBundle {
                    source: asset_server.load("sounds/camera.ogg"),
                    settings: PlaybackSettings { 
                        mode: PlaybackMode::Once, 
                        volume: Volume::Relative(VolumeLevel::new(1.0)), 
                        speed: 1.0, paused: true }
                },
        
            ));
        }
    }
    
}

pub fn despawn_camera(
    mut commands: Commands,
    entity_q: Query<Entity, With<Camera>>,
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn alert_security (
    cameras_q: Query<(&CameraPosition, &Orientation, &FOVLength, &AudioSink), With<Camera>>, 
    mut player_q: Query<(&WorldPosition, &mut Stealth), With<Player>>,
    mut intrustion_event: EventWriter<Intrusion>, 
    active_q: Query<&Active, With<Security>>, 
    footage_q: Query<&Available, With<Footage>>, 
){
    if let Ok((player_pos, stealth)) = player_q.get_single_mut() {
        for (camera_pos, orientation, length, sink) in cameras_q.iter() {
            let target_vector = Vec3::from(*player_pos)-Vec3::new(camera_pos.x, camera_pos.y, 0.0);
            let fov_vector = orientation.0.mul_quat(Quat::from_rotation_z(-ROTATION_CORRECTION)).mul_vec3(Vec3::X*length.0);
            let angle = target_vector.angle_between(fov_vector);
            let player_distance = target_vector.length();
            let fov_distance = length.0*(1.0+1.0/2.0); //see length isocel triangle
            if angle < PI/4.0 && player_distance <= fov_distance {
                if let Ok(active) = active_q.get_single() {
                    if let Ok(available) = footage_q.get_single() {
                        if active.0 && available.0 {
                            match *stealth {
                                Stealth::Ghost => {
                                    intrustion_event.send(Intrusion(Device::Camera));
                                    sink.play(); 
                                }
                                _ => {}
                            }
                        }
                    }
                    
                }
                
            }
        }
    }
}

pub fn update_fov_position( 
    mut cameras_q: Query<(&CameraPosition, &Orientation, &FOVLength, &mut WorldPosition), With<Camera>>, 
) {
    cameras_q.for_each_mut(|(camera_pos, orientation, length, mut fov_pos)| {
        let origin = Vec3::new(camera_pos.x, camera_pos.y, 0.0);
        let fov_position = origin + orientation.0.mul_quat(Quat::from_rotation_z(-ROTATION_CORRECTION)).mul_vec3(Vec3::X*length.0);
        *fov_pos = WorldPosition {x: fov_position.x, y:fov_position.y};
    });
}

pub fn rotate_cameras(
    mut cameras_q: Query<(&mut Orientation, &mut CameraPattern), With<Camera>>
) {
    cameras_q.for_each_mut(|(mut orientation, mut pattern)| {
        match *pattern {
            CameraPattern::Arc((magnitude, rotate_angle, rotate), ) => {
                match rotate {
                    Rotate::Trigo => {
                        if rotate_angle >= magnitude {
                            *pattern = CameraPattern::Arc((magnitude, magnitude, Rotate::AntiTrigo));
                        }
                        else {
                            orientation.0 = orientation.0.mul_quat(Quat::from_rotation_z(0.01));
                            *pattern = CameraPattern::Arc((magnitude, rotate_angle+0.01, Rotate::Trigo)); 
                        }
                    },
                    Rotate::AntiTrigo => {
                        if rotate_angle <= 0.0 {
                            *pattern = CameraPattern::Arc((magnitude, 0.0, Rotate::Trigo)); 
                        }
                        else {
                            orientation.0 = orientation.0.mul_quat(Quat::from_rotation_z(-0.01));
                            *pattern = CameraPattern::Arc((magnitude, rotate_angle-0.01, Rotate::AntiTrigo)); 
                        }
                    },
                }
            }
            _ => {
            
            }
        }
    });
}