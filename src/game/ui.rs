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

pub fn new_round(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut state: ResMut<GameEventsListenerState>,
    events: Res<Events<GameEvents>>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut buttons: ResMut<Assets<crate::ui::button::Button>>,
    mut round_text: Query<(&mut Text, &UiComponent)>,
) {
    for event in state.event_reader.iter(&events) {
        match event {
            GameEvents::NewRound => {
                game.round += 1;
                for (mut text, component) in &mut round_text.iter() {
                    if *component == UiComponent::Round {
                        text.value = format!("Round {}", game.round);
                    }
                }
                let mut rng = rand::thread_rng();
                match LaserPowerUp::iter().choose(&mut rng).unwrap() {
                    LaserPowerUp::Speed => {
                        game.laser.speed = (game.laser.speed as f64 * 0.9) as u64
                    }
                    LaserPowerUp::ObstacleSpawnDelay => {
                        game.laser.spawn_obstacles_delay =
                            (game.laser.spawn_obstacles_delay as f32 * 0.8) as u16
                    }
                    LaserPowerUp::ObstacleSpawnCount => game.laser.nb_obstacles += 2,
                    LaserPowerUp::ObstacleStrengh => game.laser.obstacle_strength += 2,
                }
            }
            GameEvents::Lost => {
                commands.spawn((DeathAnimation(Timer::from_seconds(2., false)), ScreenTag));
                game.state = GameState::Death;
            }
            GameEvents::Pause => {
                let dim_background = materials.add(Color::rgba_linear(0.3, 0.3, 0.3, 0.8).into());
                let font = asset_handles.get_font_main_handle(&asset_server);
                let font_sub = asset_handles.get_font_sub_handle(&asset_server);

                let button_handle = asset_handles.get_button_handle(
                    &asset_server,
                    &mut textures,
                    &mut materials,
                    &mut buttons,
                );
                let button = buttons.get(&button_handle).unwrap();

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
                                        material: dim_background,
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
                                                font,
                                                style: TextStyle {
                                                    color: crate::ui::ColorScheme::TEXT,
                                                    font_size: 100.,
                                                },
                                            },
                                            ..Default::default()
                                        });
                                        pause_parent
                                            .spawn(NodeComponents {
                                                material: dim_background,
                                                ..Default::default()
                                            })
                                            .with_children(|buttons_parent| {
                                                button.add(
                                                    buttons_parent,
                                                    300.,
                                                    75.,
                                                    Rect::all(Val::Px(50.)),
                                                    font_sub,
                                                    PauseButton::ToMenu,
                                                    50.,
                                                );
                                                button.add(
                                                    buttons_parent,
                                                    300.,
                                                    75.,
                                                    Rect::all(Val::Px(50.)),
                                                    font_sub,
                                                    PauseButton::Continue,
                                                    50.,
                                                );
                                            });
                                    });
                            });
                    });
                game.state = GameState::Pause(commands.current_entity().unwrap());
            }
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
    for mut death in &mut death_query.iter() {
        death.0.tick(time.delta_seconds);
        if death.0.just_finished {
            game_screen.current_screen = crate::Screen::Lost;
        } else {
            for mut animation in &mut animation_query.iter() {
                if *animation != Animation::Die {
                    *animation = Animation::Die;
                }
            }
        }
    }
}

pub fn score(mut game: ResMut<Game>, mut score: Mut<Text>, ui: &UiComponent, timer: &Timer) {
    if game.state == GameState::Play && *ui == UiComponent::Score && timer.just_finished {
        score.value = format!("{}", game.score);
        game.score += game.round;
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
        let bomb_background = materials.add(Color::NONE.into());
        let bomb_icon_handle = asset_handles
            .get_board_handles(&asset_server, materials)
            .bomb_icon_handle;
        commands
            .spawn(TextComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(150. / 3.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(10. / 3.),
                        top: Val::Px(10. / 3.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "Prepare yourself!".to_string(),
                    font,
                    style: TextStyle {
                        color: crate::ui::ColorScheme::TEXT,
                        font_size: 150.0 / 3.,
                    },
                },
                ..Default::default()
            })
            .with(UiComponent::Round)
            .with(ScreenTag);

        commands
            .spawn(TextComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(120. / 4.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(10. / 4.),
                        top: Val::Px(10. / 4.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "0".to_string(),
                    font,
                    style: TextStyle {
                        color: crate::ui::ColorScheme::TEXT,
                        font_size: 120.0 / 4.,
                    },
                },
                ..Default::default()
            })
            .with(UiComponent::Score)
            .with(Timer::from_seconds(0.1, true))
            .with(ScreenTag);

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
                material: bomb_background,
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
    materials: ResMut<Assets<ColorMaterial>>,
    mut used_bombs: Query<&BombComponent>,
    mut parent_component: Query<(Entity, &mut Children, &UiComponent)>,
) {
    let used_bombs = used_bombs.iter().iter().count();
    let bomb_icon_handle = asset_handles
        .get_board_handles(&asset_server, materials)
        .bomb_icon_handle;
    for (entity, mut children, component) in &mut parent_component.iter() {
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
                        material: bomb_icon_handle,
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
    for (_button, interaction, button_id) in &mut interaction_query.iter() {
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
                        *game = Game::default();
                        game_screen.current_screen = crate::Screen::Menu;
                    }
                }
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
