use super::*;

pub struct TeleportIndicationSprite;
pub struct TeleportIndicationComponent {
    timer: Timer,
}

pub fn remove_indications(
    mut commands: Commands,
    time: Res<Time>,
    mut indication_query: Query<(Entity, &mut TeleportIndicationComponent, &mut Children)>,
    indication_sprite_query: Query<&TeleportIndicationSprite>,
) {
    for (entity, mut indication, mut children) in indication_query.iter_mut() {
        indication.timer.tick(time.delta_seconds);
        if indication.timer.just_finished {
            commands.remove_one::<TeleportIndicationComponent>(entity);
            let mut targets = vec![];
            for child in children.iter() {
                if indication_sprite_query
                    .get::<TeleportIndicationSprite>(*child)
                    .is_ok()
                {
                    commands.despawn(*child);
                    targets.push(*child);
                }
            }
            children.retain(|i| !targets.contains(i));
        }
    }
}

pub fn setup(
    mut commands: Commands,
    (game_screen, mut game, screen, mut stats): (
        Res<crate::GameScreen>,
        ResMut<Game>,
        Res<Screen>,
        ResMut<crate::lost::GameStats>,
    ),
    mut asset_handles: ResMut<crate::AssetHandles>,
    (asset_server, mut materials): (Res<AssetServer>, ResMut<Assets<ColorMaterial>>),
    wnds: Res<Windows>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen (board)");
        *game = Game::default();
        *stats = crate::lost::GameStats::default();

        let ratio = wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        let board_handles = asset_handles.get_board_handles(&asset_server, &mut materials);

        if game.board.is_none() {
            for x in 0..BOARD_X {
                commands
                    .spawn(SpriteComponents {
                        material: board_handles.border_bottom.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                x_to(x as i32, ratio),
                                y_to(-1, ratio),
                                Z_BACKGROUND,
                            ),
                            scale: Vec3::splat(ratio),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(ScreenTag);
            }
            let mut board = vec![];
            for y in 0..BOARD_Y {
                let mut line = vec![];
                for x in 0..BOARD_X {
                    commands
                        .spawn(SpriteComponents {
                            material: match (x, y) {
                                (0, _) => board_handles.grass.clone(),
                                (x, 0) if x == BOARD_X - 1 => {
                                    board_handles.corner_bottom_right.clone()
                                }
                                (1, 0) => board_handles.corner_bottom_left.clone(),
                                (1, y) if y == BOARD_Y - 1 => board_handles.corner_top_left.clone(),
                                (x, y) if x == BOARD_X - 1 && y == BOARD_Y - 1 => {
                                    board_handles.corner_top_right.clone()
                                }
                                (1, _) => board_handles.ground_left.clone(),
                                (x, _) if x == BOARD_X - 1 => board_handles.ground_right.clone(),
                                (_, 0) => board_handles.ground_bottom.clone(),
                                (_, y) if y == BOARD_Y - 1 => board_handles.ground_top.clone(),
                                _ => board_handles.ground.clone(),
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    x_to(x as i32, ratio),
                                    y_to(y as i32, ratio),
                                    Z_BACKGROUND,
                                ),
                                scale: Vec3::splat(ratio),
                                ..Default::default()
                            },

                            ..Default::default()
                        })
                        .with(ScreenTag);
                    let entity = commands.current_entity().unwrap();
                    if x == 0 || x == BOARD_X - 1 {
                        commands.with(TeleportIndicationComponent {
                            timer: Timer::from_seconds(5., false),
                        });
                        commands.with_children(|cell| {
                            cell.spawn(SpriteComponents {
                                material: if x == 0 {
                                    board_handles.arrow_left.clone()
                                } else {
                                    board_handles.arrow_right.clone()
                                },
                                transform: Transform {
                                    translation: Vec3::new(0., 0., Z_INDICATION),
                                    scale: Vec3::splat(ratio * 0.6),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with(
                                Transform {
                                    translation: Vec3::new(0., 0., Z_INDICATION),
                                    scale: Vec3::splat(ratio * 0.6),
                                    ..Default::default()
                                }
                                .ease_to(
                                    Transform {
                                        translation: Vec3::new(0., 0., Z_INDICATION),
                                        scale: Vec3::splat(0.),
                                        ..Default::default()
                                    },
                                    bevy_easings::EaseFunction::BounceIn,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_secs(5),
                                    },
                                ),
                            )
                            .with(TeleportIndicationSprite);
                        });
                    }
                    line.push(Cell { entity });
                }
                board.push(line);
            }
            game.board = Some(board);
            for x in 0..BOARD_X {
                commands
                    .spawn(SpriteComponents {
                        material: board_handles.border_top.clone(),
                        transform: Transform {
                            translation: Vec3::new(
                                x_to(x as i32, ratio),
                                y_to(BOARD_Y as i32, ratio),
                                Z_BACKGROUND,
                            ),
                            scale: Vec3::splat(ratio),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(ScreenTag);
            }
            for y in (BOARD_Y + 1)..(BOARD_Y + 3) {
                for x in 0..BOARD_X {
                    commands
                        .spawn(SpriteComponents {
                            material: board_handles.water.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    x_to(x as i32, ratio),
                                    y_to(y as i32, ratio),
                                    Z_BACKGROUND,
                                ),
                                scale: Vec3::splat(ratio),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(ScreenTag);
                }
            }
        }
    }
}
