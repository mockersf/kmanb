use bevy::prelude::*;
use tracing::info;

const CURRENT_SCREEN: crate::Screen = crate::Screen::Lost;

struct ScreenTag;

struct Screen {
    loaded: bool,
}
impl Default for Screen {
    fn default() -> Self {
        Screen { loaded: false }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Screen::default())
            .add_system(mouse_input_system.system())
            .add_system(setup.system())
            .add_system(keyboard_input_system.system())
            .add_system(hurt_animate_sprite_system.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

fn setup(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut screen: ResMut<Screen>,
    mut game: ResMut<crate::game::Game>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut textures, &mut texture_atlases);

        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);

        let font_sub: Handle<Font> = asset_handles.get_font_sub_handle(&asset_server);

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: character_handle,
                transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(0.2, true))
            .with(ScreenTag);

        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(50.),
                        right: Val::Undefined,
                        bottom: Val::Percent(30.),
                        top: Val::Undefined,
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with(ScreenTag)
            .with_children(|parent| {
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(150. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "You lost".to_string(),
                        font,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 150.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(200. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: format!("{} points", game.score),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 200.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
                parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(100. / 2.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: format!("at round {}", game.round),
                        font: font_sub,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 100.0 / 2.,
                        },
                    },
                    ..Default::default()
                });
            });

        *game = crate::game::Game::default();

        screen.loaded = true;
    }
}

fn tear_down(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut screen: ResMut<Screen>,
    mut query: Query<(Entity, &ScreenTag)>,
) {
    if game_state.current_screen != CURRENT_SCREEN && screen.loaded {
        info!("tear down");

        for (entity, _tag) in &mut query.iter() {
            commands.despawn_recursive(entity);
        }

        screen.loaded = false;
    }
}

fn keyboard_input_system(
    mut game_state: ResMut<crate::GameState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if game_state.current_screen == CURRENT_SCREEN && keyboard_input.just_released(KeyCode::Escape)
    {
        game_state.current_screen = crate::Screen::Menu;
    }
}

fn mouse_input_system(
    mut game_state: ResMut<crate::GameState>,
    screen: Res<Screen>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if game_state.current_screen == CURRENT_SCREEN
        && screen.loaded
        && mouse_button_input.just_pressed(MouseButton::Left)
    {
        game_state.current_screen = crate::Screen::Menu;
    }
}

fn hurt_animate_sprite_system(
    game_state: Res<crate::GameState>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    if game_state.current_screen == CURRENT_SCREEN {
        for (timer, mut sprite) in &mut query.iter() {
            if timer.finished {
                if sprite.index == 0 {
                    sprite.index = 4;
                } else {
                    sprite.index = 0
                }
            }
        }
    }
}
