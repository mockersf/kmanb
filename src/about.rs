use bevy::prelude::*;
use tracing::info;

const CURRENT_SCREEN: crate::Screen = crate::Screen::About;

struct ScreenTag;

struct Screen {
    loaded: bool,
}
impl Default for Screen {
    fn default() -> Self {
        Screen { loaded: false }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Screen::default())
            .add_system(mouse_input_system.system())
            .add_system(setup.system())
            .add_system(keyboard_input_system.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

fn setup(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut screen: ResMut<Screen>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_handles: ResMut<crate::AssetHandles>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let color_none = materials.add(Color::NONE.into());

        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);

        let font_sub: Handle<Font> = asset_handles.get_font_sub_handle(&asset_server);

        let board_assets = asset_handles.get_board_handles(&asset_server, materials);

        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(20.),
                        right: Val::Undefined,
                        bottom: Val::Undefined,
                        top: Val::Percent(25.),
                    },
                    size: Size::<Val> {
                        height: Val::Px(190.),
                        width: Val::Auto,
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: color_none,
                ..Default::default()
            })
            .with_children(|title_parent| {
                title_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(150. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Keep Moving".to_string(),
                        font,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 150.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
                title_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(40. / 2.),
                            ..Default::default()
                        },
                        margin: Rect {
                            right: Val::Px(20. / 2.),
                            ..Default::default()
                        },
                        align_self: AlignSelf::FlexEnd,
                        ..Default::default()
                    },
                    text: Text {
                        value: format!("v{}", env!("CARGO_PKG_VERSION")),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT_DIM,
                            font_size: 40.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
            })
            .with(ScreenTag);

        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(10.),
                        right: Val::Undefined,
                        bottom: Val::Undefined,
                        top: Val::Percent(40.),
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: color_none,
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|instruction_parent| {
                instruction_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(70. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Instructions:".to_string(),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 70.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
                instruction_parent
                    .spawn(NodeComponents {
                        style: Style {
                            position: Rect::<Val> {
                                left: Val::Px(50. / 2.),
                                right: Val::Undefined,
                                bottom: Val::Undefined,
                                top: Val::Px(10. / 2.),
                            },
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        material: color_none,
                        ..Default::default()
                    })
                    .with_children(|controls_parent| {
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60. / 2.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Use arrows for direction".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0 / 2.,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60. / 2.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Avoid the wall of fire".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0 / 2.,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60. / 2.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Use spacebar to place a bomb".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0 / 2.,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60. / 2.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Blow away obstacles or avoid them".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0 / 2.,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60. / 2.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Survive for as long as you can".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0 / 2.,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60. / 2.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Game difficulty will increase each round".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0 / 2.,
                                },
                            },
                            ..Default::default()
                        });
                    });
            });

        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(50.),
                        right: Val::Undefined,
                        bottom: Val::Undefined,
                        top: Val::Percent(40.),
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: color_none,
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|bonus_parent| {
                bonus_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(70. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Bonus:".to_string(),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 70.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
                bonus_parent
                    .spawn(NodeComponents {
                        style: Style {
                            position: Rect::<Val> {
                                left: Val::Px(50. / 2.),
                                right: Val::Undefined,
                                bottom: Val::Undefined,
                                top: Val::Px(10. / 2.),
                            },
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        material: color_none,
                        ..Default::default()
                    })
                    .with_children(|controls_parent| {
                        controls_parent
                            .spawn(NodeComponents {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                material: color_none,
                                ..Default::default()
                            })
                            .with_children(|this_bonus_parent| {
                                this_bonus_parent.spawn(ImageComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(30.),
                                            width: Val::Px(30.),
                                        },
                                        ..Default::default()
                                    },
                                    material: board_assets.powerup_score_handle,
                                    ..Default::default()
                                });
                                this_bonus_parent.spawn(TextComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(60. / 2.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Score bonus".to_string(),
                                        font: font_sub,
                                        style: TextStyle {
                                            color: crate::ui::ColorScheme::TEXT,
                                            font_size: 60.0 / 2.,
                                        },
                                    },
                                    ..Default::default()
                                });
                            });
                        controls_parent
                            .spawn(NodeComponents {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                material: color_none,
                                ..Default::default()
                            })
                            .with_children(|this_bonus_parent| {
                                this_bonus_parent.spawn(ImageComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(30.),
                                            width: Val::Px(30.),
                                        },
                                        ..Default::default()
                                    },
                                    material: board_assets.powerup_bomb_damage_handle,
                                    ..Default::default()
                                });
                                this_bonus_parent.spawn(TextComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(60. / 2.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Increase bomb damage".to_string(),
                                        font: font_sub,
                                        style: TextStyle {
                                            color: crate::ui::ColorScheme::TEXT,
                                            font_size: 60.0 / 2.,
                                        },
                                    },
                                    ..Default::default()
                                });
                            });
                        controls_parent
                            .spawn(NodeComponents {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                material: color_none,
                                ..Default::default()
                            })
                            .with_children(|this_bonus_parent| {
                                this_bonus_parent.spawn(ImageComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(30.),
                                            width: Val::Px(30.),
                                        },
                                        ..Default::default()
                                    },
                                    material: board_assets.powerup_bomb_range_handle,
                                    ..Default::default()
                                });
                                this_bonus_parent.spawn(TextComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(60. / 2.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Increase bomb range".to_string(),
                                        font: font_sub,
                                        style: TextStyle {
                                            color: crate::ui::ColorScheme::TEXT,
                                            font_size: 60.0 / 2.,
                                        },
                                    },
                                    ..Default::default()
                                });
                            });
                        controls_parent
                            .spawn(NodeComponents {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                material: color_none,
                                ..Default::default()
                            })
                            .with_children(|this_bonus_parent| {
                                this_bonus_parent.spawn(ImageComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(30.),
                                            width: Val::Px(30.),
                                        },
                                        ..Default::default()
                                    },
                                    material: board_assets.powerup_bomb_count_handle,
                                    ..Default::default()
                                });
                                this_bonus_parent.spawn(TextComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(60. / 2.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Increase number of bombs".to_string(),
                                        font: font_sub,
                                        style: TextStyle {
                                            color: crate::ui::ColorScheme::TEXT,
                                            font_size: 60.0 / 2.,
                                        },
                                    },
                                    ..Default::default()
                                });
                            });
                        controls_parent
                            .spawn(NodeComponents {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                material: color_none,
                                ..Default::default()
                            })
                            .with_children(|this_bonus_parent| {
                                this_bonus_parent.spawn(ImageComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(30.),
                                            width: Val::Px(30.),
                                        },
                                        ..Default::default()
                                    },
                                    material: board_assets.powerup_bomb_speed_handle,
                                    ..Default::default()
                                });
                                this_bonus_parent.spawn(TextComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(60. / 2.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text {
                                        value: "Decrease delay before bomb explosion".to_string(),
                                        font: font_sub,
                                        style: TextStyle {
                                            color: crate::ui::ColorScheme::TEXT,
                                            font_size: 60.0 / 2.,
                                        },
                                    },
                                    ..Default::default()
                                });
                            });
                    });
            });

        screen.loaded = true;
    }
}

fn tear_down(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut screen: ResMut<Screen>,
    mut query: Query<(Entity, &ScreenTag)>,
) {
    if game_state.current_screen != CURRENT_SCREEN && screen.loaded {
        info!("tear down");

        for (entity, _tag) in &mut query.iter() {
            commands.despawn_recursive(entity);
        }

        screen.loaded = false;
    }
}

fn keyboard_input_system(
    mut game_state: ResMut<crate::GameState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if game_state.current_screen == CURRENT_SCREEN && keyboard_input.just_released(KeyCode::Escape)
    {
        game_state.current_screen = crate::Screen::Menu;
    }
}

fn mouse_input_system(
    mut game_state: ResMut<crate::GameState>,
    screen: Res<Screen>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if game_state.current_screen == CURRENT_SCREEN
        && screen.loaded
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        game_state.current_screen = crate::Screen::Menu;
    }
}
