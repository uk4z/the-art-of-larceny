use bevy::prelude::*; 
use bevy::sprite::MaterialMesh2dBundle;

use super::{components::*, laser_extremum, get_extremum_values};
use super::components::Direction;
use crate::game::playground::components::{WorldPosition, Orientation};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::playground::security::components::{Security, Intrusion};

pub fn spawn_laser(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(200.0, 4.0, 0.0))).into(),
            transform: Transform::from_xyz(800.0, 400.0, 4.0),
            material: materials.add(ColorMaterial::from(Color::GREEN)), 
            ..default()
        },
        LaserBundle {
            position: WorldPosition {x: 800.0, y: 400.0},
            orientation: Orientation(Quat::from_rotation_z(Direction::Horizontal.into())), //angle is 0.0 or PI/2.0 ; 
            length: LaserLength(200.0)
        }, 
        Laser
    ));
}


pub fn alert_security (
    lasers_q: Query<(&WorldPosition, &Orientation, &LaserLength), With<Laser>>, 
    mut player_q: Query<(&WorldPosition, &mut Stealth), With<Player>>,
    mut security_q: Query<&mut Intrusion, With<Security>>, 
){
    if let Ok((player_pos, mut stealth)) = player_q.get_single_mut() {
        for (laser_pos, orientation, length) in lasers_q.iter() {
            let extremum = laser_extremum(laser_pos, orientation, length);
            
            let x_axis: Vec<f32> = extremum.iter().map(|point| {point.x}).collect();
            let y_axis: Vec<f32> = extremum.iter().map(|point| {point.y}).collect();

            let (min_x, max_x) = get_extremum_values(x_axis);
            let (min_y, max_y) = get_extremum_values( y_axis); 

            if player_pos.x >= min_x && player_pos.x <= max_x && player_pos.y >= min_y && player_pos.y <= max_y {
                if let Ok(mut intrusion) = security_q.get_single_mut() {
                    intrusion.0 = true; 
                    *stealth = Stealth::Begineer;
                    println!("{:?}", player_pos);
                }
            }
        }
    }
}