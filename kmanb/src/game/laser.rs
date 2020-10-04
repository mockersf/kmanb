use super::*;

use rand::Rng;

pub fn move_laser(
    mut commands: Commands,
    mut game: ResMut<Game>,
    wnds: Res<Windows>,
    mut game_events: ResMut<Events<GameEvents>>,
    timer: &Timer,
    mut transform: Mut<Transform>,
    entity: Entity,
    _: &LaserComponent,
) {
    if timer.just_finished {
        let ratio = wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        game.laser.x += 1;
        if game.laser.x == BOARD_X + 2 {
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
    }
}

pub fn jitter_laser(mut transform: Mut<Transform>, _: &LaserComponent) {
    let mut rng = rand::thread_rng();
    transform.translate(Vec3::new(0., rng.gen_range(-5., 5.), 0.));
}

pub struct ObstacleSpawner;

pub fn setup(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    screen: Res<Screen>,
    game: Res<Game>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen (laser)");

        commands.spawn((
            ObstacleSpawner,
            Timer::new(
                std::time::Duration::from_millis(game.laser.spawn_obstacles_delay as u64),
                true,
            ),
        ));
    }
}

pub struct ObstacleComponent(usize);

pub fn spawn_obstacles(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    game: Res<Game>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    wnds: Res<Windows>,
    mut timer_query: Query<(&ObstacleSpawner, &Timer)>,
    occupied_tiles: Query<(Entity, &ObstacleComponent)>,
) {
    if game_state.current_screen == CURRENT_SCREEN {
        let ratio = wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        let crate_handle = asset_handles
            .get_board_handles(&asset_server, materials)
            .crate_handle;
        for (_, timer) in &mut timer_query.iter() {
            if timer.just_finished {
                let mut rng = rand::thread_rng();
                std::iter::repeat_with(|| {
                    let x = rng.gen_range(0, BOARD_X);
                    let y = rng.gen_range(0, BOARD_Y);
                    (x, y)
                })
                .skip_while(|(x, y)| *x == game.player.x && *y == game.player.y)
                .map(|(x, y)| game.board.as_ref().unwrap()[y][x].entity)
                .skip_while(|cell| occupied_tiles.get::<ObstacleComponent>(*cell).is_ok())
                .take(game.laser.nb_obstacles)
                .for_each(|entity| {
                    commands.spawn(SpriteComponents {
                        material: crate_handle,
                        transform: Transform::from_translation(Vec3::new(0., 0., Z_PLAYER))
                            .with_scale(ratio * 0.7),
                        ..Default::default()
                    });
                    let obstacle = commands.current_entity().unwrap();
                    commands.push_children(entity, &[obstacle]);
                    commands.insert_one(entity, ObstacleComponent(game.laser.obstacle_strength));
                });
            }
        }
    }
}
