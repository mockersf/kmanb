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
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let font: Handle<Font> = asset_server
            .load("assets/fonts/kenvector_future.ttf")
            .expect("was able to load font");

        let font_sub: Handle<Font> = asset_server
            .load("assets/fonts/mandrill.ttf")
            .expect("was able to load font");

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
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|title_parent| {
                title_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(150.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Keep Moving".to_string(),
                        font,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 150.0,
                        },
                    },
                    ..Default::default()
                });
                title_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(40.),
                            ..Default::default()
                        },
                        margin: Rect {
                            right: Val::Px(20.),
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
                            font_size: 40.0,
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
                        left: Val::Percent(30.),
                        right: Val::Undefined,
                        bottom: Val::Undefined,
                        top: Val::Percent(40.),
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|instruction_parent| {
                instruction_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(70.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Instructions:".to_string(),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 70.0,
                        },
                    },
                    ..Default::default()
                });
                instruction_parent
                    .spawn(NodeComponents {
                        style: Style {
                            position: Rect::<Val> {
                                left: Val::Px(50.),
                                right: Val::Undefined,
                                bottom: Val::Undefined,
                                top: Val::Px(10.),
                            },
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        ..Default::default()
                    })
                    .with_children(|controls_parent| {
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Move right".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Blow away obstacles or avoid them".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0,
                                },
                            },
                            ..Default::default()
                        });
                        controls_parent.spawn(TextComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(60.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text {
                                value: "Survive for as long as you can".to_string(),
                                font: font_sub,
                                style: TextStyle {
                                    color: crate::ui::ColorScheme::TEXT,
                                    font_size: 60.0,
                                },
                            },
                            ..Default::default()
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
