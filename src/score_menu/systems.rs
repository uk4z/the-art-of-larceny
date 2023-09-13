use bevy::prelude::*;
use crate::components::Layer;
use crate::game::components::{SimulationState, ScoreEvent};
use crate::score_menu::components::*;
use crate::AppState;
use bevy_ui_borders::BorderColor;

pub fn interact_with_restart_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<RestartButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                app_state_next_state.set(AppState::MainMenu);
                simulation_state_next_state.set(SimulationState::None);
                
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

pub fn interact_with_leave_button(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<LeaveButton>),
    >,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                app_state_next_state.set(AppState::MainMenu);
                simulation_state_next_state.set(SimulationState::None);
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
    asset_server: Res<AssetServer>
) {
    commands.spawn((
    NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
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
                    size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
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
                        size: Size::new(Val::Percent(80.0), Val::Px(100.0)),
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
                RestartButton,
            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Restart",
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
                        size: Size::new(Val::Percent(80.0), Val::Px(100.0)),
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
                    size: Size { width: Val::Percent(40.0), height: Val::Percent(60.0) },
                    justify_content: JustifyContent::Start,
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                background_color: Color::rgba(0.18, 0.20, 0.25, 0.8).into(),
                ..default()
            },
            BorderColor(Color::WHITE),
        )).with_children(|node|{
            node.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex, 
                    size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
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
                    size: Size::new(Val::Percent(90.0), Val::Percent(70.0)),
                    justify_content: JustifyContent::SpaceBetween, 
                    align_items: AlignItems::Center,
                    padding: UiRect::new(Val::Px(20.0), Val::Undefined, Val::Undefined, Val::Undefined),
                    ..default()
                },
                ..default()
            }).with_children(|category|{
                category.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex, 
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
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
                });

                category.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex, 
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        justify_content: JustifyContent::Start, 
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|title_section| {
                    title_section.spawn(
                        TextBundle::from_section(
                            "Items found:",
                            TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                    );
                });

                category.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex, 
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
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
                });
            });
            
        });
    });
}

pub fn handle_score_event( 
    mut commands: Commands,
    mut score_event: EventReader<ScoreEvent>,
    mut entity_q: Query<Entity, With<Comment>>,
    asset_server: Res<AssetServer>
) {
    for score in score_event.iter() {
        if let Ok(entity) = entity_q.get_single_mut() {
            commands.entity(entity).with_children(|node|{
                for word in score.comment.split(" ") {
                    node.spawn((
                        TextBundle::from_section(
                            word.to_string(),
                            TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_text_alignment(TextAlignment::Center)
                        .with_style(Style {
                            // this is required becouse of the bevy bug
                            // https://github.com/bevyengine/bevy/issues/5834
                            max_size: Size::new(Val::Undefined, Val::Px(18.0)),
        
                            // this is the size of the spaces between words
                            margin: UiRect::all(Val::Px(3.0)),
                            ..default()
                        }),
                    ));
                }
                node.spawn(NodeBundle {
                    style: Style {
                        flex_grow: 1.,
                        ..default()
                    },
                    ..default()
                });
            });
        }
    }
}
