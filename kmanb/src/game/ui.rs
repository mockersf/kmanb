use super::*;

#[derive(Default)]
pub struct GameEventsListenerState {
    event_reader: EventReader<GameEvents>,
}

pub fn new_round(
    mut game_state: ResMut<crate::GameState>,
    mut game: ResMut<Game>,
    mut state: ResMut<GameEventsListenerState>,
    events: Res<Events<GameEvents>>,
    mut round_text: Query<(&mut Text, &UiComponent)>,
) {
    for event in state.event_reader.iter(&events) {
        match event {
            GameEvents::NewRound => {
                game.round += 1;
                for (mut text, component) in &mut round_text.iter() {
                    if *component == UiComponent::Round {
                        text.value = format!("Round {}", game.round);
                    }
                }
            }
            GameEvents::Lost => {
                game_state.current_screen = crate::Screen::Lost;
            }
        }
    }
}

pub fn score(mut game: ResMut<Game>, mut score: Mut<Text>, ui: &UiComponent, timer: &Timer) {
    if *ui == UiComponent::Score && timer.just_finished {
        score.value = format!("{}", game.score);
        game.score += game.round;
    }
}

#[derive(PartialEq)]
pub enum UiComponent {
    Round,
    Score,
}

pub fn setup(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    screen: Res<Screen>,
    asset_server: Res<AssetServer>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen (ui)");
        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);

        commands
            .spawn(TextComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(150.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(10.),
                        top: Val::Px(10.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "Prepare yourself!".to_string(),
                    font,
                    style: TextStyle {
                        color: crate::ui::ColorScheme::TEXT,
                        font_size: 150.0,
                    },
                },
                ..Default::default()
            })
            .with(UiComponent::Round)
            .with(ScreenTag);

        commands
            .spawn(TextComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(150.),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    position: Rect {
                        right: Val::Px(10.),
                        top: Val::Px(10.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    value: "0".to_string(),
                    font,
                    style: TextStyle {
                        color: crate::ui::ColorScheme::TEXT,
                        font_size: 150.0,
                    },
                },
                ..Default::default()
            })
            .with(UiComponent::Score)
            .with(Timer::from_seconds(0.1, true))
            .with(ScreenTag);
    }
}
