use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};


use super::components::*;
use super::get_scenery_position_from_window;
use crate::components::Layer;
use crate::game::components::Level;
use crate::game::bundle::scenery::get_scenery_bundle;



pub fn spawn_scenery(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    level: Res<Level>,
) {
    let window = window_q.get_single().unwrap();
    let (x, y) = get_scenery_position_from_window(&window.width(), &window.height());

    if let Some(bundle) = get_scenery_bundle(&level) {
        commands.spawn(
            (SpriteBundle{
                texture: asset_server.load(bundle.path.0.clone()),
                transform: Transform::from_xyz(x, y, Layer::Interactable.into()),
            ..default()
            },
            Scenery,
            bundle,
        ));
    }
}

pub fn despawn_scenery(
    mut commands: Commands,
    entity_q: Query<Entity, With<Scenery>>, 
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn set_bounds(
    resource: Res<BoundsResource>,
    assets: ResMut<Assets<Image>>,
    mut bounds_q: Query<&mut Bounds, With<Scenery>>,
) { 
    if let Some(img) = assets.get(&resource.handle) {
        let height = img.texture_descriptor.size.height as usize;
        let width = img.texture_descriptor.size.width as usize;

        if let Ok(mut bounds) = bounds_q.get_single_mut() {
            if bounds.0.len() <= 0 {
                let mut pixel_rgba = Vec::new(); 
                for i in (0..img.data.len() as usize).step_by(4) {
                    let r = img.data[i];
                    let g = img.data[i + 1];
                    let b = img.data[i + 2];
                    let a = img.data[i + 3];
                    pixel_rgba.push((r, g, b, a));
                }
    
                let mut update_bounds = Vec::new();
                for i in 0..height {
                    let row: Vec<i32> = pixel_rgba[i*width..(i+1)*width].iter().map(|(r,g,b,_)| {
                        if *r == 255 && *g == 255 && *b == 255 {
                            0
                        } else {
                            1
                        }
                    }).collect();
                    update_bounds.push(row);
                }
                *bounds = Bounds(update_bounds); 
            }
        };
    } else {
    };
}

