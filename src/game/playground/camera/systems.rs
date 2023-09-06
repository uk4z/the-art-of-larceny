use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::game::playground::components::{WorldPosition, Orientation};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::playground::security::components::{Intrusion, Security};
use super::components::Camera;

use super::components::*;
use std::f32::consts::PI;

pub const ROTATION_CORRECTION: f32 = PI/2.0;


pub fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(140.0, 3))).into(),
            transform: Transform::from_xyz(1500.0, 1000.0, 4.0),
            material: materials.add(ColorMaterial::from(Color::GREEN)), 
            ..default()
        },
        CameraBundle {
            position: CameraPosition {x: 1500.0, y: 1000.0},
            fov_position: WorldPosition {x:1500.0, y: 1000.0},
            orientation: Orientation(Quat::from_rotation_z(3.0*PI/2.0)*Quat::from_rotation_z(ROTATION_CORRECTION)),
            pattern: CameraPattern::Arc((PI/2.0, 0.0, Rotate::Trigo)),
            fov_length: FOVLength(140.0),
        }, 
        Camera
    ));
}

pub fn alert_security (
    cameras_q: Query<(&CameraPosition, &Orientation, &FOVLength), With<Camera>>, 
    mut player_q: Query<(&WorldPosition, &mut Stealth), With<Player>>,
    mut security_q: Query<&mut Intrusion, With<Security>>, 
){
    if let Ok((player_pos, mut stealth)) = player_q.get_single_mut() {
        for (camera_pos, orientation, length) in cameras_q.iter() {
            let target_vector = Vec3::from(*player_pos)-Vec3::new(camera_pos.x, camera_pos.y, 0.0);
            let fov_vector = orientation.0.mul_quat(Quat::from_rotation_z(-ROTATION_CORRECTION)).mul_vec3(Vec3::X*length.0);
            let angle = target_vector.angle_between(fov_vector);
            let player_distance = target_vector.length();
            let fov_distance = length.0*(1.0+1.0/2.0); //see length isocel triangle
            if angle < PI/4.0 && player_distance <= fov_distance {
                if let Ok(mut intrusion) = security_q.get_single_mut() {
                    intrusion.0 = true; 
                    *stealth = Stealth::Begineer;
                    println!("{:?}", player_pos);
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