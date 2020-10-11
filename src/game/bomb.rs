use super::*;

use rand::{prelude::*, Rng};

pub fn flash_bombs(
    mut commands: Commands,
    game: Res<Game>,
    wnds: Res<Windows>,
    time: Res<Time>,
    asset_handles: Res<crate::AssetHandles>,
    mut bombs_query: Query<(Entity, &mut BombComponent, &mut Children)>,
    bombs_sprite_query: Query<&BombSprite>,
    bomb_and_fire_sprites_query: Query<&FireSprite>,
) {
    if game.state == GameState::Play {
        let fire_handle = asset_handles.get_board_handles_unsafe().fire;
        let ratio = wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        for (entity, mut bomb, mut children) in &mut bombs_query.iter() {
            bomb.timer.tick(time.delta_seconds);
            let mut explode_now = false;
            for child in children.iter() {
                if bomb_and_fire_sprites_query
                    .get::<FireSprite>(*child)
                    .is_ok()
                {
                    explode_now = true;
                }
            }
            if bomb.timer.just_finished && bomb.state == BombState::Fuse {
                for child in children.iter() {
                    if bombs_sprite_query.get::<BombSprite>(*child).is_ok() {
                        commands.insert_one(
                            *child,
                            bevy_easings::Ease::ease(
                                Some(
                                    Transform::from_translation(Vec3::new(0., 0., Z_PLAYER))
                                        .with_scale(ratio * 0.6),
                                ),
                                Transform::from_translation(Vec3::new(0., 0., Z_PLAYER))
                                    .with_scale(ratio * 0.7),
                                bevy_easings::EaseFunction::QuarticInOut,
                                bevy_easings::EasingType::PingPong {
                                    duration: std::time::Duration::from_millis(100),
                                    pause: std::time::Duration::from_millis(25),
                                },
                            ),
                        );
                    }
                }
                bomb.state = BombState::Flash;
                bomb.timer.reset();
            }
            if bomb.timer.just_finished && bomb.state == BombState::Flash || explode_now {
                commands.remove::<(Occupied, BombComponent)>(entity);
                let mut targets = vec![];
                for child in children.iter() {
                    if bombs_sprite_query.get::<BombSprite>(*child).is_ok() {
                        commands.despawn(*child);
                        targets.push(*child);
                    }
                }
                children.retain(|i| !targets.contains(i));

                for x in (bomb.x as i32 - bomb.range as i32)..=(bomb.x as i32 + bomb.range as i32) {
                    if x >= 0 && x < BOARD_X as i32 {
                        let entity = game.board.as_ref().unwrap()[bomb.y][x as usize].entity;
                        commands
                            .spawn(SpriteComponents {
                                material: fire_handle,
                                transform: Transform::from_translation(Vec3::new(0., 0., Z_FIRE))
                                    .with_scale(ratio * 1.3),
                                ..Default::default()
                            })
                            .with(FireSprite);
                        let fire = commands.current_entity().unwrap();
                        commands.push_children(entity, &[fire]);
                        commands.insert(
                            entity,
                            (FireComponent {
                                damage: bomb.damage,
                                x: x as usize,
                                y: bomb.y,
                                timer: Timer::from_seconds(250. / 1000., false),
                            },),
                        );
                    }
                }
                for y in (bomb.y as i32 - bomb.range as i32)..=(bomb.y as i32 + bomb.range as i32) {
                    if y >= 0 && y < BOARD_Y as i32 {
                        if y as usize != bomb.y {
                            let entity = game.board.as_ref().unwrap()[y as usize][bomb.x].entity;
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
                                    damage: bomb.damage,
                                    x: bomb.x,
                                    y: y as usize,
                                    timer: Timer::from_seconds(250. / 1000., false),
                                },),
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn fire(
    mut commands: Commands,
    game: Res<Game>,
    time: Res<Time>,
    mut game_events: ResMut<Events<GameEvents>>,
    mut fire_query: Query<(Entity, &mut FireComponent, &mut Children)>,
    fire_sprite_query: Query<&FireSprite>,
    obstacle_query: Query<&mut super::laser::ObstacleComponent>,
) {
    if game.state == GameState::Play {
        for (entity, mut fire, mut children) in &mut fire_query.iter() {
            fire.timer.tick(time.delta_seconds);
            if game.player.x == fire.x && game.player.y == fire.y {
                game_events.send(GameEvents::Lost)
            }
            if fire.timer.just_finished {
                commands.remove_one::<FireComponent>(entity);
                let mut targets = vec![];
                for child in children.iter() {
                    if fire_sprite_query.get::<FireSprite>(*child).is_ok() {
                        commands.despawn(*child);
                        targets.push(*child);
                    }
                }
                children.retain(|i| !targets.contains(i));
                if let Ok(mut obstacle) =
                    obstacle_query.get_mut::<super::laser::ObstacleComponent>(entity)
                {
                    obstacle.0 = if obstacle.0 < fire.damage {
                        0
                    } else {
                        obstacle.0 - fire.damage
                    };
                }
            }
        }
    }
}

pub fn destroyed_obstacles(
    mut commands: Commands,
    asset_handles: Res<crate::AssetHandles>,
    wnds: Res<Windows>,
    mut obstacle_query: Query<(Entity, &super::laser::ObstacleComponent, &mut Children)>,
    obstacle_sprite_query: Query<&super::laser::ObstacleSprite>,
) {
    let mut rng = rand::thread_rng();
    let assets = asset_handles.get_board_handles_unsafe();
    let ratio = wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

    for (entity, obstacle, mut children) in &mut obstacle_query.iter() {
        if obstacle.0 == 0 {
            commands.remove::<(Occupied, super::laser::ObstacleComponent)>(entity);
            let mut targets = vec![];
            for child in children.iter() {
                if obstacle_sprite_query
                    .get::<super::laser::ObstacleSprite>(*child)
                    .is_ok()
                {
                    commands.despawn(*child);
                    targets.push(*child);
                }
            }
            children.retain(|i| !targets.contains(i));

            if rng.gen_bool(0.2) {
                let powerup = PlayerPowerUp::iter().choose(&mut rng).unwrap();
                commands.insert(
                    entity,
                    (
                        PowerUpComponent {
                            powerup,
                            timer: Timer::from_seconds(20., false),
                        },
                        Occupied,
                    ),
                );
                commands
                    .spawn(SpriteComponents {
                        material: match powerup {
                            PlayerPowerUp::Score => assets.powerup_score,
                            PlayerPowerUp::BombCount => assets.powerup_bomb_count,
                            PlayerPowerUp::BombDamage => assets.powerup_bomb_damage,
                            PlayerPowerUp::BombRange => assets.powerup_bomb_range,
                            PlayerPowerUp::BombSpeed => assets.powerup_bomb_speed,
                        },
                        transform: Transform::from_translation(Vec3::new(0., 0., Z_PLAYER))
                            .with_scale(ratio * 0.5),
                        ..Default::default()
                    })
                    .with(PowerUpSprite);
                let fire = commands.current_entity().unwrap();
                commands.push_children(entity, &[fire]);
            }
        }
    }
}

pub fn player_powerups(
    mut commands: Commands,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut powerup_query: Query<(Entity, &mut PowerUpComponent, &mut Children)>,
    powerup_sprite_query: Query<&PowerUpSprite>,
) {
    if game.state == GameState::Play {
        for (entity, mut powerup, mut children) in &mut powerup_query.iter() {
            powerup.timer.tick(time.delta_seconds);

            let cell = game.board.as_ref().unwrap()[game.player.y][game.player.x].entity;
            let mut consumed = false;
            if entity == cell {
                match powerup.powerup {
                    PlayerPowerUp::Score => game.score += game.round * 200,
                    PlayerPowerUp::BombCount => game.player.nb_bombs += 1,
                    PlayerPowerUp::BombDamage => game.player.bomb_damage += 1,
                    PlayerPowerUp::BombRange => game.player.bomb_range += 1,
                    PlayerPowerUp::BombSpeed => {
                        game.player.bomb_speed = (game.player.bomb_speed as f64 * 0.9) as u64
                    }
                }
                consumed = true;
            }
            if powerup.timer.just_finished || consumed {
                commands.remove::<(Occupied, PowerUpComponent)>(entity);
                let mut targets = vec![];
                for child in children.iter() {
                    if powerup_sprite_query.get::<PowerUpSprite>(*child).is_ok() {
                        commands.despawn(*child);
                        targets.push(*child);
                    }
                }
                children.retain(|i| !targets.contains(i));
            }
        }
    }
}
pub struct PowerUpSprite;
pub struct PowerUpComponent {
    timer: Timer,
    powerup: PlayerPowerUp,
}
