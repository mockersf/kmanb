use super::*;

use rand::Rng;

pub fn move_laser(
    mut commands: Commands,
    mut game: ResMut<Game>,
    wnds: Res<Windows>,
    time: Res<Time>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut game_events: ResMut<Events<GameEvents>>,
    fire_query: Query<&FireComponent>,
    mut laser_query: Query<(Entity, &mut Transform, &mut LaserComponent)>,
) {
    if !game.died {
        if game.laser.x == game.player.x + 2 {
            game_events.send(GameEvents::Lost)
        }
        let fire_handle = asset_handles
            .get_board_handles(&asset_server, materials)
            .fire_handle;
        for (entity, mut transform, mut laser) in &mut laser_query.iter() {
            laser.0.tick(time.delta_seconds);
            if laser.0.just_finished {
                let ratio =
                    wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

                if 0 <= game.laser.x as i32 - 1 && game.laser.x as i32 - 1 < BOARD_X as i32 {
                    for y in 0..BOARD_Y {
                        let entity =
                            game.board.as_ref().unwrap()[y as usize][game.laser.x - 1].entity;
                        if fire_query.get::<FireComponent>(entity).is_err() {
                            commands
                                .spawn(SpriteComponents {
                                    material: fire_handle,
                                    transform: Transform::from_translation(Vec3::new(
                                        0., 0., Z_FIRE,
                                    ))
                                    .with_scale(ratio * 1.3),
                                    ..Default::default()
                                })
                                .with(FireSprite);
                            let fire = commands.current_entity().unwrap();
                            commands.push_children(entity, &[fire]);
                            commands.insert(
                                entity,
                                (FireComponent {
                                    damage: 1,
                                    x: game.laser.x - 1,
                                    y: y as usize,
                                    timer: Timer::from_seconds(1500. / 1000., false),
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
                        Z_FIRE,
                    ));
                    commands.remove_one::<bevy_easings::EasingComponent<Transform>>(entity);
                } else {
                    commands.insert_one(
                        entity,
                        transform.ease_to(
                            Transform::from_translation(Vec3::new(
                                x_to(game.laser.x as i32 - 1, ratio),
                                1.,
                                Z_FIRE,
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
    transform.translate(Vec3::new(0., rng.gen_range(-5., 5.), 0.));
}

pub struct ObstacleSpawner(Timer);

pub fn setup(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    screen: Res<Screen>,
    game: Res<Game>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
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
pub struct ObstacleComponent(pub usize);

pub fn spawn_obstacles(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    game: Res<Game>,
    time: Res<Time>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    wnds: Res<Windows>,
    mut spawner_query: Query<&mut ObstacleSpawner>,
    occupied_tiles: Query<&Occupied>,
) {
    if !game.died {
        if game_state.current_screen == CURRENT_SCREEN {
            let ratio =
                wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

            let crate_handle = asset_handles
                .get_board_handles(&asset_server, materials)
                .crate_handle;
            for mut spawner in &mut spawner_query.iter() {
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
                    .map(|(x, y)| game.board.as_ref().unwrap()[y][x].entity)
                    .filter(|cell| occupied_tiles.get::<Occupied>(*cell).is_err())
                    .take(game.laser.nb_obstacles)
                    .for_each(|entity| {
                        commands
                            .spawn(SpriteComponents {
                                material: crate_handle,
                                transform: Transform::from_translation(Vec3::new(0., 0., Z_PLAYER))
                                    .with_scale(ratio * 1.),
                                ..Default::default()
                            })
                            .with(ObstacleSprite);
                        let obstacle = commands.current_entity().unwrap();
                        commands.push_children(entity, &[obstacle]);
                        commands.insert(
                            entity,
                            (Occupied, ObstacleComponent(game.laser.obstacle_strength)),
                        );
                    });
                    spawner.0.duration = game.laser.spawn_obstacles_delay as f32 / 1000.;
                }
            }
        }
    }
}
