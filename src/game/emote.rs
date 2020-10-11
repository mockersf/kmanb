use bevy::prelude::*;

use super::{Game, GameEvents};

#[derive(Default)]
pub struct GameEventsListenerState {
    event_reader: EventReader<GameEvents>,
}

pub fn emote_setter(
    mut commands: Commands,
    game: Res<Game>,
    mut state: ResMut<GameEventsListenerState>,
    events: Res<Events<GameEvents>>,
    asset_handles: Res<crate::AssetHandles>,
    wnds: Res<Windows>,
    _emote_holder: &super::EmoteHolder,
    entity: Entity,
) {
    let ratio =
        wnds.get_primary().unwrap().width as f32 / super::BOARD_X as f32 / super::TILE_SIZE as f32;
    let emotes = asset_handles.get_emote_handles_unsafe();

    for event in state.event_reader.iter(&events) {
        match event {
            GameEvents::NewRound => {
                if game.round % 2 == 0 {
                    commands
                        .spawn(SpriteComponents {
                            transform: Transform::from_scale(ratio * 0.7),
                            material: emotes.face_angry,
                            ..Default::default()
                        })
                        .with(crate::menu::Emote(Timer::from_seconds(1., false)));
                    let emote = commands.current_entity().unwrap();
                    commands.push_children(entity, &[emote]);
                }
            }
            GameEvents::Lost => {
                commands
                    .spawn(SpriteComponents {
                        transform: Transform::from_scale(ratio * 0.7),
                        material: emotes.heart_broken,
                        ..Default::default()
                    })
                    .with(crate::menu::Emote(Timer::from_seconds(5., false)));
                let emote = commands.current_entity().unwrap();
                commands.push_children(entity, &[emote]);
            }
            GameEvents::NewHighround | GameEvents::NewHighscore => {
                commands
                    .spawn(SpriteComponents {
                        transform: Transform::from_scale(ratio * 0.7),
                        material: emotes.star,
                        ..Default::default()
                    })
                    .with(crate::menu::Emote(Timer::from_seconds(1., false)));
                let emote = commands.current_entity().unwrap();
                commands.push_children(entity, &[emote]);
            }
            GameEvents::Pause => (),
        };
    }
}
