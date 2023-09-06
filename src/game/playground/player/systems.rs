use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};
use bevy::utils::Duration;


use super::components::*;
use super::{get_player_direction, DISTANCE_PER_SECOND};

use crate::components::Layer;
use crate::game::playground::components::{WorldPosition, Orientation, AnimatedMotion, ReachDistance};
use crate::game::playground::scenery::get_scenery_scale_from_window;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    commands.spawn(
        (SpriteBundle{
            texture: asset_server.load("player/static.png"),
            transform: Transform::from_xyz(0.0, 0.0, Layer::Interactable.into()).with_scale(Vec3::new(scale, scale, 1.0)),       
        ..default()
        }, 
        PlayerBundle { 
            position: WorldPosition {x: 1000.0, y: 1000.0,},
            orientation: Orientation(Quat::IDENTITY),
            pace: PlayerPace::Walk,
            animation: AnimatedMotion {
                walk_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                run_timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
            },
            reach: ReachDistance(20.0),
            stealth: Stealth::Ghost,
        },
        Player, 
    ));
}

pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<(&mut WorldPosition, &mut Orientation, &PlayerPace), With<Player>>,

) {
    if let Ok((mut position, mut orientation, pace)) = player_q.get_single_mut() { 
        let direction = get_player_direction(keyboard_input);

        //update position
        let speed = match pace {
            PlayerPace::Run => {
                <PlayerPace as Into<f32>>::into(PlayerPace::Run)*DISTANCE_PER_SECOND*time.delta_seconds()
            },
            PlayerPace::Walk => {
                <PlayerPace as Into<f32>>::into(PlayerPace::Walk)*DISTANCE_PER_SECOND*time.delta_seconds()
            },
        };
        let translation: Vec3 = direction*speed;
        position.x += translation.x; 
        position.y += translation.y;

        //update orientation
        if direction.length() > 0.0 {
            orientation.0 = Quat::from_rotation_arc(Vec3::X, direction);
        }
        
    };
}


pub fn set_player_pace(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<&mut PlayerPace, With<Player>>,
) {
    if let Ok(mut player_pace) = player_q.get_single_mut() {
        let pace = player_pace.as_mut();
        if keyboard_input.pressed(KeyCode::R) {
            *pace = PlayerPace::Run;
        } else {
            *pace = PlayerPace::Walk;
        }
    }
}

pub fn motion_handler(
    mut player_q: Query<(&mut Handle<Image> ,&mut AnimatedMotion, &mut Transform, &PlayerPace), With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if let Ok((mut texture, mut animated, mut transform, pace)) 
            = player_q.get_single_mut() {

        if keyboard_input.any_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Right, KeyCode::Left]) {
            *texture = asset_server.load("player/motion.png");
            match pace {
                PlayerPace::Run => {
                    animated.run_timer.tick(time.delta());
                    if animated.run_timer.finished() {
                        transform.scale.y = -transform.scale.y;
                    }
                },
                PlayerPace::Walk => {
                    animated.walk_timer.tick(time.delta());
                    if animated.walk_timer.finished() {
                        transform.scale.y = -transform.scale.y;
                    }
                },
            }
        } else {
            *texture = asset_server.load("player/static.png")
        }
    }
}
