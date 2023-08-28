use bevy::prelude::*;
use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};
use bevy_ui_borders::BorderColor;

use super::components::*;
use crate::components::Layer;
use crate::game::playground::item::components::Item;
use crate::game::playground::target::components::{UnlockTimer, Target};
use crate::game::playground::components;


pub fn spawn_board (
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        //root node
        .spawn((NodeBundle {
            style: Style {
                display: Display::Flex,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Start,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
            ..default()
        },
        BoardUI
        ))
        .with_children(|parent| {
            //helper
            parent.spawn( NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::End,
                    flex_grow: 1.0,
                    ..default()
                },
                ..default()
            }).with_children(|helper|{
                helper.spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|text_division|{
                    text_division.spawn((TextBundle {
                        text: Text::from_section(
                            "This will be the helper section",
                            TextStyle {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            }),
                            background_color: Color::BLACK.into(),
                            ..default()
                    },
                    Helper,
                    ));
                });
            });

            //board
            parent.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::MIDNIGHT_BLUE.into(),
                ..default()
            }).with_children(|board| {
                
                //---------------------------------------------------------------------
                //---------------------------------------------------------------------
                //instruction
                //---------------------------------------------------------------------
                //---------------------------------------------------------------------

                board.spawn( NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(2.0)),
                        size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                }).with_children(|instruction| {
                    //instruction title
                    instruction.spawn(TextBundle {
                        text: Text::from_section(
                        "Instructions",
                        TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        }),
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    });
                    //instruction list
                    instruction.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            ..default()
                        },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    }).with_children(|instruction_list| {
                        instruction_list.spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,    
                                flex_grow: 1.0,
                                size: Size::new(Val::Percent(100.0), Val::Px(150.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            ..default()},
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        )).with_children(|scroll_list| {
                            for i in 0..10 {
                                scroll_list.spawn((
                                    TextBundle::from_section(
                                        format!("Item {i}"),
                                        TextStyle {
                                            font: asset_server
                                                .load("FiraMono-Medium.ttf"),
                                            font_size: 16.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    Label,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));
                            }
                            scroll_list.spawn((TextBundle {
                                text: Text::from_section(
                                "- Find all the items.",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                }),
                                style: Style {
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                ..default()
                                },
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                            scroll_list.spawn((TextBundle {
                                text: Text::from_section(
                                "- Hack the computer.",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                }),
                                style: Style {
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                ..default()
                                },
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                            scroll_list.spawn((TextBundle {
                                text: Text::from_section(
                                "- Stole a currency.",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                }),
                                style: Style {
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                ..default()
                                },
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                            scroll_list.spawn((TextBundle {
                                text: Text::from_section(
                                "- Escape through the extraction point.",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 16.0,
                                    color: Color::WHITE,
                                }),
                                style: Style {
                                    flex_grow: 1.0,
                                    ..default()
                                },
                                ..default()
                                },
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                        });
                        
                    }); 
                });
                
                //---------------------------------------------------------------------
                //---------------------------------------------------------------------
                //item
                //---------------------------------------------------------------------
                //---------------------------------------------------------------------
                board.spawn( NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                }).with_children(|item| {
                    //item title
                    item.spawn(TextBundle {
                        text: Text::from_section(
                        "Items",
                        TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        }),
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    });
                    //item list
                    item.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            ..default()
                        },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    }).with_children(|item_list| {
                        item_list.spawn((NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,    
                                flex_grow: 1.0,
                                size: Size::new(Val::Percent(100.0), Val::Px(150.0)),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            ..default()},
                            AccessibilityNode(NodeBuilder::new(Role::List)),
                        )).with_children(|scroll_list| {
                            for i in 0..6 {
                                scroll_list.spawn((
                                    TextBundle::from_section(
                                        format!("Item {i}"),
                                        TextStyle {
                                            font: asset_server
                                                .load("FiraMono-Medium.ttf"),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ItemBoard,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ));
                            }
                        });
                        
                    }); 
                });
                        
                //---------------------------------------------------------------------
                //---------------------------------------------------------------------
                //target
                //---------------------------------------------------------------------
                //---------------------------------------------------------------------

                board.spawn(
                        NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            border: UiRect::all(Val::Px(2.0)),
                            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    }).with_children(|target|{
                    //target title
                    target.spawn(TextBundle {
                        text: Text::from_section(
                        "Target",
                        TextStyle {
                            font: asset_server.load("FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        }),
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..default()
                    });
                    //button section
                    target.spawn((
                        NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            background_color: Color::MIDNIGHT_BLUE.into(),
                            ..default()
                        },
                        LockedButton,
                    )).with_children(|button_section|{
                        button_section.spawn((
                            NodeBundle { 
                                    style: Style {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                                        /* border: UiRect::all(Val::Px(5.0)), */
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        position_type: PositionType::Absolute,
                                        border: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                            }, 
                            Button,
                            BorderColor(Color::BLACK),
                        )).with_children(|button| {
                            button.spawn((
                                NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(0.0), Val::Percent(100.0)),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::SEA_GREEN.into(),
                                    ..default()
                                },
                                LoadingBar,
                            ));
                            button.spawn(
                                TextBundle::from_section(
                                "Locked",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                        });
                    });

                    target.spawn((
                        NodeBundle { 
                            style: Style {
                                flex_grow: 1.0,
                                display: Display::None,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                ..default()
                            },
                            background_color: Color::MIDNIGHT_BLUE.into(),
                            ..default()
                            },
                            OpenTarget,
                    )).with_children(|values_section|{
                        values_section.spawn((
                            //button
                            ButtonBundle { 
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UnlockedButton,
                            BorderColor(Color::BLACK),
                        )).with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                "1000",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                            button.spawn(
                                TextBundle::from_section(
                                "euros",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                        });
                        values_section.spawn((
                            //button
                            ButtonBundle { 
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UnlockedButton,
                            BorderColor(Color::BLACK),
                        )).with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                "2500",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                            button.spawn(
                                TextBundle::from_section(
                                "dollars",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                        });
                        values_section.spawn((
                            //button
                            ButtonBundle { 
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    size: Size::new(Val::Percent(50.0), Val::Percent(20.0)),
                                    border: UiRect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UnlockedButton, 
                            BorderColor(Color::BLACK),
                        )).with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                "6500",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                            button.spawn(
                                TextBundle::from_section(
                                "yens",
                                TextStyle {
                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::BLACK.into()
                                    }
                            ));
                        });
                    });
                });
            });
        });
}

pub fn despawn_board(
    mut commands: Commands, 
    board_q: Query<Entity, With<BoardUI>>,
) {
    if let Ok(board) = board_q.get_single() {
        commands.entity(board).despawn_recursive();
    };
    
}

pub fn clean_helper (
    mut helper_q: Query<&mut Text, With<Helper>>,
) {
    //This function is called at the begining of each frame to clean the helper section of any given text. 
    //Therefore, the BoardPlugin has to be added before the PlaygroundPlugin in the LevelPlugin.
    if let Ok(mut text) = helper_q.get_single_mut() {
        text.sections[0].value = "".to_string();
    }
}


pub fn lock_animation (
    mut button_q: Query<&mut Style, With<LoadingBar>>,
    timer_q: Query<&UnlockTimer, With<Target>>,
) {
    if let Ok(mut loading_bar) = button_q.get_single_mut() {
        if let Ok(timer) = timer_q.get_single() {
            let elapsed_time = timer.0.elapsed().as_millis() as f32;
            let total_time = timer.0.duration().as_millis() as f32;
            loading_bar.size = Size::new(Val::Percent(100.0*elapsed_time/total_time), Val::Percent(100.0))
        }
    }
}

pub fn unlock_target (
    mut button_q: Query<&mut Style, (With<LockedButton>, Without<OpenTarget>)>,
    timer_q: Query<&UnlockTimer, With<Target>>,
    mut target_q: Query<&mut Style, (With<OpenTarget>, Without<LockedButton>)>,
) {
    if let Ok(mut button) = button_q.get_single_mut() {
        if let Ok(mut open_target) = target_q.get_single_mut() {
            if let Ok(timer) = timer_q.get_single() {
                if timer.0.finished() {
                    button.display = Display::None; 
                    open_target.display = Display::Flex;
                }
            }
        }
       
    } 
}

pub fn button_system(
    mut button_q: Query<(&mut BorderColor, &Interaction),  With<UnlockedButton>>,
    mut currency_locked: ResMut<CurrencyLocked>, 
) {
    button_q.for_each_mut(|(mut border_color, interaction) | {
        match interaction {
            Interaction::Clicked => {
                border_color.0 = Color::GREEN;
                currency_locked.0 = true; 
            }
            Interaction::Hovered => {
                if !currency_locked.0 {
                    border_color.0 = Color::RED;
                }
            }
            Interaction::None => {
                if !currency_locked.0 {
                    border_color.0 = Color::BLACK;
                }
            }
        }
        
        
    });
}

pub fn show_item_found(
    item_q: Query<(&components::Name, &components::Value, &Visibility), With<Item>>, 
    mut board_item_q: Query<&mut Text, With<ItemBoard>>,
) {
    let mut items: Vec<String> = item_q.iter().map(|(name, value, visibility)| {
        let item = match visibility {
            Visibility::Hidden => {format!("{}: {}", name.0, value.0)},
            _ => {format!("{}: Not found", name.0)},
        };
        return item
    }).collect();

    board_item_q.for_each_mut(|mut text| {
        text.sections[0].value = match items.pop(){
            Some(item) => {item},
            None => {"".to_string()},
        };
    })
}