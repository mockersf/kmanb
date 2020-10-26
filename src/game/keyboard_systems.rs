use super::*;

#[derive(Default)]
pub struct KeyboardState {
    event_reader: EventReader<KeyboardInput>,
}

#[derive(Debug, Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum PlayerAction {
    PoseBomb,
    Move(MoveDirection),
}

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    (game_screen, game): (Res<crate::GameScreen>, Res<Game>),
    mut player_action: ResMut<Events<PlayerAction>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && game.state == GameState::Play {
        if keyboard_input.pressed(KeyCode::Left) {
            player_action.send(PlayerAction::Move(MoveDirection::Left));
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player_action.send(PlayerAction::Move(MoveDirection::Right));
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player_action.send(PlayerAction::Move(MoveDirection::Up));
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player_action.send(PlayerAction::Move(MoveDirection::Down));
        }
    }
}

pub fn keyboard_event_system(
    mut commands: Commands,
    (game_screen, mut game): (Res<crate::GameScreen>, ResMut<Game>),
    (mut state, keyboard_input_events): (ResMut<KeyboardState>, Res<Events<KeyboardInput>>),
    mut game_events: ResMut<Events<GameEvents>>,
    mut player_action: ResMut<Events<PlayerAction>>,
    mut used_bomb: Query<&BombComponent>,
) {
    if game_screen.current_screen == CURRENT_SCREEN {
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
                match event.key_code {
                    Some(KeyCode::Space) => {
                        if game.player.nb_bombs > used_bomb.iter().iter().count() {
                            player_action.send(PlayerAction::PoseBomb);
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

#[derive(Default)]
pub struct PlayerActionListenerState {
    event_reader: EventReader<PlayerAction>,
}

pub fn player_move_timer(time: Res<Time>, mut player: Mut<PlayerComponent>) {
    if let Some(timer) = player.0.as_mut() {
        timer.tick(time.delta_seconds);
    }
}

pub fn player_command(
    mut commands: Commands,
    mut game: ResMut<Game>,
    (mut state, events): (ResMut<PlayerActionListenerState>, Res<Events<PlayerAction>>),
    asset_handles: Res<crate::AssetHandles>,
    (wnds, time): (Res<Windows>, Res<Time>),
    mut interesting_events: ResMut<Events<InterestingEvent>>,
    mut player_query: Query<Without<PlayerMoving, (Entity, &mut PlayerComponent, &Transform)>>,
    occupied_tiles: Query<(Entity, &super::laser::ObstacleComponent)>,
    chained_eased_query: Query<&bevy_easings::EasingChainComponent<Transform>>,
) {
    let ratio = wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

    if game.board.is_some() {
        let mut moved = false;
        for event in state.event_reader.iter(&events) {
            match (event, moved) {
                (PlayerAction::PoseBomb, _) => {
                    let bomb_handle = asset_handles.get_board_handles_unsafe().bomb;
                    commands
                        .spawn(SpriteComponents {
                            material: bomb_handle.clone(),
                            transform: Transform {
                                translation: Vec3::new(0., 0., Z_BOMB),
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
                }
                (PlayerAction::Move(direction), false) => {
                    let change = match direction {
                        MoveDirection::Left => (-1, 0),
                        MoveDirection::Right => (1, 0),
                        MoveDirection::Up => (0, 1),
                        MoveDirection::Down => (0, -1),
                    };
                    let x = game.player.x as i32 + change.0;
                    let y = game.player.y as i32 + change.1;
                    let mut move_to_do = MoveToDo::Move(*direction, x, y);
                    if y < 0 || y >= BOARD_Y as i32 {
                        move_to_do = MoveToDo::Bump(*direction);
                    } else if x < 0 {
                        if occupied_tiles
                            .get::<super::laser::ObstacleComponent>(
                                game.board.as_ref().unwrap()[y as usize][BOARD_X - 1].entity,
                            )
                            .is_ok()
                        {
                            move_to_do = MoveToDo::Bump(*direction);
                        } else {
                            move_to_do = MoveToDo::Teleport(BOARD_X as u32 - 1, y as u32);
                        }
                    } else if x >= BOARD_X as i32 {
                        move_to_do = MoveToDo::Teleport(0, y as u32);
                    } else if occupied_tiles
                        .get::<super::laser::ObstacleComponent>(
                            game.board.as_ref().unwrap()[y as usize][x as usize].entity,
                        )
                        .is_ok()
                    {
                        move_to_do = MoveToDo::Bump(*direction);
                    }

                    for (entity, mut player, transform) in &mut player_query.iter() {
                        if chained_eased_query
                            .get::<bevy_easings::EasingChainComponent<Transform>>(entity)
                            .is_ok()
                        {
                            commands.remove_one::<bevy_easings::EasingChainComponent<Transform>>(
                                entity,
                            );
                        }

                        let mut remaining_move: u64 = 0;
                        if let Some(timer) = player.0.as_ref() {
                            remaining_move =
                                u64::max(0, ((timer.duration - timer.elapsed) * 1000.) as u64);
                        }
                        let move_delay = game.player.speed + remaining_move;
                        let buffer_delay = move_delay * 7 / 10;

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

                        match move_to_do {
                            MoveToDo::Move(direction, x, y) => {
                                match direction {
                                    MoveDirection::Left => {
                                        game.player.direction = FacingDirection::Left
                                    }
                                    MoveDirection::Right => {
                                        game.player.direction = FacingDirection::Right
                                    }
                                    _ => (),
                                }
                                game.player.x = x as usize;
                                game.player.y = y as usize;
                                commands.insert_one(
                                    entity,
                                    transform.ease_to(
                                        Transform {
                                            translation: Vec3::new(
                                                x_to(x as i32, ratio),
                                                y_to(y as i32, ratio),
                                                Z_PLAYER,
                                            ),
                                            scale: match game.player.direction {
                                                FacingDirection::Right => Vec3::new(1., 1., 1.),
                                                FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                            },
                                            ..Default::default()
                                        },
                                        bevy_easings::EaseMethod::Linear,
                                        bevy_easings::EasingType::Once {
                                            duration: std::time::Duration::from_millis(move_delay),
                                        },
                                    ),
                                );
                            }
                            MoveToDo::Teleport(x, y) => {
                                game.player.x = x as usize;
                                game.player.y = y as usize;
                                commands.insert_one(
                                    entity,
                                    transform.ease_to(
                                        Transform {
                                            translation: Vec3::new(
                                                x_to(x as i32, ratio),
                                                y_to(y as i32, ratio),
                                                Z_PLAYER,
                                            ),
                                            ..base_transform
                                        },
                                        bevy_easings::EaseMethod::Discrete,
                                        bevy_easings::EasingType::Once {
                                            duration: std::time::Duration::from_millis(move_delay),
                                        },
                                    ),
                                );
                            }
                            MoveToDo::Bump(_) => {
                                commands.insert_one(
                                    entity,
                                    transform
                                        .ease_to(
                                            Transform {
                                                translation: Vec3::new(
                                                    x_to(game.player.x as i32, ratio)
                                                        + change.0 as f32
                                                            * 0.65
                                                            * ratio
                                                            * TILE_SIZE as f32
                                                            / 2.,
                                                    y_to(game.player.y as i32, ratio)
                                                        + change.1 as f32
                                                            * 0.65
                                                            * ratio
                                                            * TILE_SIZE as f32
                                                            / 2.,
                                                    Z_PLAYER,
                                                ),
                                                ..base_transform
                                            },
                                            bevy_easings::EaseFunction::QuadraticIn,
                                            bevy_easings::EasingType::Once {
                                                duration: std::time::Duration::from_millis(
                                                    move_delay / 4,
                                                ),
                                            },
                                        )
                                        .ease_to(
                                            Transform {
                                                translation: Vec3::new(
                                                    x_to(game.player.x as i32, ratio),
                                                    y_to(game.player.y as i32, ratio),
                                                    Z_PLAYER,
                                                ),
                                                ..base_transform
                                            },
                                            bevy_easings::EaseFunction::QuadraticOut,
                                            bevy_easings::EasingType::Once {
                                                duration: std::time::Duration::from_millis(
                                                    move_delay / 4,
                                                ),
                                            },
                                        ),
                                );
                            }
                        }
                        game.time_last_move = time.seconds_since_startup;
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(buffer_delay),
                                    false,
                                ),
                            },
                        );
                        player.0 = Some(Timer::from_seconds(move_delay as f32 / 1000., false));
                        moved = true;
                    }
                }
                _ => (),
            }
        }
    }
}

#[derive(Debug)]
enum MoveToDo {
    Move(MoveDirection, i32, i32),
    Teleport(u32, u32),
    Bump(MoveDirection),
}
