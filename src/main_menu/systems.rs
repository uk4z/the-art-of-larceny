use bevy::app::AppExit;
use bevy::prelude::*;

use crate::components::Layer;
use crate::game::SimulationState;
use crate::main_menu::components::*;
use crate::AppState;
use bevy_ui_borders::BorderColor;

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                app_state_next_state.set(AppState::Game);
                simulation_state_next_state.set(SimulationState::Paused);
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

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn spawn_main_menu(
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
    MainMenu,
   )).with_children(|root|{
        root.spawn(
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
        });

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
                    background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
                    ..default()
                },
                BorderColor(Color::WHITE),
                PlayButton,
            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Play",
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
                    background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
                    ..default()
                },
                BorderColor(Color::WHITE),
                LevelButton,

            )).with_children(|button| {
                button.spawn((
                    TextBundle::from_section(
                        "Level",
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
                    background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
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
