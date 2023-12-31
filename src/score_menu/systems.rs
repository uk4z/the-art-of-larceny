use bevy::window::PrimaryWindow;
use rand::prelude::*;
use bevy::audio::{PlaybackMode, VolumeLevel, Volume};

use bevy::prelude::*;
use crate::components::Layer;
use crate::game::components::{ItemCount, GameTime, ScoreEvent};
use crate::game::playground::guard::components::Guard;
use crate::game::playground::player::components::{Stealth, Player};
use crate::score_menu::components::*;
use crate::{AppState, get_scale_reference};


pub fn enter_score_menu_sound(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/score.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn, 
                volume: Volume::Relative(VolumeLevel::new(0.8)), 
                speed: 1.0, paused: false}
        },
        SoundScore, 
    ));
}


pub fn interact_with_leave_button(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<LeaveButton>),
    >,
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    sink_q: Query<&AudioSink, With<SoundScore>>, 
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                app_state_next_state.set(AppState::MainMenu);

                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("sounds/confirmation.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn, 
                            volume: Volume::Relative(VolumeLevel::new(0.2)), 
                            speed: 1.0, paused: false}
                    }
                );

                if let Ok(sink) = sink_q.get_single() {
                    sink.stop()
                }
            }
            Interaction::Hovered => {
                border.0 = Color::WHITE;
                color.0 = Color::RED.into();
            }
            Interaction::None => {
                border.0 = Color::WHITE;
                color.0 = Color::rgba(0.18, 0.20, 0.25, 0.8).into();
            }
        }
    }
}

pub fn despawn_score_menu(mut commands: Commands, score_menu_query: Query<Entity, With<ScoreMenu>>) {
    if let Ok(score_menu_entity) = score_menu_query.get_single() {
        commands.entity(score_menu_entity).despawn_recursive();
    }
}

pub fn spawn_score_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    time: Res<GameTime>, 
    mut score_event: EventReader<ScoreEvent>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let elapsed_time = format!(" {}:{}:{}", time.0.elapsed().as_secs()/3600, time.0.elapsed().as_secs()/60, (time.0.elapsed().as_secs()%3600)%60);

    let mut total_score = "You lost !".to_string();

    let mut value = 0; 
    for ev in score_event.iter() {
        total_score = ev.comment.clone(); 
        value = ev.value; 
    }
    
    let window = window_q.get_single().unwrap(); 
    let reference_scale = get_scale_reference(&window.width(), &window.height());

    commands.spawn((
    NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into())
                .with_scale(Vec3::new(reference_scale, reference_scale, 1.0)),
        visibility: Visibility::Visible, 
        //background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
        ..default()
    },
    ScoreMenu,
   )).with_children(|root|{
        root.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Start,
                    padding: UiRect::new(Val::Percent(5.0),
                                                Val::Px(0.0),
                                                Val::Px(0.0),
                                                Val::Px(0.0),),
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            }, 
        )).with_children(|values_section| {
            values_section.spawn((
                //button
                ButtonBundle { 
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(80.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
                    border_color: Color::WHITE.into(), 
                    ..default()
                },
                LeaveButton,

            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Leave",
                    TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE.into()
                        }),
                ));
            });
        });

        root.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex, 
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    width: Val::Percent(40.0),
                    height: Val::Percent(60.0),
                    justify_content: JustifyContent::Start,
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: Color::rgba(0.18, 0.20, 0.25, 0.8).into(),
                border_color: Color::WHITE.into(), 
                ..default()
            },
        )).with_children(|node|{
            node.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex, 
                    width: Val::Percent(100.0),
                    height: Val::Percent(30.0),
                    justify_content: JustifyContent::Center, 
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }).with_children(|title_section| {
                title_section.spawn(
                    TextBundle::from_section(
                        "Score",
                        TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 80.0,
                            color: Color::WHITE,
                        },
                    )
                );
            });

            node.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex, 
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(90.0),
                    height: Val::Percent(70.0),
                    justify_content: JustifyContent::SpaceBetween, 
                    align_items: AlignItems::Center,
                    padding: UiRect::new(Val::Px(20.0), Val::Auto, Val::Auto, Val::Auto),
                    ..default()
                },
                ..default()
            }).with_children(|node|{
                node.spawn(
                    NodeBundle {
                        style: Style {
                            display: Display::Flex, 
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceEvenly, 
                            width: Val::Percent(100.0),
                            height: Val::Percent(70.0),
                            ..default()
                        },
                        ..default()
                    }
                ).with_children(|category|{
                    for _ in 0..5 {
                        category.spawn((
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("FiraMono-Medium.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                            ),
                            SubScores,
                        ));
                    }
                });
                
                node.spawn(
                    NodeBundle {
                        style: Style {
                            display: Display::Flex, 
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceEvenly, 
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    }
                ).with_children(|category|{
                    category.spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex, 
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Start, 
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    }).with_children(|title_section| {
                        title_section.spawn(
                            TextBundle::from_section(
                                "Time:",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                        );

                        title_section.spawn((
                            TextBundle::from_section(
                                elapsed_time.clone(),
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TimeBoard, 
                            ElapsedTime(time.0.elapsed().as_secs())
                        ));
                    });

                    category.spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex, 
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::Start, 
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    }).with_children(|title_section| {
                        title_section.spawn(
                            TextBundle::from_section(
                                "Total:",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                        );

                        title_section.spawn((
                            TextBundle::from_section(
                                total_score.clone(),
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TotalScore,
                            ScoreValue(value), 
                        ));
                    });
                });
            });
            
        });
    });
}

pub fn reset_score_timer(
    mut score_timer: ResMut<ScoreTimer>,
) {
    score_timer.timer.reset(); 
}

pub fn score_animation(
    mut score_timer: ResMut<ScoreTimer>, 
    mut score_q: Query<&mut Text, With<SubScores>>,
    time: Res<Time>, 
    count: Res<ItemCount>, 
    guard_q: Query<Entity, With<Guard>>,
    player_q: Query<&Stealth, With<Player>>,
    elapsed_time_q: Query<&ElapsedTime, With<TimeBoard>>,
) {
    let nb_guards = guard_q.iter().len();

    let mut time_score = 0; 

    if let Ok(elapsed_time) = elapsed_time_q.get_single() {
        time_score = 50000*(3600-elapsed_time.0) /3600;
    }
    

    let mut stealth_message = "Unknown stealth"; 
    if let Ok(stealth) = player_q.get_single() {
        stealth_message = match *stealth {
            Stealth::None => {"You have been seen by guards!"},
            Stealth::Begineer => {"You have been spotted by the security system!"},
            Stealth::Engineer => {"You have suppressed the footages!"},
            Stealth::Ghost => {"You have not been seen!"}
        };
    } 
    

    let scores = vec![
        format!("Gems collected: + {} x 25 000", count.0), 
        format!("Guards non alerted: + {} x 50 000", nb_guards), 
        format!("Target cleared: + 100 000"),
        format!("Time bonus: + {}", time_score),
        format!("Stealth: {}", stealth_message),
    ]; 

    for (index, mut text) in score_q.iter_mut().enumerate() {
        score_timer.timer.tick(time.delta());

        let score = scores.get(index).unwrap_or(&format!("Unknown")).clone(); 

        if !score_timer.timer.finished() {

            let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
            let mut rng = thread_rng();
    
            let number = (score_timer.timer.percent()*(score.len() as f32)) as usize;

            let mut random_score = "".to_string();
            for _ in 0..score.len()-1 {
                let index: usize = rng.gen_range(0..data.len()-1);
                random_score.push_str(&data[index..index+1]);
            }
    
            if number <= 0 {
                text.sections[0].value = score; 
            } else if number >= score.len() {
                text.sections[0].value = random_score; 
            } else {
                text.sections[0].value = format!("{}{}", &score[..number], &random_score[number..]);
            }
        } else {
            text.sections[0].value = format!("{}", score);
        }
    } 
}   