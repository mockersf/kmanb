use super::*;
use rand::prelude::IteratorRandom;

#[derive(Debug, Clone, PartialEq, Eq, enum_utils::IterVariants)]
enum LaserPowerUp {
    Speed,
    ObstacleSpawnDelay,
    ObstacleSpawnCount,
    ObstacleStrengh,
}

#[derive(Clone, Copy)]
pub enum PauseButton {
    Continue,
    ToMenu,
}

impl Into<String> for PauseButton {
    fn into(self) -> String {
        match self {
            PauseButton::Continue => "continue".to_string(),
            PauseButton::ToMenu => "exit to menu".to_string(),
        }
    }
}

#[derive(Default)]
pub struct GameEventsListenerState {
    event_reader: EventReader<GameEvents>,
}

pub struct Stared;

pub struct PauseMenuItemSelector(i32);

pub fn ui_event_update(
    mut commands: Commands,
    screen: Res<crate::GameScreen>,
    mut game: ResMut<Game>,
    (mut state, events): (ResMut<GameEventsListenerState>, ResMut<Events<GameEvents>>),
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nine_patches: ResMut<Assets<bevy_ninepatch::NinePatchBuilder>>,
    mut game_events: ResMut<Events<GameEvents>>,
    mut buttons: ResMut<Assets<crate::ui::button::Button>>,
    mut round_text: Query<(&mut Text, &UiComponent, &Parent)>,
    is_new_best: Query<&Stared>,
    player_query: Query<With<PlayerComponent, Entity>>,
) {
    let transparent_background = materials.add(Color::NONE.into());
    let menu_indicator: Handle<ColorMaterial> =
        asset_handles.get_ui_selection_handle(&asset_server, &mut materials);

    for event in state.event_reader.iter(&events) {
        match event {
            GameEvents::NewRound => {
                game.round += 1;
                for (mut text, component, parent) in round_text.iter_mut() {
                    if *component == UiComponent::Round {
                        text.value = format!("Round {}", game.round);
                        if screen.is_new_highround(game.round) {
                            if is_new_best.get::<Stared>(parent.0).is_err() {
                                text.style.color = crate::ui::ColorScheme::TEXT_HIGHLIGHT;
                                commands.spawn(ImageComponents {
                                    style: Style {
                                        size: Size {
                                            height: Val::Px(50.),
                                            width: Val::Px(50.),
                                        },
                                        ..Default::default()
                                    },
                                    material: asset_handles.get_board_handles_unsafe().star,
                                    ..Default::default()
                                });
                                let entity = commands.current_entity().unwrap();
                                commands.push_children(parent.0, &[entity]);
                                commands.insert_one(parent.0, Stared);
                                game_events.send(GameEvents::NewHighround);
                            }
                        }
                    }
                }
                let mut rng = rand::thread_rng();
                match if game.round == 1 {
                    // avoid power up ObstacleStrengh on first round
                    LaserPowerUp::iter()
                        .filter(|bonus| *bonus != LaserPowerUp::ObstacleStrengh)
                        .choose(&mut rng)
                        .unwrap()
                } else {
                    LaserPowerUp::iter().choose(&mut rng).unwrap()
                } {
                    LaserPowerUp::Speed => {
                        game.laser.speed =
                            (game.laser.speed as f64 * crate::CONFIG.laser_powerup_speed) as u64
                    }
                    LaserPowerUp::ObstacleSpawnDelay => {
                        game.laser.spawn_obstacles_delay = (game.laser.spawn_obstacles_delay as f32
                            * crate::CONFIG.laser_powerup_obstacle_delay)
                            as u16
                    }
                    LaserPowerUp::ObstacleSpawnCount => {
                        game.laser.nb_obstacles += crate::CONFIG.laser_powerup_nb_obstacles
                    }
                    LaserPowerUp::ObstacleStrengh => {
                        game.laser.obstacle_strength +=
                            crate::CONFIG.laser_powerup_obstacle_strength
                    }
                }
            }
            GameEvents::Lost(_) => {
                commands.spawn((DeathAnimation(Timer::from_seconds(2., false)), ScreenTag));
                for player_entity in &mut player_query.iter() {
                    commands.remove_one::<bevy_easings::EasingComponent<Transform>>(player_entity);
                }
                game.state = GameState::Death;
            }
            GameEvents::Pause => {
                let dim_background = materials.add(Color::rgba_linear(0.3, 0.3, 0.3, 0.8).into());
                let font = asset_handles.get_font_main_handle(&asset_server);
                let font_sub = asset_handles.get_font_sub_handle(&asset_server);

                let button_handle = asset_handles.get_button_handle(
                    &asset_server,
                    &mut materials,
                    &mut nine_patches,
                    &mut buttons,
                );
                let button = buttons.get(&button_handle).unwrap();

                commands.spawn(NodeComponents {
                    style: Style {
                        margin: Rect::all(Val::Px(50.)),
                        flex_direction: FlexDirection::RowReverse,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    material: transparent_background.clone(),
                    ..Default::default()
                });
                let button_menu = commands.current_entity().unwrap();
                let button_entity = button.add(
                    &mut commands,
                    300.,
                    75.,
                    Rect::all(Val::Px(50.)),
                    font_sub.clone(),
                    PauseButton::ToMenu,
                    50.,
                );
                commands
                    .spawn(ImageComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(17.),
                                width: Val::Px(17.),
                            },
                            margin: Rect {
                                right: Val::Px(15.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        draw: Draw {
                            is_transparent: true,
                            is_visible: false,
                            ..Default::default()
                        },
                        material: menu_indicator.clone(),
                        ..Default::default()
                    })
                    .with(PauseMenuItemSelector(0));
                let indicator = commands.current_entity().unwrap();
                commands.push_children(button_menu, &[button_entity, indicator]);

                commands.spawn(NodeComponents {
                    style: Style {
                        margin: Rect::all(Val::Px(50.)),
                        flex_direction: FlexDirection::RowReverse,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    material: transparent_background.clone(),
                    ..Default::default()
                });
                let button_continue = commands.current_entity().unwrap();
                let button_entity = button.add(
                    &mut commands,
                    300.,
                    75.,
                    Rect::all(Val::Px(50.)),
                    font_sub.clone(),
                    PauseButton::Continue,
                    50.,
                );
                commands
                    .spawn(ImageComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(17.),
                                width: Val::Px(17.),
                            },
                            margin: Rect {
                                right: Val::Px(15.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        draw: Draw {
                            is_transparent: true,
                            is_visible: false,
                            ..Default::default()
                        },
                        material: menu_indicator.clone(),
                        ..Default::default()
                    })
                    .with(PauseMenuItemSelector(1));
                let indicator = commands.current_entity().unwrap();
                commands.push_children(button_continue, &[button_entity, indicator]);

                // number of NodeComponents to trick around z-system for UI nodes, that increase with the length of
                // the hierarchy to the root node
                commands
                    .spawn(NodeComponents {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::RowReverse,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeComponents {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                    margin: Rect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::RowReverse,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeComponents {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                            margin: Rect::all(Val::Auto),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::ColumnReverse,
                                            ..Default::default()
                                        },
                                        material: dim_background.clone(),
                                        ..Default::default()
                                    })
                                    .with_children(|pause_parent| {
                                        pause_parent.spawn(TextComponents {
                                            style: Style {
                                                size: Size {
                                                    height: Val::Px(100.),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            text: Text {
                                                value: "Paused".to_string(),
                                                font: font.clone(),
                                                style: TextStyle {
                                                    color: crate::ui::ColorScheme::TEXT,
                                                    font_size: 100.,
                                                },
                                            },
                                            ..Default::default()
                                        });
                                        let button_parent = pause_parent
                                            .spawn(NodeComponents {
                                                material: dim_background.clone(),
                                                ..Default::default()
                                            })
                                            .current_entity()
                                            .unwrap();
                                        pause_parent.push_children(
                                            button_parent,
                                            &[button_menu, button_continue],
                                        );
                                    });
                            });
                    });
                game.state = GameState::Pause(commands.current_entity().unwrap());
            }
            GameEvents::NewHighround => {}
            GameEvents::NewHighscore => {}
            GameEvents::PlayerBonus => {}
        }
    }
}

pub struct DeathAnimation(Timer);

pub fn death_animation(
    mut game_screen: ResMut<crate::GameScreen>,
    time: Res<Time>,
    mut animation_query: Query<&mut Animation>,
    mut death_query: Query<&mut DeathAnimation>,
) {
    for mut death in death_query.iter_mut() {
        death.0.tick(time.delta_seconds);
        if death.0.just_finished {
            game_screen.current_screen = crate::Screen::Lost;
        } else {
            for mut animation in animation_query.iter_mut() {
                if *animation != Animation::Die {
                    *animation = Animation::Die;
                }
            }
        }
    }
}

pub fn score(
    mut commands: Commands,
    screen: Res<crate::GameScreen>,
    mut game: ResMut<Game>,
    mut game_events: ResMut<Events<GameEvents>>,
    asset_handles: Res<crate::AssetHandles>,
    mut score_text: Query<(&mut Text, &UiComponent, &Timer, &Parent)>,
    is_new_best: Query<&Stared>,
) {
    if game.state == GameState::Play {
        for (mut score, ui, timer, parent) in score_text.iter_mut() {
            if *ui == UiComponent::Score && timer.just_finished {
                score.value = format!("{}", game.score);
                game.score += game.round as u32;
                if screen.is_new_highscore(game.score) {
                    if is_new_best.get::<Stared>(parent.0).is_err() {
                        score.style.color = crate::ui::ColorScheme::TEXT_HIGHLIGHT;
                        commands.spawn(ImageComponents {
                            style: Style {
                                size: Size {
                                    height: Val::Px(30.),
                                    width: Val::Px(30.),
                                },
                                ..Default::default()
                            },
                            material: asset_handles.get_board_handles_unsafe().star,
                            ..Default::default()
                        });
                        let entity = commands.current_entity().unwrap();
                        commands.push_children(parent.0, &[entity]);
                        commands.insert_one(parent.0, Stared);
                        game_events.send(GameEvents::NewHighscore);
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum UiComponent {
    Round,
    Score,
    BombsAvailable,
}

pub fn setup(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    screen: Res<Screen>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen (ui)");
        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);
        let transparent_background = materials.add(Color::NONE.into());
        let bomb_icon_handle = asset_handles
            .get_board_handles(&asset_server, &mut materials)
            .bomb_icon;
        commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(50.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(4.),
                        top: Val::Px(4.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: transparent_background.clone(),
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|round_parent| {
                round_parent
                    .spawn(TextComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(50.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: "Prepare yourself!".to_string(),
                            font: font.clone(),
                            style: TextStyle {
                                color: crate::ui::ColorScheme::TEXT,
                                font_size: 50.,
                            },
                        },
                        ..Default::default()
                    })
                    .with(UiComponent::Round);
            });

        commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(30.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(3.),
                        top: Val::Px(3.),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::RowReverse,
                    ..Default::default()
                },
                material: transparent_background.clone(),
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|score_parent| {
                score_parent
                    .spawn(TextComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(30.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: "0".to_string(),
                            font: font.clone(),
                            style: TextStyle {
                                color: crate::ui::ColorScheme::TEXT,
                                font_size: 30.,
                            },
                        },
                        ..Default::default()
                    })
                    .with(UiComponent::Score)
                    .with(Timer::from_seconds(0.1, true));
            });

        commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(30. / 2.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(10. / 4.),
                        top: Val::Px(130. / 4.),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::RowReverse,
                    ..Default::default()
                },
                material: transparent_background.clone(),
                ..Default::default()
            })
            .with(UiComponent::BombsAvailable)
            .with_children(|p| {
                p.spawn(ImageComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(30. / 2.),
                            width: Val::Px(30. / 2.),
                        },
                        margin: Rect {
                            left: Val::Px(10. / 2.),
                            right: Val::Px(10. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    material: bomb_icon_handle,
                    ..Default::default()
                });
            })
            .with(ScreenTag);
    }
}

pub fn display_bombs_available(
    mut commands: Commands,
    game: Res<Game>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    used_bombs: Query<&BombComponent>,
    mut parent_component: Query<(Entity, &mut Children, &UiComponent)>,
) {
    let used_bombs = used_bombs.iter().count();
    let bomb_icon_handle = asset_handles
        .get_board_handles(&asset_server, &mut materials)
        .bomb_icon;
    for (entity, mut children, component) in parent_component.iter_mut() {
        if *component == UiComponent::BombsAvailable {
            if children.0.len() != game.player.nb_bombs - used_bombs {
                for _ in 0..children.0.len() {
                    let child = children.pop().unwrap();
                    commands.despawn(child);
                }
                for _ in 0..(game.player.nb_bombs - used_bombs) {
                    commands.spawn(ImageComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(30. / 2.),
                                width: Val::Px(30. / 2.),
                            },
                            margin: Rect {
                                left: Val::Px(10. / 2.),
                                right: Val::Px(10. / 2.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: bomb_icon_handle.clone(),
                        ..Default::default()
                    });
                    let bomb_entity = commands.current_entity().unwrap();
                    commands.push_children(entity, &[bomb_entity]);
                }
            }
        }
    }
}

pub fn button_system(
    mut commands: Commands,
    mut game_screen: ResMut<crate::GameScreen>,
    mut game: ResMut<Game>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &crate::ui::button::ButtonId<PauseButton>,
    )>,
) {
    for (_button, interaction, button_id) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => match button_id.0 {
                PauseButton::Continue => {
                    if let GameState::Pause(entity) = game.state {
                        commands.despawn_recursive(entity);
                        game.state = GameState::Play
                    }
                }
                PauseButton::ToMenu => {
                    if let GameState::Pause(entity) = game.state {
                        commands.despawn_recursive(entity);
                        game.state = GameState::Death;
                        game_screen.current_screen = crate::Screen::Menu;
                    }
                }
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub fn display_menu_item_selector(
    screen: Res<Screen>,
    mut query: Query<(&PauseMenuItemSelector, &mut Draw)>,
) {
    for (selector, mut draw) in query.iter_mut() {
        if selector.0 == screen.pause_menu_selector {
            draw.is_visible = true;
        } else {
            draw.is_visible = false;
        }
    }
}
