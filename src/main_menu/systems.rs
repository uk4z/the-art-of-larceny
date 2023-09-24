use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use bevy::audio::{PlaybackMode, VolumeLevel, Volume};

use crate::get_scale_reference;
use crate::components::Layer;
use crate::game::components::{SimulationState, Level};
use crate::game::playground::scenery::components::BoundsResource;
use crate::main_menu::components::*;
use super::get_main_scale_from_window;


pub fn spawn_music(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/tension.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop, 
                volume: Volume::Relative(VolumeLevel::new(0.06)), 
                speed: 1.0, paused: false}
        },
        BackgroundMusic,
    ));    
}

pub fn despawn_music(
    mut commands: Commands, 
    music_q: Query<Entity, With<BackgroundMusic>>, 
) {
    if let Ok(entity) = music_q.get_single() {
        commands.entity(entity).despawn(); 
    } 
}


pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut level_q: Query<&mut Visibility, With<LevelMenu>>,
    mut main_q: Query<&mut Visibility, (Without<LevelMenu>, With<MainMenu>)>,
    level: ResMut<Level>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    window_q: Query<&Window, With<PrimaryWindow>>, 
    main_image_q: Query<Entity, With<MainImage>>, 
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Ok(mut level_visibility) = level_q.get_single_mut() {
                    if let Ok(mut main_visibility) = main_q.get_single_mut() {
                        *level_visibility = Visibility::Visible;
                        *main_visibility = Visibility::Hidden;

                        if let Ok(image) = main_image_q.get_single() {
                            commands.entity(image).despawn(); 
                        }

                        let image_path = match *level {
                            Level::Factory => {
                                "levels/backgrounds/factory.png"
                            }, 
                            Level::Tutorial => {
                                "levels/backgrounds/tutorial.png"
                            },
                            Level::Warehouse => {
                                "levels/backgrounds/warehouse.png"
                            },
                            Level::MillerHouse => {
                                "levels/backgrounds/millerhouse.png"
                            },
                            Level::Maze => {
                                "levels/backgrounds/maze.png"
                            },
                            Level::Office => {
                                "levels/backgrounds/office.png"
                            },
                            Level::Canyon => {
                                "levels/backgrounds/canyon.png"
                            },
                        };
                        
                        let window = window_q.get_single().unwrap(); 
                        let (width, height) = (window.width(), window.height()); 
    
                        commands.spawn((
                            SpriteBundle{
                                texture: asset_server.load(image_path),
                                transform: Transform::from_xyz(width/2.0, height/2.0, Layer::LevelImage.into()),
                            ..default()
                            },
                            LevelImage, 
                        ));

                        commands.spawn(
                            AudioBundle {
                                source: asset_server.load("sounds/confirmation.ogg"),
                                settings: PlaybackSettings {
                                    mode: PlaybackMode::Despawn, 
                                    volume: Volume::Relative(VolumeLevel::new(0.2)), 
                                    speed: 1.0, paused: false}
                            }
                        );
                    }
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

pub fn interact_with_select_button(
    asset_server: Res<AssetServer>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SelectButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut bounds_resource: ResMut<BoundsResource>,  
    level: ResMut<Level>,
    mut commands: Commands,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                let asset_path = match *level {
                    Level::Factory => {"levels/bounds/factory_bounds.png"}, 
                    Level::Tutorial => {"levels/bounds/tutorial_bounds.png"},
                    Level::Warehouse => {"levels/bounds/warehouse_bounds.png"},
                    Level::MillerHouse => {"levels/bounds/millerhouse_bounds.png"},
                    Level::Maze => {"levels/bounds/maze_bounds.png"},
                    Level::Office => {"levels/bounds/office_bounds.png"},
                    Level::Canyon => {"levels/bounds/canyon_bounds.png"},
                };
                
                let asset: Handle<Image> = asset_server.load(asset_path);
                bounds_resource.handle = Some(asset); 
                
                simulation_state_next_state.set(SimulationState::Loading);

                commands.spawn(
                    AudioBundle {
                        source: asset_server.load("sounds/confirmation.ogg"),
                        settings: PlaybackSettings {
                            mode: PlaybackMode::Despawn, 
                            volume: Volume::Relative(VolumeLevel::new(0.2)), 
                            speed: 1.0, paused: false}
                    }
                );

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

pub fn switch_level(
    mut level: ResMut<Level>,
    keyboard_input: Res<Input<KeyCode>>, 
    level_q: Query<&Visibility, With<LevelMenu>>,
) {

    if let Ok(visibility) = level_q.get_single() {
        match visibility {
            Visibility::Visible => {
                if keyboard_input.just_pressed(KeyCode::Right) {
                    match *level {
                        Level::Tutorial => {
                            *level = Level::Maze
                        },
                        Level::Maze => {
                            *level = Level::MillerHouse;
                        },
                        Level::MillerHouse => {
                            *level = Level::Factory;
                        },
                        Level::Factory => {
                            *level = Level::Warehouse;
                        },
                        Level::Warehouse => {
                            *level = Level::Office;
                        },
                        Level::Office => {
                            *level = Level::Canyon;
                        },      
                        Level::Canyon => {
                            *level = Level::Tutorial;
                        },  
                    }
                }
                if keyboard_input.just_pressed(KeyCode::Left) {
                    match *level {
                        Level::Canyon => {
                            *level = Level::Office;
                        }, 
                        Level::Office => {
                            *level = Level::Warehouse;
                        },
                        Level::Warehouse => {
                            *level = Level::Factory
                        },
                        Level::Factory => {
                            *level = Level::MillerHouse;
                        }, 
                        Level::MillerHouse => {
                            *level = Level::Maze;
                        },
                        Level::Maze => {
                            *level = Level::Tutorial;
                        },
                        Level::Tutorial => {
                            *level = Level::Canyon; 
                        },
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn update_level_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    image_q: Query<Entity, With<LevelImage>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    level: Res<Level>,
) {
    if let Ok(image) = image_q.get_single() {
        if let Ok(window) = window_q.get_single() {
            commands.entity(image).despawn(); 

            let (width, height) = (window.width(), window.height());
            let scale = get_main_scale_from_window(&width, &height); 

            let image_path = match *level {
                Level::Factory => {
                    "levels/backgrounds/factory.png"
                }, 
                Level::Tutorial => {
                    "levels/backgrounds/tutorial.png"
                },
                Level::Warehouse => {
                    "levels/backgrounds/warehouse.png"
                },
                Level::MillerHouse => {
                    "levels/backgrounds/millerhouse.png"
                },
                Level::Maze => {
                    "levels/backgrounds/maze.png"
                },
                Level::Office => {
                    "levels/backgrounds/office.png"
                },
                Level::Canyon => {
                    "levels/backgrounds/canyon.png"
                },
            };

            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load(image_path),
                    transform: Transform::from_xyz(width/2.0, height/2.0, Layer::LevelImage.into())
                    .with_scale(Vec3::new(scale, scale, 1.0)),
                ..default()
                },
                LevelImage, 
            ));
        }
    }
}

pub fn spawn_main_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap(); 
    let scale = 0.7; 
    let scale_reference = get_scale_reference(&window.width(), &window.height());

    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("menu_image.png"),
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, Layer::Scenery.into())
                .with_scale(Vec3::new(scale*scale_reference, scale*scale_reference, 1.0)),
        ..default()
        },
        MainImage, 
    ));
}

pub fn despawn_level_image(
    mut commands: Commands, 
    entity_q: Query<Entity, With<LevelImage>>, 
) {
    for entity in entity_q.iter() {
        commands.entity(entity).despawn(); 
    }
}

pub fn display_level_title (
    level: Res<Level>, 
    mut text_q: Query<&mut Text, With<LevelLabel>>, 
) {
    if let Ok(mut text) = text_q.get_single_mut() {
        match *level {
            Level::Factory => {
                text.sections[0].value = "< Factory >".to_string();
            },
            Level::Tutorial => {
                text.sections[0].value = "< Tutorial >".to_string(); 
            },
            Level::Warehouse => {
                text.sections[0].value = "< Warehouse >".to_string(); 
            },
            Level::MillerHouse => {
                text.sections[0].value = "< Mr. Miller House >".to_string(); 
            },
            Level::Maze => {
                text.sections[0].value = "< Maze >".to_string(); 
            },
            Level::Office => {
                text.sections[0].value = "< Office >".to_string(); 
            },
            Level::Canyon => {
                text.sections[0].value = "< Canyon >".to_string(); 
            },
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                app_exit_event_writer.send(AppExit);
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

pub fn despawn_main_menu(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenu>>,
    
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    
}

pub fn despawn_main_image(
    main_image_query: Query<Entity, With<MainImage>>,
    mut commands: Commands, 
) {
    if let Ok(main_image_entity) = main_image_query.get_single() {
        commands.entity(main_image_entity).despawn_recursive();
    }
}

pub fn spawn_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap(); 
    let scale_reference = get_scale_reference(&window.width(), &window.height());

    commands.spawn((
    NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into())
                .with_scale(Vec3::new(scale_reference, scale_reference, 1.0)),
        visibility: Visibility::Visible, 
        ..default()
     },
     MainMenu,
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
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
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
                        width: Val::Percent(30.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.4).into(),
                    border_color: Color::WHITE.into(), 
                    ..default()
                },
                PlayButton,
            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "New Game",
                    TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE.into()
                        }),
                ));
            });

            values_section.spawn((
                //button
                ButtonBundle { 
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(30.0),
                        height: Val::Px(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.4).into(),
                    border_color: Color::WHITE.into(), 
                    ..default()
                },
                QuitButton,

            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Quit",
                    TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE.into()
                        }),
                ));
            });
        });
    });
}

pub fn despawn_level_menu(
    mut commands: Commands, 
    level_menu_query: Query<Entity, With<LevelMenu>>,
) {
    if let Ok(level_menu_entity) = level_menu_query.get_single() {
        commands.entity(level_menu_entity).despawn_recursive();
    }
}


pub fn spawn_level_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands
        //root node
        .spawn((NodeBundle {
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
            ..default()
        },
        LevelMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.4).into(),
                    border_color: Color::WHITE.into(), 
                    ..default()
                },
            ))
            .with_children(|intel|{
                intel.spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::rgba(0.18, 0.20, 0.25, 0.8).into(),
                        ..default()
                    },

                )).with_children(|menu|{
                    menu.spawn((
                        //label
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(10.0),    
                                ..default()
                            },
                            ..default()
                        },
                    )).with_children(|label_section|{
                        label_section.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("FiraMono-Medium.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    }),
                                    ..default()
                            },
                            LevelLabel
                        )); 
                    });

                    menu.spawn(
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Start,
                                margin: UiRect { 
                                    left: Val::Percent(10.0), 
                                    right: Val::Auto, 
                                    top: Val::Auto, 
                                    bottom: Val::Auto },
                                    width: Val::Percent(90.0),
                                    height: Val::Percent(70.0),
                                ..default()
                            },
                            ..default()
                        }
                    ).with_children(|node| {
                        node.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("FiraMono-Medium.ttf"),
                                        font_size: 25.0,
                                        color: Color::WHITE,
                                    }),
                                    ..default()
                            },
                            BestScore
                        ));

                        node.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("FiraMono-Medium.ttf"),
                                        font_size: 25.0,
                                        color: Color::WHITE,
                                    }),
                                    ..default()
                            },
                            BestTime
                        ));
                    });

                    menu.spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Percent(20.0),        
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
                                    width: Val::Percent(30.0),
                                    height: Val::Px(50.0),
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
                            SelectButton,
                        )).with_children(|button| {
                            button.spawn((
                                TextBundle::from_section(
                                    "Select",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE.into()
                                    }),
                            ));
                        });
                    });
                });
            });
        });

}

pub fn update_level_image_on_resize(
    mut resize_event: EventReader<WindowResized>, 
    mut image_q: Query<&mut Transform, With<LevelImage>>,
) {
    for resized_window in resize_event.iter() {
        if let Ok(mut transform) = image_q.get_single_mut() {
            let new_width = resized_window.width;
            let new_height = resized_window.height;
            let new_scale = get_main_scale_from_window(&new_width, &new_height);
            let (new_x, new_y) = (new_width/2.0, new_height/2.0);

            let window_position = Vec3::new(new_x, new_y, Layer::UI.into());

            transform.translation = window_position; 
            transform.scale = Vec3::new(new_scale, new_scale, 1.0);

        }
    }
}



pub fn update_main_image_on_resize(
    mut resize_event: EventReader<WindowResized>, 
    mut image_q: Query<&mut Transform, With<MainImage>>,
) {
    for resized_window in resize_event.iter() {
        if let Ok(mut transform) = image_q.get_single_mut() {
            let new_width = resized_window.width;
            let new_height = resized_window.height;
            let (new_x, new_y) = (new_width/2.0, new_height/2.0);

            let window_position = Vec3::new(new_x, new_y, Layer::UI.into());

            let scale_reference = get_scale_reference(&resized_window.width, &resized_window.height);
            let scale = 0.7; 
            

            transform.translation = window_position; 
            transform.scale = Vec3::new(scale*scale_reference, scale*scale_reference, 1.0)

        }
    }
}

pub fn update_main_menu_on_resize(
    mut menu_q: Query<&mut Transform, With<MainMenu>>,  
    mut resize_event: EventReader<WindowResized>, 
) {
    for resized_window in resize_event.iter() {
        let scale_reference = get_scale_reference(&resized_window.width, &resized_window.height);

        if let Ok(mut transform) = menu_q.get_single_mut() {
            transform.scale = Vec3::new(scale_reference, scale_reference, 1.0);
        }
    }
}