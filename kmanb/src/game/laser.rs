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
        if game.laser.x == BOARD_X {
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
