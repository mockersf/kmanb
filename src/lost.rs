use bevy::prelude::*;
use tracing::info;

const CURRENT_SCREEN: crate::Screen = crate::Screen::Lost;

struct ScreenTag;

struct Screen {
    loaded: bool,
}
impl Default for Screen {
    fn default() -> Self {
        Screen { loaded: false }
    }
}

#[derive(Default)]
struct GameStats {
    last_seen_cause_of_death: Option<crate::game::CauseOfDeath>,
    bomb_placed: u16,
    obstacle_destroyed_by_player: u16,
    obstacle_destroyed_by_lazer: u16,
    bomb_chained: u16,
    score_first_bomb: u32,
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Screen::default())
            .init_resource::<GameEventsListenerState>()
            .init_resource::<GameStats>()
            .init_resource::<InterestingEventsListenerState>()
            .add_system(mouse_input_system.system())
            .add_system(setup.system())
            .add_system(update_cause_of_death.system())
            .add_system(update_stats.system())
            .add_system(keyboard_input_system.system())
            .add_system(hurt_animate_sprite_system.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

#[derive(Default)]
pub struct GameEventsListenerState {
    event_reader: EventReader<crate::game::GameEvents>,
}

#[derive(Default)]
pub struct InterestingEventsListenerState {
    event_reader: EventReader<crate::game::InterestingEvent>,
}

fn update_cause_of_death(
    mut stats: ResMut<GameStats>,
    (mut state, events): (
        ResMut<GameEventsListenerState>,
        ResMut<Events<crate::game::GameEvents>>,
    ),
) {
    state
        .event_reader
        .iter(&events)
        .filter_map(|event| match event {
            crate::game::GameEvents::Lost(c) => Some(c),
            _ => None,
        })
        .for_each(|cause| stats.last_seen_cause_of_death = Some(*cause));
}

fn update_stats(
    mut stats: ResMut<GameStats>,
    game: Res<crate::game::Game>,
    (mut state, events): (
        ResMut<InterestingEventsListenerState>,
        ResMut<Events<crate::game::InterestingEvent>>,
    ),
) {
    for event in state.event_reader.iter(&events) {
        match event {
            crate::game::InterestingEvent::BombChainDetonated => stats.bomb_chained += 1,
            crate::game::InterestingEvent::BombPlaced => {
                stats.bomb_placed += 1;
                if stats.score_first_bomb == 0 {
                    stats.score_first_bomb = game.score;
                }
            }
            crate::game::InterestingEvent::ObstacleDestroyedByLaser => {
                stats.obstacle_destroyed_by_lazer += 1
            }
            crate::game::InterestingEvent::ObstacleDestroyedByPlayer => {
                stats.obstacle_destroyed_by_player += 1
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut game_screen: ResMut<crate::GameScreen>,
    mut screen: ResMut<Screen>,
    stats: Res<GameStats>,
    mut game: ResMut<crate::game::Game>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut texture_atlases);

        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);

        let font_sub: Handle<Font> = asset_handles.get_font_sub_handle(&asset_server);

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: character_handle,
                transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(0.2, true))
            .with(ScreenTag);

        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(50.),
                        right: Val::Undefined,
                        top: Val::Percent(25.),
                        bottom: Val::Undefined,
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|parent| {
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(75.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "You lost".to_string(),
                        font,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 75.,
                        },
                    },
                    ..Default::default()
                });
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(100.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: format!("{} points", game.score),
                        font: font_sub,
                        style: TextStyle {
                            color: if game_screen.is_new_highscore(game.score) {
                                crate::ui::ColorScheme::TEXT_HIGHLIGHT
                            } else {
                                crate::ui::ColorScheme::TEXT
                            },
                            font_size: 100.0,
                        },
                    },
                    ..Default::default()
                });
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(50.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: format!("at round {}", game.round),
                        font: font_sub,
                        style: TextStyle {
                            color: if game_screen.is_new_highround(game.round) {
                                crate::ui::ColorScheme::TEXT_HIGHLIGHT
                            } else {
                                crate::ui::ColorScheme::TEXT
                            },
                            font_size: 50.,
                        },
                    },
                    ..Default::default()
                });
                let medals = asset_handles.get_medal_handles(&asset_server, &mut materials);
                if let Some(medal) = match game.score {
                    x if x < crate::CONFIG.score_bronze_medal => None,
                    x if x < crate::CONFIG.score_silver_medal => Some(medals.bronze),
                    x if x < crate::CONFIG.score_gold_medal => Some(medals.silver),
                    _ => Some(medals.gold),
                } {
                    parent.spawn(ImageComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(50.),
                                ..Default::default()
                            },
                            margin: Rect {
                                top: Val::Px(30.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: medal,
                        ..Default::default()
                    });
                }
                if let Some(cause_of_death) = stats.last_seen_cause_of_death.as_ref() {
                    parent.spawn(TextComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(50.),
                                ..Default::default()
                            },
                            margin: Rect {
                                top: Val::Px(30.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: format!("death by {}", cause_of_death),
                            font: font_sub,
                            style: TextStyle {
                                color: crate::ui::ColorScheme::TEXT,
                                font_size: 50.,
                            },
                        },
                        ..Default::default()
                    });
                }
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(30.),
                            ..Default::default()
                        },
                        margin: Rect {
                            top: Val::Px(30.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: format!(
                            "placed {} bombs, {} chain explosion",
                            stats.bomb_placed, stats.bomb_chained
                        ),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 30.,
                        },
                    },
                    ..Default::default()
                });
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(30.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: format!(
                            "destroyed {} obstacles by bombs, {} by the laser",
                            stats.obstacle_destroyed_by_player, stats.obstacle_destroyed_by_lazer
                        ),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 30.,
                        },
                    },
                    ..Default::default()
                });
            });

        if game.score > game_screen.highscore {
            game_screen.highscore = game.score;
        }
        if game.round > game_screen.highround {
            game_screen.highround = game.round;
        }
        *game = crate::game::Game::default();

        screen.loaded = true;
    }
}

fn tear_down(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut screen: ResMut<Screen>,
    mut query: Query<(Entity, &ScreenTag)>,
) {
    if game_screen.current_screen != CURRENT_SCREEN && screen.loaded {
        info!("tear down");

        for (entity, _tag) in &mut query.iter() {
            commands.despawn_recursive(entity);
        }

        screen.loaded = false;
    }
}

fn keyboard_input_system(
    mut game_screen: ResMut<crate::GameScreen>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && keyboard_input.just_released(KeyCode::Escape)
    {
        game_screen.current_screen = crate::Screen::Menu;
    }
}

fn mouse_input_system(
    mut game_screen: ResMut<crate::GameScreen>,
    screen: Res<Screen>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN
        && screen.loaded
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        game_screen.current_screen = crate::Screen::Menu;
    }
}

fn hurt_animate_sprite_system(
    game_screen: Res<crate::GameScreen>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    if game_screen.current_screen == CURRENT_SCREEN {
        for (timer, mut sprite) in &mut query.iter() {
            if timer.finished {
                if sprite.index == 0 {
                    sprite.index = 4;
                } else {
                    sprite.index = 0
                }
            }
        }
    }
}
