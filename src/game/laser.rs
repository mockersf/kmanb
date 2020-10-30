use super::*;

use rand::Rng;

pub fn move_laser(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut game: ResMut<Game>,
    (wnds, time): (Res<Windows>, Res<Time>),
    asset_handles: Res<crate::AssetHandles>,
    mut game_events: ResMut<Events<GameEvents>>,
    fire_query: Query<&FireComponent>,
    mut laser_query: Query<(Entity, &mut Transform, &mut LaserComponent)>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && game.state == GameState::Play {
        if game.laser.x == game.player.x + 2 {
            game_events.send(GameEvents::Lost(CauseOfDeath::Laser))
        }
        let fire_handle = asset_handles.get_board_handles_unsafe().fire;
        for (entity, mut transform, mut laser) in laser_query.iter_mut() {
            laser.0.tick(time.delta_seconds);
            if laser.0.just_finished {
                let ratio =
                    wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

                if 0 <= game.laser.x as i32 - 1 && game.laser.x as i32 - 1 < BOARD_X as i32 {
                    for y in 0..BOARD_Y {
                        let entity =
                            game.board.as_ref().unwrap()[y as usize][game.laser.x - 1].entity;
                        if fire_query.get::<FireComponent>(entity).is_err() {
                            commands
                                .spawn(SpriteComponents {
                                    material: fire_handle.clone(),
                                    transform: Transform {
                                        translation: Vec3::new(0., 0., Z_FIRE),
                                        scale: Vec3::splat(ratio * 1.3),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with(FireSprite);
                            let fire = commands.current_entity().unwrap();
                            commands.push_children(entity, &[fire]);
                            commands.insert(
                                entity,
                                (FireComponent {
                                    damage: crate::CONFIG.laser_fire_damage,
                                    x: game.laser.x - 1,
                                    y: y as usize,
                                    timer: Timer::from_seconds(
                                        crate::CONFIG.laser_fire_timer,
                                        false,
                                    ),
                                    from_player: false,
                                },),
                            );
                        }
                    }
                }

                game.laser.x += 1;
                if game.laser.x == BOARD_X + 1 {
                    game.laser.x = 0;
                    *transform = Transform::from_translation(Vec3::new(
                        x_to(game.laser.x as i32 - 1, ratio),
                        0.,
                        Z_LASER,
                    ));
                    commands.remove_one::<bevy_easings::EasingComponent<Transform>>(entity);
                } else {
                    commands.insert_one(
                        entity,
                        transform.ease_to(
                            Transform::from_translation(Vec3::new(
                                x_to(game.laser.x as i32 - 1, ratio),
                                1.,
                                Z_LASER,
                            )),
                            bevy_easings::EaseFunction::BounceInOut,
                            bevy_easings::EasingType::Once {
                                duration: std::time::Duration::from_millis(game.laser.speed),
                            },
                        ),
                    );
                }
                if game.laser.x == BOARD_X / 2 {
                    game_events.send(GameEvents::NewRound)
                }
                laser.0.duration = game.laser.speed as f32 / 1000.;
            }
        }
    }
}

pub fn jitter_laser(mut transform: Mut<Transform>, _: &LaserComponent) {
    let mut rng = rand::thread_rng();
    *transform = transform.mul_transform(Transform::from_translation(Vec3::new(
        0.,
        rng.gen_range(-5., 5.),
        0.,
    )));
}

pub struct ObstacleSpawner(Timer);

pub fn setup(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    screen: Res<Screen>,
    game: Res<Game>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen (laser)");

        commands
            .spawn((ObstacleSpawner(Timer::new(
                std::time::Duration::from_millis(game.laser.spawn_obstacles_delay as u64),
                true,
            )),))
            .with(ScreenTag);
    }
}

pub struct ObstacleSprite;
pub struct ObstacleComponent {
    pub original_life: i32,
    pub remaining_life: i32,
}

impl ObstacleComponent {
    fn new(life: usize) -> Self {
        ObstacleComponent {
            original_life: life as i32,
            remaining_life: life as i32,
        }
    }
}

pub fn update_obstacle_sprite(
    game_screen: Res<crate::GameScreen>,
    asset_handles: Res<crate::AssetHandles>,
    mut sprite_query: Query<(&mut Handle<ColorMaterial>, &ObstacleSprite)>,
    obstacle_query: Query<(&ObstacleComponent, &Children)>,
) {
    if game_screen.current_screen == CURRENT_SCREEN {
        let board_assets = asset_handles.get_board_handles_unsafe();
        let obstacle_100 = board_assets.obstacle_100;
        let obstacle_75 = board_assets.obstacle_75;
        let obstacle_50 = board_assets.obstacle_50;
        let obstacle_25 = board_assets.obstacle_25;

        for (obstacle, children) in &mut obstacle_query.iter() {
            let state = obstacle.remaining_life as f32 / obstacle.original_life as f32;
            let obstacle_handle = if state > 0.75 {
                obstacle_100.clone()
            } else if state > 0.5 {
                obstacle_75.clone()
            } else if state > 0.25 {
                obstacle_50.clone()
            } else {
                obstacle_25.clone()
            };
            for child in children.iter() {
                if let Ok(mut sprite) = sprite_query.get_mut::<Handle<ColorMaterial>>(*child) {
                    if *sprite != obstacle_handle.clone() {
                        *sprite = obstacle_handle.clone();
                    }
                }
            }
        }
    }
}

pub fn spawn_obstacles(
    mut commands: Commands,
    (game_screen, game): (Res<crate::GameScreen>, Res<Game>),
    asset_handles: Res<crate::AssetHandles>,
    (wnds, time): (Res<Windows>, Res<Time>),
    mut spawner_query: Query<&mut ObstacleSpawner>,
    occupied_tiles: Query<&Occupied>,
    moving: Query<&PlayerMoving>,
) {
    if game.state == GameState::Play {
        if game_screen.current_screen == CURRENT_SCREEN {
            let ratio =
                wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

            let crate_handle = asset_handles.get_board_handles_unsafe().obstacle_100;
            for mut spawner in spawner_query.iter_mut() {
                spawner.0.tick(time.delta_seconds);
                if spawner.0.just_finished {
                    let mut rng = rand::thread_rng();

                    std::iter::repeat_with(|| {
                        let x = rng.gen_range(1, BOARD_X);
                        let y = rng.gen_range(0, BOARD_Y);
                        (x, y)
                    })
                    .filter(|(x, y)| *x != game.player.x || *y != game.player.y)
                    .take(game.laser.nb_obstacles * 2)
                    .filter(|(x, y)| {
                        !moving
                            .iter()
                            .next()
                            .map(|m| m.to == (*x, *y))
                            .unwrap_or(false)
                    })
                    .map(|(x, y)| game.board.as_ref().unwrap()[y][x].entity)
                    .filter(|cell| occupied_tiles.get::<Occupied>(*cell).is_err())
                    .take(game.laser.nb_obstacles)
                    .for_each(|entity| {
                        commands
                            .spawn(SpriteComponents {
                                material: crate_handle.clone(),
                                transform: Transform {
                                    translation: Vec3::new(0., 0., Z_OBSTACLE),
                                    scale: Vec3::splat(ratio * 0.5),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with(ObstacleSprite);
                        let obstacle = commands.current_entity().unwrap();
                        commands.push_children(entity, &[obstacle]);
                        commands.insert(
                            entity,
                            (
                                Occupied,
                                ObstacleComponent::new(game.laser.obstacle_strength),
                            ),
                        );
                    });
                    spawner.0.duration = game.laser.spawn_obstacles_delay as f32 / 1000.;
                }
            }
        }
    }
}
