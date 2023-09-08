use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};


use super::components::{Scenery, BoundsEvent, SceneryBundle, Bounds};
use super::{get_scenery_position_from_window, get_scenery_scale_from_window};
use crate::components::Layer;



pub fn spawn_scenery(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let (x, y) = get_scenery_position_from_window(&window.width(), &window.height());
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    commands.spawn(
        (SpriteBundle{
            texture: asset_server.load("levels/backgrounds/test.png"),
            transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(scale, scale, Layer::Scenery.into())),
        ..default()
        },
        Scenery,
        SceneryBundle {
            bounds: Bounds(Vec::new())
        },
    ));
}

pub fn bounds_event(
    mut ev: EventWriter<BoundsEvent>,
    asset_server: Res<AssetServer>,
) {
    let asset: Handle<Image> = asset_server.load("levels/backgrounds/bounds.png");
    ev.send(BoundsEvent{handle: asset});
}


pub fn set_bounds(
    assets: Res<Assets<Image>>,
    mut ev: EventReader<BoundsEvent>,
    mut bounds_q: Query<&mut Bounds, With<Scenery>>,
) {
    for bounds_ev in ev.iter() {
        if let Some(img) = assets.get(&bounds_ev.handle) {
            let height = img.texture_descriptor.size.height as usize;
            let width = img.texture_descriptor.size.width as usize;

            if let Ok(mut bounds) = bounds_q.get_single_mut() {

                let mut pixel_rgba = Vec::new(); 
                for i in (0..img.data.len() as usize).step_by(4) {
                    let r = img.data[i];
                    let g = img.data[i + 1];
                    let b = img.data[i + 2];
                    let a = img.data[i + 3];
                    if i == 0 || i == 4 || i == 8 || i == 17920 || i == 17928 || i == 35840 {
                        println!("{:?}", (r, g, b, a));
                    }
                    pixel_rgba.push((r, g, b, a));
                }

                let mut update_bounds = Vec::new();
                for i in 0..height {
                    let row: Vec<i32> = pixel_rgba[i*width..(i+1)*width].iter().map(|(r,g,b,_)| {
                        if *r == 0 && *g == 0 && *b == 0 {
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
    }
}

