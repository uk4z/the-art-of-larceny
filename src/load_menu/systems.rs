use std::time::Instant;

use bevy::prelude::*;
use crate::AppState;
use crate::components::Layer;
use crate::game::components::{SimulationState, ItemCount, GameTime};
use crate::load_menu::components::*;
use bevy_ui_borders::BorderColor;

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>, 
    mut count: ResMut<ItemCount>, 
    mut time: ResMut<GameTime>, 
) {
    if let Ok((interaction, mut color, mut border)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                match state.0 {
                    AppState::MainMenu => {
                        count.0 = 0; 
                        time.0 = Instant::now(); 
                        app_state_next_state.set(AppState::Game);
                    }
                    _ => {}
                }
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

pub fn despawn_load_menu(
    mut commands: Commands, 
    load_menu_query: Query<Entity, With<LoadMenu>>,
) {
    if let Ok(load_menu_entity) = load_menu_query.get_single() {
        commands.entity(load_menu_entity).despawn_recursive();
    }
}

pub fn spawn_load_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    println!("spawn load menu");
    
    commands.spawn((
    NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgba(0.18, 0.20, 0.25, 1.0).into(),
        transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
        visibility: Visibility::Visible, 
        ..default()
    },
    LoadMenu,
   )).with_children(|root|{
        root.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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