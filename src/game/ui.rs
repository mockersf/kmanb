use super::*;
use enum_utils::IterVariants;
use rand::prelude::IteratorRandom;

// use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq, IterVariants)]
enum LaserPowerUp {
    Speed,
    ObstacleSpawnDelay,
    ObstacleSpawnCount,
    ObstacleStrengh,
}

#[derive(Default)]
pub struct GameEventsListenerState {
    event_reader: EventReader<GameEvents>,
}

pub fn new_round(
    mut commands: Commands,
    // mut game_state: ResMut<crate::GameState>,
    mut game: ResMut<Game>,
    mut state: ResMut<GameEventsListenerState>,
    events: Res<Events<GameEvents>>,
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
                game.died = true;
            }
        }
    }
}

pub struct DeathAnimation(Timer);

pub fn death_animation(
    mut game_state: ResMut<crate::GameState>,
    time: Res<Time>,
    mut animation_query: Query<&mut Animation>,
    mut death_query: Query<&mut DeathAnimation>,
) {
    for mut death in &mut death_query.iter() {
        death.0.tick(time.delta_seconds);
        if death.0.just_finished {
            game_state.current_screen = crate::Screen::Lost;
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
    if *ui == UiComponent::Score && timer.just_finished {
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
    game_state: Res<crate::GameState>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    screen: Res<Screen>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
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
