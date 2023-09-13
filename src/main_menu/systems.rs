use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::components::Layer;
use crate::game::components::{SimulationState, Level};
use crate::game::playground::scenery::get_scenery_scale_from_window;
use crate::main_menu::components::*;
use bevy_ui_borders::BorderColor;

use super::get_main_scale_from_window;

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut level_q: Query<&mut Visibility, With<LevelMenu>>,
    mut main_q: Query<&mut Visibility, (Without<LevelMenu>, With<MainMenu>)>,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                if let Ok(mut level_visibility) = level_q.get_single_mut() {
                    if let Ok(mut main_visibility) = main_q.get_single_mut() {
                        *level_visibility = Visibility::Visible;
                        *main_visibility = Visibility::Hidden;
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
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<SelectButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                simulation_state_next_state.set(SimulationState::Loading);
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
                        Level::Starting => {
                            *level = Level::Mock;
                        },
                        Level::Mock => {
                            *level = Level::Starting;
                        }
                    }
                }
                if keyboard_input.just_pressed(KeyCode::Left) {
                    match *level {
                        Level::Starting => {
                            *level = Level::Mock;
                        },
                        Level::Mock => {
                            *level = Level::Starting;
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

pub fn update_main_image(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    main_q: Query<Entity, With<MainImage>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    level: Res<Level>,
) {
    if let Ok(main) = main_q.get_single() {
        if let Ok(window) = window_q.get_single() {
            commands.entity(main).despawn();

            let (width, height) = (window.width(), window.height());
            let scale = get_scenery_scale_from_window(&width, &height);

            let image_path = match *level {
                Level::Starting => {
                    "levels/backgrounds/test.png"
                }, 
                Level::Mock => {
                    "test.png"
                }
            };


            commands.spawn((
                SpriteBundle{
                    texture: asset_server.load(image_path),
                    transform: Transform::from_xyz(width/2.0, height/2.0, Layer::Scenery.into()).with_scale(Vec3::new(scale, scale, 1.0)),
                ..default()
                },
                MainImage, 
            ));
        }
    }
}

pub fn display_level_title (
    level: Res<Level>, 
    mut text_q: Query<&mut Text, With<LevelLabel>>, 
) {
    if let Ok(mut text) = text_q.get_single_mut() {
        match *level {
            Level::Starting => {
                text.sections[0].value = "< Starting >".to_string();
            },
            Level::Mock => {
                text.sections[0].value = "< Mock >".to_string();
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
            Interaction::Clicked => {
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

pub fn clear_main_image(
    mut level: ResMut<Level>, 
) {
    *level = Level::Mock; 
}

pub fn spawn_main_image(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let (x, y) = (window.width()/2.0, window.height()/2.0);
    let scale = get_main_scale_from_window(&window.width(), &window.height());

    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("test.png"),
            transform: Transform::from_xyz(x, y, Layer::UI.into()).with_scale(Vec3::new(scale, scale, 1.0)),
            visibility: Visibility::Visible,
        ..default()
        },
        MainImage, 
    ));
}

pub fn spawn_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    

    commands.spawn((
    NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
        visibility: Visibility::Visible, 
        ..default()
     },
     MainMenu,
     )).with_children(|root|{
        /* root.spawn(
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    padding: UiRect::new(Val::Percent(5.0),
                                                Val::Px(0.0),
                                                Val::Px(0.0),
                                                Val::Px(0.0),),
                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                    ..default()
                },
                ..default()
        }).with_children(|title_section|{
                title_section.spawn((
                    TextBundle::from_section(
                        "The Art of Larceny: Rogue's Riches",
                    TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 80.0,
                        color: Color::WHITE.into()
                        }),
                ));
        }); */

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
                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
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
                        size: Size::new(Val::Percent(30.0), Val::Px(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.4).into(),
                    ..default()
                },
                BorderColor(Color::WHITE),
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
                        size: Size::new(Val::Percent(30.0), Val::Px(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.4).into(),
                    ..default()
                },
                BorderColor(Color::WHITE),
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
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                        size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.4).into(),
                    ..default()
                },
                BorderColor(Color::WHITE),
            ))
            .with_children(|intel|{
                intel.spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
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
                                    right: Val::Undefined, 
                                    top: Val::Undefined, 
                                    bottom: Val::Undefined },
                                size: Size::new(Val::Percent(90.0), Val::Percent(70.0)),
                                ..default()
                            },
                            ..default()
                        }
                    ).with_children(|node| {
                        node.spawn(
                            TextBundle {
                                text: Text::from_section(
                                    "Score:",
                                    TextStyle {
                                        font: asset_server.load("FiraMono-Medium.ttf"),
                                        font_size: 25.0,
                                        color: Color::WHITE,
                                    }),
                                    ..default()
                            },
                        );

                        node.spawn(
                            TextBundle {
                                text: Text::from_section(
                                    "Time:",
                                    TextStyle {
                                        font: asset_server.load("FiraMono-Medium.ttf"),
                                        font_size: 25.0,
                                        color: Color::WHITE,
                                    }),
                                    ..default()
                            },
                        );
                    });

                    menu.spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
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
                                    size: Size::new(Val::Percent(30.0), Val::Px(50.0)),
                                    border: UiRect::all(Val::Px(2.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
                                ..default()
                            },
                            BorderColor(Color::WHITE),
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

pub fn update_main_image_on_resize(
    mut resize_event: EventReader<WindowResized>, 
    mut image_q: Query<&mut Transform, With<MainImage>>,
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