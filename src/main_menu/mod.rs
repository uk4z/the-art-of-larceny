pub mod components;
pub mod systems;

use bevy_ui_borders::BordersPlugin;
use systems::*;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool; 

use std::fs::File;
use std::io::Write;


use crate::{AppState, game::components::{Level, GameTime, ScoreEvent, SimulationState}};

use self::components::Best;

pub const MAIN_SIZE: (f32, f32) = (3260.0, 2240.0);
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(BordersPlugin)
            .insert_resource(Level::Starting)
            .register_type::<Best>()
            .add_startup_system(load_score_level)
            .add_system(save_scene_system.in_schedule(OnEnter(SimulationState::Score)))
            // OnEnter State Systems
            .add_systems(
                (
                    spawn_main_menu,
                    spawn_level_menu,
                    spawn_main_image,
                ).in_schedule(OnEnter(AppState::MainMenu)))
            // Systems
            .add_systems(
                (
                    interact_with_play_button, 
                    interact_with_quit_button, 
                    interact_with_select_button, 
                    //update_level_image_on_resize, 
                    switch_level, 
                    display_level_title,
                    update_level_image,
                    update_main_image_on_resize,
                    update_main_menu_on_resize,
                    get_loaded_score, 
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_systems(
                (   
                    despawn_main_image,
                    despawn_level_image, 
                    despawn_main_menu,
                    despawn_level_menu,
                ).in_schedule(OnExit(AppState::MainMenu)));
    }
}

pub fn get_main_scale_from_window(
    window_w: &f32, 
    window_h: &f32,
) -> f32 {
    let w_scale = window_w/MAIN_SIZE.0;
    let h_scale = window_h/MAIN_SIZE.1;
    if w_scale > h_scale {
        h_scale
    } else {
        w_scale
    }
}

// The initial scene file will be loaded below and not change when the scene is saved
const SCORE_FILE_PATH: &str = "scores/load_factory_score.scn.ron";

pub fn save_scene_system(
    type_registry_resource: Res<AppTypeRegistry>, 
    time: Res<GameTime>, 
    mut score_event: EventReader<ScoreEvent>,
    best_q: Query<&Best>, 
) {
    let mut best_score = 0; 
    let mut best_time = 0; 
    if let Ok(best) = best_q.get_single() {
        best_score = best.score; 
        best_time = best.time; 
    };

    let mut new_score = 0; 
    for ev in score_event.iter() {
        new_score = ev.value; 
    }

    // Scenes can be created from any ECS World. You can either create a new one for the scene or
    // use the current World.
    let mut scene_world = World::new();
    if best_score > new_score {
        scene_world.spawn(Best { time: best_time, score: best_score});
    } else if best_score < new_score {
        scene_world.spawn(Best { time: time.0.elapsed().as_secs(), score: new_score});
    } else {
        if time.0.elapsed().as_secs() > best_time {
            scene_world.spawn(Best { time: best_time, score: best_score});
        } else {
            scene_world.spawn(Best { time: time.0.elapsed().as_secs(), score: new_score});
        }
    }
    

    
    let type_registry = &type_registry_resource.to_owned(); 
    // The TypeRegistry resource contains information about all registered types (including
    // components). This is used to construct scenes.
    let scene = DynamicScene::from_world(&scene_world, type_registry);

    // Scenes can be serialized like this:
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{SCORE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}