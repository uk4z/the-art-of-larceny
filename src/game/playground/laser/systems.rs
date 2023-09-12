use bevy::prelude::*; 
use bevy::sprite::MaterialMesh2dBundle;

use super::{components::*, laser_extremum, get_extremum_values};
use crate::game::components::Level;
use crate::game::bundle::laser::get_laser_bundle;
use crate::game::playground::components::{WorldPosition, Orientation};
use crate::game::playground::player::components::{Player, Stealth};
use crate::game::playground::security::components::{Security, Intrusion};

pub fn spawn_laser(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    level: Res<Level>
) {
    if let Some(lasers) = get_laser_bundle(&level.name) {
        for bundle in lasers {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(146.0, 4.0, 0.0))).into(),
                    transform: Transform::from_xyz(800.0, 400.0, 4.0),
                    material: materials.add(ColorMaterial::from(Color::rgba(0.0, 1.0, 0.0, 0.6))), 
                    ..default()
                },
                bundle, 
                Laser
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