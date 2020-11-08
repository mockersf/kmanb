use bevy::prelude::*;
use rand::Rng;

use super::{Game, GameEvents};

pub fn emote_setter(
    mut commands: Commands,
    (mut event_reader, events): (Local<EventReader<GameEvents>>, Res<Events<GameEvents>>),
    asset_handles: Res<crate::AssetHandles>,
    game: Res<Game>,
    (wnds, time): (Res<Windows>, Res<Time>),
    _emote_holder: &super::EmoteHolder,
    entity: Entity,
) {
    let ratio = wnds.get_primary().unwrap().width() as f32
        / super::BOARD_X as f32
        / super::TILE_SIZE as f32;
    let emotes = asset_handles.get_emote_handles_unsafe();

    if game.laser.x == game.player.x || game.laser.x == game.player.x + 1 {
        commands
            .spawn(SpriteComponents {
                transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                material: emotes.exclamation,
                ..Default::default()
            })
            .with(crate::menu::Emote(Timer::from_seconds(1., false)));
        let emote = commands.current_entity().unwrap();
        commands.push_children(entity, &[emote]);
        return;
    }
    if game.time_last_move > 0. && time.seconds_since_startup - game.time_last_move > 3. {
        commands
            .spawn(SpriteComponents {
                transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                material: emotes.sleep,
                ..Default::default()
            })
            .with(crate::menu::Emote(Timer::from_seconds(0.3, false)));
        let emote = commands.current_entity().unwrap();
        commands.push_children(entity, &[emote]);
        return;
    }
    for event in event_reader.iter(&events) {
        match event {
            GameEvents::NewRound => {
                if game.round % 2 == 0 {
                    commands
                        .spawn(SpriteComponents {
                            transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                            material: emotes.face_angry,
                            ..Default::default()
                        })
                        .with(crate::menu::Emote(Timer::from_seconds(1., false)));
                    let emote = commands.current_entity().unwrap();
                    commands.push_children(entity, &[emote]);
                    return;
                }
            }
            GameEvents::Lost(_) => {
                commands
                    .spawn(SpriteComponents {
                        transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                        material: emotes.heart_broken,
                        ..Default::default()
                    })
                    .with(crate::menu::Emote(Timer::from_seconds(5., false)));
                let emote = commands.current_entity().unwrap();
                commands.push_children(entity, &[emote]);
                return;
            }
            GameEvents::NewHighround | GameEvents::NewHighscore => {
                commands
                    .spawn(SpriteComponents {
                        transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                        material: emotes.star,
                        ..Default::default()
                    })
                    .with(crate::menu::Emote(Timer::from_seconds(1., false)));
                let emote = commands.current_entity().unwrap();
                commands.push_children(entity, &[emote]);
                return;
            }
            GameEvents::PlayerBonus => {
                if rand::thread_rng().gen_bool(0.25) {
                    commands
                        .spawn(SpriteComponents {
                            transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                            material: emotes.face_happy,
                            ..Default::default()
                        })
                        .with(crate::menu::Emote(Timer::from_seconds(0.5, false)));
                    let emote = commands.current_entity().unwrap();
                    commands.push_children(entity, &[emote]);
                    return;
                }
            }
            GameEvents::Pause => (),
        };
    }
}
