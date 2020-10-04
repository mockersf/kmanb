use super::*;

pub fn flash_bombs(
    mut commands: Commands,
    game: Res<Game>,
    wnds: Res<Windows>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut bombs_query: Query<(Entity, &mut BombComponent, &mut Timer, &mut Children)>,
    bombs_sprite_query: Query<&BombSprite>,
) {
    let fire_handle = asset_handles
        .get_board_handles(&asset_server, materials)
        .fire_handle;
    let ratio = wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

    for (entity, mut bomb, mut timer, mut children) in &mut bombs_query.iter() {
        if timer.just_finished && bomb.state == BombState::Fuse {
            timer.reset();
            bomb.state = BombState::Flash;
        }
        if timer.just_finished && bomb.state == BombState::Flash {
            commands.remove_one::<BombComponent>(entity);
            commands.remove_one::<Timer>(entity);
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
                                .with_scale(ratio),
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
                                transform: Transform::from_translation(Vec3::new(0., 0., Z_FIRE))
                                    .with_scale(ratio),
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

pub fn fire(
    mut commands: Commands,
    game: Res<Game>,
    time: Res<Time>,
    mut game_events: ResMut<Events<GameEvents>>,
    mut fire_query: Query<(Entity, &mut FireComponent, &mut Children)>,
    fire_sprite_query: Query<&FireSprite>,
    obstacle_query: Query<&mut super::laser::ObstacleComponent>,
) {
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

pub fn destroyed_obstacles(
    mut commands: Commands,
    mut obstacle_query: Query<(Entity, &super::laser::ObstacleComponent, &mut Children)>,
    obstacle_sprite_query: Query<&super::laser::ObstacleSprite>,
) {
    for (entity, obstacle, mut children) in &mut obstacle_query.iter() {
        if obstacle.0 == 0 {
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
        }
    }
}
