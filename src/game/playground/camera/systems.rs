use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::game::playground::components::{WorldPosition, Orientation};
use crate::game::playground::player::components::Player;
use crate::game::playground::security::components::{Intrusion, Security};
use crate::game::playground::orientate_angle;
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
            mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(130.0, 3))).into(),
            transform: Transform::from_xyz(1500.0, 1000.0, 4.0),
            material: materials.add(ColorMaterial::from(Color::GREEN)), 
            ..default()
        },
        CameraBundle {
            position: CameraPosition {x: 1500.0, y: 1000.0},
            fov_position: WorldPosition {x:1500.0, y: 1000.0},
            orientation: Orientation(Quat::from_rotation_arc(Vec3::X, Vec3::X)*Quat::from_rotation_z(ROTATION_CORRECTION)),
            pattern: CameraPattern::Static,
            fov_length: FOVLength(130.0),
        }, 
        Camera
    ));
}

pub fn alert_security (
    cameras_q: Query<(&CameraPosition, &Orientation, &FOVLength), With<Camera>>, 
    player_q: Query<&WorldPosition, With<Player>>,
    mut security_q: Query<&mut Intrusion, With<Security>>, 
){
    if let Ok(player_pos) = player_q.get_single() {
        for (camera_pos, orientation, length) in cameras_q.iter() {
            let target_vector = Vec3::from(*player_pos)-Vec3::new(camera_pos.x, camera_pos.y, 0.0);
            let fov_vector = orientation.0.mul_quat(Quat::from_rotation_z(-ROTATION_CORRECTION)).mul_vec3(Vec3::X*length.0);
            let angle = orientate_angle(target_vector.angle_between(fov_vector));
            let distance = target_vector.length();
            if angle < PI/4.0 && length.0 >= distance {
                if let Ok(mut intrusion) = security_q.get_single_mut() {
                    intrusion.0 = true; 
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