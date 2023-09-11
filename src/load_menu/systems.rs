use bevy::prelude::*;

use crate::components::Layer;
use crate::game::SimulationState;
use crate::load_menu::components::*;
use bevy_ui_borders::BorderColor;

use super::level_story::STORY;

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                simulation_state_next_state.set(SimulationState::Running);
                
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

pub fn despawn_load_menu(mut commands: Commands, load_menu_query: Query<Entity, With<LoadMenu>>) {
    if let Ok(load_menu_entity) = load_menu_query.get_single() {
        commands.entity(load_menu_entity).despawn_recursive();
    }
}

pub fn spawn_load_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn((
    NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
        visibility: Visibility::Visible, 
        background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
        ..default()
    },
    LoadMenu,
   )).with_children(|root|{
        root.spawn(
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                    ..default()
                },
                ..default()
        }).with_children(|title_section|{
            title_section.spawn( NodeBundle {
                style: Style {
                    display: Display::Flex, 
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    size: Size { width: Val::Percent(80.0), height: Val::Percent(80.0) },
                    justify_content: JustifyContent::SpaceBetween,
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..default()
            }).with_children(|commands|{
                for sentence in STORY.split("\n").into_iter() {
                    commands.spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceBetween,
                            flex_wrap: FlexWrap::Wrap, 
                            ..default()
                        },
                        ..default()
                    }).with_children(|commands| {
                        for word in sentence.split(" ") {
                            commands.spawn((
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
                        commands.spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.,
                                ..default()
                            },
                            ..default()
                        });
                    });
                }
                
            });
        });

        root.spawn((
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
                        size: Size::new(Val::Percent(30.0), Val::Px(100.0)),
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
                StartButton,
            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Start",
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
