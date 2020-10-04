use super::*;

pub fn input_system(mut game_state: ResMut<crate::GameState>, keyboard_input: Res<Input<KeyCode>>) {
    if game_state.current_screen == CURRENT_SCREEN && keyboard_input.just_released(KeyCode::Escape)
    {
        game_state.current_screen = crate::Screen::Menu;
    }
}

#[derive(Default)]
pub struct KeyboardState {
    event_reader: EventReader<KeyboardInput>,
}

pub fn event_system(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut state: ResMut<KeyboardState>,
    mut game: ResMut<Game>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    wnds: Res<Windows>,
    mut player_query: Query<Without<PlayerMoving, (Entity, &PlayerComponent, &Transform)>>,
) {
    let move_delay = 200;
    if game_state.current_screen == CURRENT_SCREEN {
        for event in state.event_reader.iter(&keyboard_input_events) {
            if event.state == ElementState::Pressed {
                let mut moved = false;
                let mut teleport_border = false;
                let mut bump_border = false;
                for (entity, _player, transform) in &mut player_query.iter() {
                    let ratio = wnds.get_primary().unwrap().width as f32
                        / BOARD_X as f32
                        / TILE_SIZE as f32;

                    match event.key_code {
                        Some(KeyCode::Right) => {
                            game.player.direction = FacingDirection::Right;
                            if game.player.x == BOARD_X - 1 {
                                game.player.x = 0;
                                teleport_border = true;
                            } else {
                                game.player.x += 1;
                                moved = true;
                            }
                        }
                        Some(KeyCode::Left) => {
                            game.player.direction = FacingDirection::Left;
                            if game.player.x == 0 {
                                game.player.x = BOARD_X - 1;
                                teleport_border = true
                            } else {
                                game.player.x -= 1;
                                moved = true;
                            }
                        }
                        Some(KeyCode::Up) => {
                            if game.player.y == BOARD_Y - 1 {
                                bump_border = true;
                            } else {
                                game.player.y += 1;
                                moved = true;
                            }
                        }
                        Some(KeyCode::Down) => {
                            if game.player.y == 0 {
                                bump_border = true
                            } else {
                                game.player.y -= 1;
                                moved = true;
                            }
                        }
                        _ => (),
                    }
                    if moved {
                        commands.insert_one(
                            entity,
                            transform.ease_to(
                                Transform::from_non_uniform_scale(match game.player.direction {
                                    FacingDirection::Right => Vec3::new(1., 1., 1.),
                                    FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                })
                                .with_translation(Vec3::new(
                                    x_to(game.player.x as i32, ratio),
                                    y_to(game.player.y as i32, ratio),
                                    Z_PLAYER,
                                )),
                                bevy_easings::EaseFunction::QuadraticInOut,
                                bevy_easings::EasingType::Once {
                                    duration: std::time::Duration::from_millis(move_delay),
                                },
                            ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(move_delay),
                                    false,
                                ),
                            },
                        );
                    }
                    if teleport_border {
                        commands.insert_one(
                            entity,
                            transform
                                .ease_to(
                                    transform.with_scale(0.),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(
                                            2 * move_delay / 5,
                                        ),
                                    },
                                )
                                .ease_to(
                                    Transform::from_non_uniform_scale(
                                        match game.player.direction {
                                            FacingDirection::Right => Vec3::new(1., 1., 1.),
                                            FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                        },
                                    )
                                    .with_translation(Vec3::new(
                                        x_to(game.player.x as i32, ratio),
                                        y_to(game.player.y as i32, ratio),
                                        Z_PLAYER,
                                    ))
                                    .with_scale(0.),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(move_delay / 5),
                                    },
                                )
                                .ease_to(
                                    Transform::from_non_uniform_scale(
                                        match game.player.direction {
                                            FacingDirection::Right => Vec3::new(1., 1., 1.),
                                            FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                        },
                                    )
                                    .with_translation(
                                        Vec3::new(
                                            x_to(game.player.x as i32, ratio),
                                            y_to(game.player.y as i32, ratio),
                                            Z_PLAYER,
                                        ),
                                    ),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(
                                            2 * move_delay / 5,
                                        ),
                                    },
                                ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(move_delay),
                                    false,
                                ),
                            },
                        );
                    }
                    if bump_border {
                        let factor = match game.player.y {
                            0 => -1.,
                            _ => 1.,
                        };
                        commands.insert_one(
                            entity,
                            transform
                                .ease_to(
                                    Transform::from_non_uniform_scale(
                                        match game.player.direction {
                                            FacingDirection::Right => Vec3::new(1., 1., 1.),
                                            FacingDirection::Left => Vec3::new(-1., 1., 1.),
                                        },
                                    )
                                    .with_translation(
                                        Vec3::new(
                                            x_to(game.player.x as i32, ratio),
                                            y_to(game.player.y as i32, ratio)
                                                + factor * 0.75 * ratio * TILE_SIZE as f32 / 2.,
                                            Z_PLAYER,
                                        ),
                                    ),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(move_delay / 2),
                                    },
                                )
                                .ease_to(
                                    *transform,
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(move_delay / 2),
                                    },
                                ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(move_delay),
                                    false,
                                ),
                            },
                        );
                    }
                }
            }
        }
    }
}
