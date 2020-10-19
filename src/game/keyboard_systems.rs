use super::*;

#[derive(Default)]
pub struct KeyboardState {
    event_reader: EventReader<KeyboardInput>,
}

pub enum BumpDirection {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn event_system(
    mut commands: Commands,
    (game_screen, mut game): (Res<crate::GameScreen>, ResMut<Game>),
    (mut state, keyboard_input_events): (ResMut<KeyboardState>, Res<Events<KeyboardInput>>),
    mut game_events: ResMut<Events<GameEvents>>,
    mut interesting_events: ResMut<Events<InterestingEvent>>,
    (wnds, time): (Res<Windows>, Res<Time>),
    asset_handles: Res<crate::AssetHandles>,
    mut player_query: Query<Without<PlayerMoving, (Entity, &PlayerComponent, &Transform)>>,
    occupied_tiles: Query<(Entity, &super::laser::ObstacleComponent)>,
    eased_query: Query<
        Or<(
            &bevy_easings::EasingComponent<Transform>,
            &bevy_easings::EasingChainComponent<Transform>,
        )>,
    >,
    mut used_bomb: Query<&BombComponent>,
) {
    let move_delay = game.player.speed;
    let buffer_delay = u64::min(game.player.speed * 7 / 10, game.player.speed - 50);
    if game_screen.current_screen == CURRENT_SCREEN {
        let bomb_handle = asset_handles.get_board_handles_unsafe().bomb;
        for event in state.event_reader.iter(&keyboard_input_events) {
            if (event.key_code == Some(KeyCode::P) || event.key_code == Some(KeyCode::Escape))
                && event.state == ElementState::Pressed
            {
                if let GameState::Pause(entity) = game.state {
                    commands.despawn_recursive(entity);
                    game.state = GameState::Play
                } else {
                    game_events.send(GameEvents::Pause);
                };
                continue;
            }
            if game.state == GameState::Play && event.state == ElementState::Pressed {
                let ratio =
                    wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;
                if event.key_code == Some(KeyCode::Space)
                    && game.player.nb_bombs > used_bomb.iter().iter().count()
                {
                    commands
                        .spawn(SpriteComponents {
                            material: bomb_handle.clone(),
                            transform: Transform {
                                translation: Vec3::new(0., 0., Z_PLAYER),
                                scale: Vec3::splat(ratio * 0.6),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(BombSprite);
                    let bomb = commands.current_entity().unwrap();
                    let entity = game.board.as_ref().unwrap()[game.player.y][game.player.x].entity;
                    commands.push_children(entity, &[bomb]);
                    commands.insert(
                        entity,
                        (
                            Occupied,
                            BombComponent {
                                damage: game.player.bomb_damage,
                                range: game.player.bomb_range,
                                state: BombState::Fuse,
                                x: game.player.x,
                                y: game.player.y,
                                timer: Timer::from_seconds(
                                    game.player.bomb_speed as f32 / 1000. / 2.,
                                    false,
                                ),
                            },
                        ),
                    );
                    interesting_events.send(InterestingEvent::BombPlaced);
                    continue;
                }
                let mut moved = false;
                let mut teleport_border = false;
                let mut bump = None;
                for (entity, _player, transform) in &mut player_query.iter() {
                    let base_transform = Transform {
                        translation: Vec3::new(
                            x_to(game.player.x as i32, ratio),
                            y_to(game.player.y as i32, ratio),
                            Z_PLAYER,
                        ),
                        scale: match game.player.direction {
                            FacingDirection::Right => Vec3::new(1., 1., 1.),
                            FacingDirection::Left => Vec3::new(-1., 1., 1.),
                        },
                        ..Default::default()
                    };
                    match event.key_code {
                        Some(KeyCode::Right) => {
                            game.player.direction = FacingDirection::Right;
                            if occupied_tiles
                                .get::<super::laser::ObstacleComponent>(
                                    game.board.as_ref().unwrap()[game.player.y]
                                        [(game.player.x + 1) % BOARD_X]
                                        .entity,
                                )
                                .is_ok()
                            {
                                bump = Some(BumpDirection::Right);
                            } else {
                                if game.player.x == BOARD_X - 1 {
                                    game.player.x = 0;
                                    teleport_border = true;
                                } else {
                                    game.player.x += 1;
                                    moved = true;
                                }
                            }
                        }
                        Some(KeyCode::Left) => {
                            game.player.direction = FacingDirection::Left;
                            if occupied_tiles
                                .get::<super::laser::ObstacleComponent>(
                                    game.board.as_ref().unwrap()[game.player.y][if game.player.x
                                        == 0
                                    {
                                        BOARD_X - 1
                                    } else {
                                        game.player.x - 1
                                    }]
                                    .entity,
                                )
                                .is_ok()
                            {
                                bump = Some(BumpDirection::Left);
                            } else {
                                if game.player.x == 0 {
                                    game.player.x = BOARD_X - 1;
                                    teleport_border = true
                                } else {
                                    game.player.x -= 1;
                                    moved = true;
                                }
                            }
                        }
                        Some(KeyCode::Up) => {
                            if occupied_tiles
                                .get::<super::laser::ObstacleComponent>(
                                    game.board.as_ref().unwrap()[(game.player.y + 1) % BOARD_Y]
                                        [game.player.x]
                                        .entity,
                                )
                                .is_ok()
                            {
                                bump = Some(BumpDirection::Bottom);
                            } else {
                                if game.player.y == BOARD_Y - 1 {
                                    bump = Some(BumpDirection::Bottom);
                                } else {
                                    game.player.y += 1;
                                    moved = true;
                                }
                            }
                        }
                        Some(KeyCode::Down) => {
                            if occupied_tiles
                                .get::<super::laser::ObstacleComponent>(
                                    game.board.as_ref().unwrap()[if game.player.y == 0 {
                                        BOARD_Y - 1
                                    } else {
                                        game.player.y - 1
                                    }][game.player.x]
                                        .entity,
                                )
                                .is_ok()
                            {
                                bump = Some(BumpDirection::Top);
                            } else {
                                if game.player.y == 0 {
                                    bump = Some(BumpDirection::Top);
                                } else {
                                    game.player.y -= 1;
                                    moved = true;
                                }
                            }
                        }
                        _ => (),
                    }
                    if moved || teleport_border || bump.is_some() {
                        if eased_query
                            .get::<bevy_easings::EasingComponent<Transform>>(entity)
                            .is_ok()
                        {
                            commands.remove_one::<bevy_easings::EasingComponent<Transform>>(entity);
                        }
                        if eased_query
                            .get::<bevy_easings::EasingChainComponent<Transform>>(entity)
                            .is_ok()
                        {
                            commands.remove_one::<bevy_easings::EasingChainComponent<Transform>>(
                                entity,
                            );
                        }
                    }
                    if moved {
                        commands.insert_one(
                            entity,
                            transform.ease_to(
                                Transform {
                                    translation: Vec3::new(
                                        x_to(game.player.x as i32, ratio),
                                        y_to(game.player.y as i32, ratio),
                                        Z_PLAYER,
                                    ),
                                    scale: match game.player.direction {
                                        FacingDirection::Right => Vec3::new(1., 1., 1.),
                                        FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                    },
                                    ..Default::default()
                                },
                                bevy_easings::EaseFunction::QuadraticIn,
                                bevy_easings::EasingType::Once {
                                    duration: std::time::Duration::from_millis(move_delay),
                                },
                            ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(buffer_delay),
                                    false,
                                ),
                            },
                        );
                    } else if teleport_border {
                        commands.insert_one(
                            entity,
                            transform.ease_to(
                                Transform {
                                    translation: Vec3::new(
                                        x_to(game.player.x as i32, ratio),
                                        y_to(game.player.y as i32, ratio),
                                        Z_PLAYER,
                                    ),
                                    scale: match game.player.direction {
                                        FacingDirection::Right => Vec3::new(1., 1., 1.),
                                        FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                    },
                                    ..Default::default()
                                },
                                bevy_easings::EaseMethod::Discrete,
                                bevy_easings::EasingType::Once {
                                    duration: std::time::Duration::from_millis(move_delay),
                                },
                            ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(buffer_delay / 2),
                                    false,
                                ),
                            },
                        );
                    } else if let Some(bump_direction) = bump.as_ref() {
                        let (x_factor, y_factor) = match bump_direction {
                            BumpDirection::Top => (0., -1.),
                            BumpDirection::Bottom => (0., 1.),
                            BumpDirection::Left => (-1., 0.),
                            BumpDirection::Right => (1., 0.),
                        };
                        commands.insert_one(
                            entity,
                            transform
                                .ease_to(
                                    Transform {
                                        translation: Vec3::new(
                                            x_to(game.player.x as i32, ratio)
                                                + x_factor * 0.65 * ratio * TILE_SIZE as f32 / 2.,
                                            y_to(game.player.y as i32, ratio)
                                                + y_factor * 0.65 * ratio * TILE_SIZE as f32 / 2.,
                                            Z_PLAYER,
                                        ),

                                        scale: match game.player.direction {
                                            FacingDirection::Right => Vec3::new(1., 1., 1.),
                                            FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                        },
                                        ..Default::default()
                                    },
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(move_delay / 4),
                                    },
                                )
                                .ease_to(
                                    base_transform,
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(move_delay / 4),
                                    },
                                ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(buffer_delay / 2),
                                    false,
                                ),
                            },
                        );
                    }
                    if moved || teleport_border || bump.is_some() {
                        game.time_last_move = time.seconds_since_startup;
                    }
                }
            }
        }
    }
}
