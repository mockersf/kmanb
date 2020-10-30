use super::*;

use rand::{prelude::*, Rng};

pub fn flash_bombs(
    mut commands: Commands,
    game: Res<Game>,
    (wnds, time): (Res<Windows>, Res<Time>),
    asset_handles: Res<crate::AssetHandles>,
    mut interesting_event: ResMut<Events<InterestingEvent>>,
    mut bombs_query: Query<(Entity, &mut BombComponent, &mut Children)>,
    bombs_sprite_query: Query<&BombSprite>,
    bomb_and_fire_sprites_query: Query<&FireSprite>,
    obstacle_query: Query<&super::laser::ObstacleComponent>,
) {
    if game.state == GameState::Play {
        let fire_handle = asset_handles.get_board_handles_unsafe().fire.clone();
        let ratio = wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        for (entity, mut bomb, mut children) in bombs_query.iter_mut() {
            bomb.timer.tick(time.delta_seconds);
            let mut explode_now = false;
            for child in children.iter() {
                if bomb_and_fire_sprites_query
                    .get::<FireSprite>(*child)
                    .is_ok()
                {
                    interesting_event.send(InterestingEvent::BombChainDetonated);
                    explode_now = true;
                }
            }
            if bomb.timer.just_finished && bomb.state == BombState::Fuse {
                for child in children.iter() {
                    if bombs_sprite_query.get::<BombSprite>(*child).is_ok() {
                        commands.insert_one(
                            *child,
                            bevy_easings::Ease::ease(
                                Some(Transform {
                                    translation: Vec3::new(0., 0., Z_BOMB),
                                    scale: Vec3::splat(ratio * 0.6),
                                    ..Default::default()
                                }),
                                Transform {
                                    translation: Vec3::new(0., 0., Z_BOMB),
                                    scale: Vec3::splat(ratio * 0.7),
                                    ..Default::default()
                                },
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
                commands.remove_one::<Occupied>(entity);
                commands.remove_one::<BombComponent>(entity);
                let mut targets = vec![];
                for child in children.iter() {
                    if bombs_sprite_query.get::<BombSprite>(*child).is_ok() {
                        commands.despawn(*child);
                        targets.push(*child);
                    }
                }
                children.retain(|i| !targets.contains(i));

                let mut set_on_fire = |entity, x, y| {
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
                            damage: bomb.damage,
                            x: x,
                            y: y,
                            timer: Timer::from_seconds(crate::CONFIG.player_bomb_fire_timer, false),
                            from_player: true,
                        },),
                    );
                };

                let mut stop_top = false;
                let mut stop_bottom = false;
                for i in 0..=bomb.range as usize {
                    if !stop_top && bomb.x + i < BOARD_X {
                        let entity = game.board.as_ref().unwrap()[bomb.y][bomb.x + i].entity;
                        stop_top = obstacle_query
                            .get::<super::laser::ObstacleComponent>(entity)
                            .is_ok();
                        set_on_fire(entity, bomb.x + i, bomb.y);
                    }
                    if !stop_bottom && bomb.x as i32 - i as i32 > 0 {
                        let entity = game.board.as_ref().unwrap()[bomb.y][bomb.x - i].entity;
                        stop_bottom = obstacle_query
                            .get::<super::laser::ObstacleComponent>(entity)
                            .is_ok();
                        set_on_fire(entity, bomb.x - i, bomb.y);
                    }
                }

                let mut stop_right = false;
                let mut stop_left = false;
                for j in 0..=bomb.range as usize {
                    if !stop_right && bomb.y + j < BOARD_Y {
                        let entity = game.board.as_ref().unwrap()[bomb.y + j][bomb.x].entity;
                        stop_right = obstacle_query
                            .get::<super::laser::ObstacleComponent>(entity)
                            .is_ok();
                        set_on_fire(entity, bomb.x, bomb.y + j);
                    }
                    if !stop_left && bomb.y as i32 - j as i32 > 0 {
                        let entity = game.board.as_ref().unwrap()[bomb.y - j][bomb.x].entity;
                        stop_left = obstacle_query
                            .get::<super::laser::ObstacleComponent>(entity)
                            .is_ok();
                        set_on_fire(entity, bomb.x, bomb.y - j);
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
    mut interesting_events: ResMut<Events<InterestingEvent>>,
    mut fire_query: Query<(Entity, &mut FireComponent, &mut Children)>,
    fire_sprite_query: Query<&FireSprite>,
    mut obstacle_query: Query<&mut super::laser::ObstacleComponent>,
) {
    if game.state == GameState::Play {
        for (entity, mut fire, mut children) in fire_query.iter_mut() {
            fire.timer.tick(time.delta_seconds);
            if game.player.x == fire.x && game.player.y == fire.y {
                game_events.send(GameEvents::Lost(if fire.from_player {
                    CauseOfDeath::BombFire
                } else {
                    CauseOfDeath::LaserFire
                }));
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
                    obstacle.remaining_life -= fire.damage as i32;
                    if obstacle.remaining_life <= 0 {
                        interesting_events.send(if fire.from_player {
                            InterestingEvent::ObstacleDestroyedByPlayer
                        } else {
                            InterestingEvent::ObstacleDestroyedByLaser
                        })
                    }
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
    let ratio = wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

    for (entity, obstacle, mut children) in obstacle_query.iter_mut() {
        if obstacle.remaining_life <= 0 {
            commands.remove_one::<Occupied>(entity);
            commands.remove_one::<super::laser::ObstacleComponent>(entity);
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

            if rng.gen_bool(crate::CONFIG.player_powerup_chance) {
                let powerup = PlayerPowerUp::iter().choose(&mut rng).unwrap();
                commands.insert(
                    entity,
                    (
                        PowerUpComponent {
                            powerup,
                            timer: Timer::from_seconds(crate::CONFIG.powerup_timer, false),
                            used: false,
                        },
                        Occupied,
                    ),
                );
                commands
                    .spawn(SpriteComponents {
                        material: match powerup {
                            PlayerPowerUp::Score => assets.powerup_score.clone(),
                            PlayerPowerUp::BombCount => assets.powerup_bomb_count.clone(),
                            PlayerPowerUp::BombDamage => assets.powerup_bomb_damage.clone(),
                            PlayerPowerUp::BombRange => assets.powerup_bomb_range.clone(),
                            PlayerPowerUp::BombSpeed => assets.powerup_bomb_speed.clone(),
                        },
                        transform: Transform {
                            translation: Vec3::new(0., 0., Z_POWERUP),
                            scale: Vec3::splat(ratio * 0.5),
                            ..Default::default()
                        },
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
    mut game_events: ResMut<Events<GameEvents>>,
    mut powerup_query: Query<(Entity, &mut PowerUpComponent, &mut Children)>,
    powerup_sprite_query: Query<&PowerUpSprite>,
) {
    if game.state == GameState::Play {
        for (entity, mut powerup, mut children) in powerup_query.iter_mut() {
            powerup.timer.tick(time.delta_seconds);

            if !powerup.used {
                let cell = game.board.as_ref().unwrap()[game.player.y][game.player.x].entity;
                if entity == cell {
                    match powerup.powerup {
                        PlayerPowerUp::Score => {
                            game.score += game.round as u32 * crate::CONFIG.player_powerup_score
                        }
                        PlayerPowerUp::BombCount => {
                            game.player.nb_bombs += crate::CONFIG.player_powerup_bomb_count
                        }
                        PlayerPowerUp::BombDamage => {
                            game.player.bomb_damage += crate::CONFIG.player_powerup_bomb_damage
                        }
                        PlayerPowerUp::BombRange => {
                            game.player.bomb_range += crate::CONFIG.player_powerup_bomb_range
                        }
                        PlayerPowerUp::BombSpeed => {
                            game.player.bomb_speed = (game.player.bomb_speed as f64
                                * crate::CONFIG.player_powerup_bomb_speed)
                                as u64
                        }
                    }
                    powerup.timer.duration = (game.player.speed as f32 / 1000.) * 3. / 4.;
                    powerup.timer.reset();
                    powerup.used = true;
                    game_events.send(GameEvents::PlayerBonus);
                }
            }
            if powerup.timer.just_finished {
                commands.remove_one::<Occupied>(entity);
                commands.remove_one::<PowerUpComponent>(entity);
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
    used: bool,
}
