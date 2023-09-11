use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};
use bevy::a11y::{
    accesskit::{NodeBuilder, Role},
    AccessibilityNode,
};
use bevy_ui_borders::BorderColor;
use rand::prelude::*;

use super::components::*;
use crate::components::Layer;
use crate::game::playground::item::components::Item;
use crate::game::playground::player::components::{Stealth, Player};
use crate::game::playground::scenery::get_scenery_scale_from_window;
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
                position_type: PositionType::Relative,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
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
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    visibility: Visibility::Visible,
                    ..default()
                }).with_children(|text_division|{
                    text_division.spawn((TextBundle {
                        text: Text::from_section(
                            "",
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
        });

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
            transform: Transform::from_xyz(0.0, 0.0, Layer::UI.into()),
            ..default()
        },
        BoardUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    background_color: Color::rgba(0.18, 0.20, 0.25, 0.8).into(),
                    ..default()
                },
                BorderColor(Color::WHITE),
                IntelMenu,
                IntelBundle {
                    section: Section::Instruction,
                },
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
                                position_type: PositionType::Absolute,
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
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
                            IntelLabel
                        ));
                
                    });

                    menu.spawn(
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            ..default()
                        },
                    ).with_children(|node|{
                        node.spawn((NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                            },
                            Vault,
                        )).with_children(|content|{
                            content.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        position_type: PositionType::Absolute, 
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        ..default()
                                    },
                                    visibility: Visibility::Inherited, 
                                    ..default()
                                },
                                Password,
                            )).with_children(|text_section| {
                                text_section.spawn((
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
                                    PasswordText,
                                ));
                            });
                            
                            content.spawn((
                                NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        position_type: PositionType::Absolute,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        align_items: AlignItems::Center,
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        ..default()
                                    },
                                    visibility: Visibility::Hidden, 
                                    ..default()
                                },
                                VaultContent, 
                            )).with_children(|values_section| {
                                let names = vec!["euros", "dollars", "yens"];
                                for i in 0..3 {
                                    let mut rng = thread_rng();
                                    let amount = rng.gen_range(100..200);
                                    let name = names[i];
                                    values_section.spawn((
                                        //button
                                        ButtonBundle { 
                                            style: Style {
                                                display: Display::Flex,
                                                flex_direction: FlexDirection::Column,
                                                size: Size::new(Val::Percent(30.0), Val::Percent(20.0)),
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
                                        button.spawn((
                                            TextBundle::from_section(
                                                format!("{}", amount),
                                            TextStyle {
                                                font: asset_server.load("FiraMono-Medium.ttf"),
                                                font_size: 20.0,
                                                color: Color::BLACK.into()
                                            }),
                                            Amount,
                                        ));
                                        button.spawn((
                                            TextBundle::from_section(
                                                name,
                                            TextStyle {
                                                font: asset_server.load("FiraMono-Medium.ttf"),
                                                font_size: 20.0,
                                                color: Color::BLACK.into()
                                                }),
                                            Currency,
                                        ));
                                    });
                                }
                            });
                        });
                    });
                        

                    menu.spawn(
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            visibility: Visibility::Inherited,
                            ..default()
                        },
                    ).with_children(|node|{
                        node.spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                                    ..default()
                                },
                                visibility: Visibility::Inherited,
                                ..default()
                            },
                            Instruction,
                        )).with_children(|instruction|{

                        
                            instruction.spawn(
                                NodeBundle {
                                    style: Style {
                                        flex_grow: 1.0,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center, 
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                            }).with_children(|instruction_list| {
                                instruction_list.spawn((
                                    NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,    
                                        size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                                        overflow: Overflow::Hidden,
                                        ..default()
                                    },
                                    ..default()},
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                )).with_children(|scroll_list| {
                                    let instructions = vec![
                                        "Find all the items.",
                                        "Unlock the target.",
                                        "Stole a currency.",
                                        "Escape through the extraction point.",
                                    ];
                                    for i in 0..instructions.len() { 
                                        scroll_list.spawn((
                                            TextBundle {
                                                text: Text::from_section(
                                                instructions[i],
                                                TextStyle {
                                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::WHITE,
                                                }),
                                                style: Style {
                                                    flex_grow: 1.0,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem))
                                        ));
                                    }
                                    scroll_list.spawn((
                                        TextBundle {
                                            text: Text::from_section(
                                            "",
                                            TextStyle {
                                                font: asset_server.load("FiraMono-Medium.ttf"),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                            }),
                                            style: Style {
                                                flex_grow: 1.0,
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        StealthStatus,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem))
                                    ));
                                });            
                            });
                        });
                    });

                    menu.spawn(
                        NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                position_type: PositionType::Absolute,
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::End,
                                align_items: AlignItems::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            visibility: Visibility::Inherited,
                            ..default()
                        },
                    ).with_children(|node|{
                        node.spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                                    ..default()
                                },
                                visibility: Visibility::Hidden,
                                ..default()
                            },
                            ItemContent,
                        )).with_children(|instruction|{
                            instruction.spawn(
                                NodeBundle {
                                    style: Style {
                                        flex_grow: 1.0,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center, 
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    ..default()
                            }).with_children(|instruction_list| {
                                instruction_list.spawn((
                                    NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,    
                                        size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                                        overflow: Overflow::Hidden,
                                        ..default()
                                    },
                                    ..default()},
                                    AccessibilityNode(NodeBuilder::new(Role::List)),
                                )).with_children(|scroll_list| {
                                    let number_items = 2; 
                                    for _ in 0..number_items { 
                                        scroll_list.spawn((
                                            TextBundle {
                                                text: Text::from_section(
                                                "",
                                                TextStyle {
                                                    font: asset_server.load("FiraMono-Medium.ttf"),
                                                    font_size: 24.0,
                                                    color: Color::WHITE,
                                                }),
                                                style: Style {
                                                    flex_grow: 1.0,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            ItemBoard,
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem))
                                        ));
                                    }
                                });            
                            });
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
    for board in board_q.iter() {
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

pub fn resize_intel_menu(
    mut style_q: Query<&mut Style, With<IntelMenu>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    let scale = get_scenery_scale_from_window(&window.width(), &window.height());

    let new_size = 
        if window.width() <= window.height() {
            Size::new(Val::Percent(90.0), Val::Percent(scale*80.0))
        } else {
            Size::new(Val::Percent(scale*90.0), Val::Percent(80.0))
        };

    if let Ok(mut style) = style_q.get_single_mut() {
        *style = Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: new_size,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        };
    }
}

pub fn handle_intel_visibility(
    mut intel_q: Query<&mut Visibility, With<IntelMenu>>, 
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(mut visibility) = intel_q.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::V) {
            match *visibility {
                Visibility::Hidden => {
                    *visibility = Visibility::Visible;
                }, 
                Visibility::Visible => {
                    *visibility = Visibility::Hidden;
                }, 
                _ => {},
            }
        }
    }
}

pub fn display_intel_label (
    section_q: Query<&Section, With<IntelMenu>>,
    mut text_q: Query<&mut Text, With<IntelLabel>>, 
) {
    if let Ok(section) = section_q.get_single() {
        if let Ok(mut text) = text_q.get_single_mut() {
            match *section {
                Section::Instruction => {
                    text.sections[0].value = "< Instruction >".to_string();
                },
                Section::Item => {
                    text.sections[0].value = "< Item >".to_string();
                },
                Section::Vault => {
                    text.sections[0].value = "< Vault >".to_string();
                }
            }
        }
    }
}

pub fn switch_section (
    mut section_q: Query<(&mut Section, &Visibility), With<IntelMenu>>,
    mut vault_q: Query<&mut Visibility, (With<Vault>, Without<Section>)>,
    mut item_q: Query<&mut Visibility,(With<ItemContent>, Without<Vault>, Without<IntelMenu>, Without<Instruction>)>,
    mut instruction_q: Query<&mut Visibility, (With<Instruction>, Without<Vault>, Without<IntelMenu>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut section, visibility)) = section_q.get_single_mut() {
        if let Ok(mut vault_vis) = vault_q.get_single_mut() {
            if let Ok(mut instruction_vis) = instruction_q.get_single_mut() {
                if let Ok(mut item_vis) = item_q.get_single_mut() {
                    match *visibility {
                        Visibility::Visible => {
                            if keyboard_input.just_pressed(KeyCode::Left) {
                                match *section {
                                    Section::Instruction => {
                                        *section = Section::Item;
                                        *vault_vis = Visibility::Hidden; 
                                        *instruction_vis = Visibility::Hidden;
                                        *item_vis = Visibility::Inherited;
                                    },
                                    Section::Item => {
                                        *section = Section::Vault;
                                        *vault_vis = Visibility::Inherited; 
                                        *instruction_vis = Visibility::Hidden;
                                        *item_vis = Visibility::Hidden;
                                    },
                                    Section::Vault => {
                                        *section = Section::Instruction;
                                        *vault_vis = Visibility::Hidden; 
                                        *instruction_vis = Visibility::Inherited;
                                        *item_vis = Visibility::Hidden;
                                    }
                                }
                            }

                            if keyboard_input.just_pressed(KeyCode::Right) {
                                match *section {
                                    Section::Instruction => {
                                        *section = Section::Vault;
                                        *vault_vis = Visibility::Inherited; 
                                        *instruction_vis = Visibility::Hidden;
                                        *item_vis = Visibility::Hidden;
                                    },
                                    Section::Item => {
                                        *section = Section::Instruction;
                                        *vault_vis = Visibility::Hidden; 
                                        *instruction_vis = Visibility::Inherited;
                                        *item_vis = Visibility::Hidden;
                                    },
                                    Section::Vault => {
                                        *section = Section::Item;
                                        *vault_vis = Visibility::Hidden; 
                                        *instruction_vis = Visibility::Hidden;
                                        *item_vis = Visibility::Inherited;
                                    }
                                }
                            }
                        }
                        _=> {},
                    }
                }
            }
        }
    }
}

pub fn unlock_animation (
    mut password_q: Query<&mut Text, (With<PasswordText>, Without<IntelMenu>)>,
    timer_q: Query<&UnlockTimer, With<Target>>,

) {
    if let Ok(mut text) = password_q.get_single_mut() {
        if let Ok(timer) = timer_q.get_single() {
            let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
            let mut rng = thread_rng();

            let number = (timer.0.percent_left()*100.0/20.0) as usize; 

            let mut random_password = "".to_string();
            for _ in 0..5 {
                let index: usize = rng.gen_range(0..data.len()-1);
                random_password.push_str(&data[index..index+1]);
            }
            let correct_password = "clear".to_string();

            if number <= 0 {
                text.sections[0].value = correct_password; 
            } else if number >= 5 {
                text.sections[0].value = random_password; 
            } else {
                text.sections[0].value = format!{"{}{}", &random_password[..number], &correct_password[number..]};
            }
        }
    } 
}


pub fn unlock_target( 
    timer_q: Query<&UnlockTimer, With<Target>>,
    mut vault_q: Query<&mut Visibility, (With<VaultContent>, Without<Password>)>, 
    mut password_q: Query<&mut Visibility, (With<Password>, Without<VaultContent>)>, 
) {
    if let Ok(timer) = timer_q.get_single() {
        if timer.0.finished() {
            if let Ok(mut vault_vis) = vault_q.get_single_mut() {
                if let Ok(mut password_vis) = password_q.get_single_mut() {
                    *vault_vis = Visibility::Inherited; 
                    *password_vis = Visibility::Hidden;
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
                if !currency_locked.0 {
                    border_color.0 = Color::GREEN;
                    currency_locked.0 = true; 
                }
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
        let data = "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".to_string();
        let mut rng = thread_rng();
        let mut random_text = "".to_string();
        for _ in 0..4 {
            let index: usize = rng.gen_range(0..data.len()-1);
            random_text.push_str(&data[index..index+1]);
        }
        let item = match visibility {
            Visibility::Hidden => {format!("{}: {}", name.0, value.0)},
            _ => {format!("{}: {}", name.0, random_text)},
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

pub fn display_stealth(
    stealth_q: Query<&Stealth, With<Player>>,
    mut status_q: Query<&mut Text, With<StealthStatus>>,
) {
    if let Ok(stealth) = stealth_q.get_single() {
        let value = match *stealth {
            Stealth::Ghost => {
                "You should stay clear from security.".to_string()
            },
            Stealth::Engineer => {
                "You should avoid guards.".to_string()
            }, 
            Stealth::Begineer => {
                "You should suppress the footage.".to_string()
            },
            Stealth::None => {
                "You should not get caught.".to_string()
            }
        };

        if let Ok(mut text) = status_q.get_single_mut() {
            text.sections[0].value = value ;
        }
    }
}