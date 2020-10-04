use bevy::{
    input::keyboard::{ElementState, KeyboardInput},
    prelude::*,
};
use tracing::info;

use bevy_easings::Ease;

const CURRENT_SCREEN: crate::Screen = crate::Screen::Game;

struct ScreenTag;

struct Screen {
    loaded: bool,
    first_load: bool,
}
impl Default for Screen {
    fn default() -> Self {
        Screen {
            loaded: false,
            first_load: true,
        }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Screen::default())
            .add_resource(Game::default())
            .init_resource::<KeyboardState>()
            .add_system(keyboard_event_system.system())
            .add_system(setup.system())
            .add_system(walk_animate_sprite_system.system())
            .add_system(keyboard_input_system.system())
            .add_system(clear_moving_marker.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

fn setup(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut game: ResMut<Game>,
    mut screen: ResMut<Screen>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    wnds: Res<Windows>,
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");
        let ratio =
            dbg!(wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32);

        let board_handles = asset_handles.get_board_handles(&asset_server, materials);

        if game.board.is_none() {
            for x in 0..BOARD_X {
                commands
                    .spawn(SpriteComponents {
                        material: board_handles.border_bottom_handle,
                        transform: Transform::from_translation(Vec3::new(
                            x_to(x as i32, ratio),
                            y_to(-1, ratio),
                            Z_BACKGROUND,
                        ))
                        .with_scale(ratio),
                        ..Default::default()
                    })
                    .with(ScreenTag);
            }
            let mut board = vec![];
            for y in 0..BOARD_Y {
                let mut line = vec![];
                for x in 0..BOARD_X {
                    commands
                        .spawn(SpriteComponents {
                            material: match (x, y) {
                                (0, 0) => board_handles.corner_bottom_left_handle,
                                (x, 0) if x == BOARD_X - 1 => {
                                    board_handles.corner_bottom_right_handle
                                }
                                (0, y) if y == BOARD_Y - 1 => board_handles.corner_top_left_handle,
                                (x, y) if x == BOARD_X - 1 && y == BOARD_Y - 1 => {
                                    board_handles.corner_top_right_handle
                                }
                                (0, _) => board_handles.ground_left_handle,
                                (x, _) if x == BOARD_X - 1 => board_handles.ground_right_handle,
                                (_, 0) => board_handles.ground_bottom_handle,
                                (_, y) if y == BOARD_Y - 1 => board_handles.ground_top_handle,
                                _ => board_handles.ground_handle,
                            },
                            transform: Transform::from_translation(Vec3::new(
                                x_to(x as i32, ratio),
                                y_to(y as i32, ratio),
                                Z_BACKGROUND,
                            ))
                            .with_scale(ratio),
                            ..Default::default()
                        })
                        .with(ScreenTag);
                    let entity = commands.current_entity().unwrap();
                    line.push(Cell { entity });
                }
                board.push(line);
            }
            game.board = Some(board);
            for x in 0..BOARD_X {
                commands
                    .spawn(SpriteComponents {
                        material: board_handles.border_top_handle,
                        transform: Transform::from_translation(Vec3::new(
                            x_to(x as i32, ratio),
                            y_to(BOARD_Y as i32, ratio),
                            Z_BACKGROUND,
                        ))
                        .with_scale(ratio),
                        ..Default::default()
                    })
                    .with(ScreenTag);
            }
            for y in (BOARD_Y + 1)..(BOARD_Y + 3) {
                for x in 0..BOARD_X {
                    commands
                        .spawn(SpriteComponents {
                            material: board_handles.water_handle,
                            transform: Transform::from_translation(Vec3::new(
                                x_to(x as i32, ratio),
                                y_to(y as i32, ratio),
                                Z_BACKGROUND,
                            ))
                            .with_scale(ratio),
                            ..Default::default()
                        })
                        .with(ScreenTag);
                }
            }
        }

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut textures, &mut texture_atlases);
        commands
            .spawn((
                Transform::from_translation(Vec3::new(
                    x_to(game.player.x as i32, ratio),
                    y_to(game.player.y as i32, ratio),
                    Z_PLAYER,
                )),
                GlobalTransform::identity(),
            ))
            .with_children(|placed_player| {
                placed_player
                    .spawn(SpriteSheetComponents {
                        texture_atlas: character_handle,
                        transform: Transform::from_scale(ratio * TILE_SIZE / PLAYER_SIZE),
                        sprite: TextureAtlasSprite {
                            index: 36,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(Timer::from_seconds(0.1, true));
            })
            .with(PlayerComponent)
            .with(ScreenTag);

        screen.loaded = true;
        screen.first_load = false;
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

struct CellComponent;
struct PlayerComponent;
struct PlayerMoving {
    timer: Timer,
}

const BOARD_X: usize = 25;
const BOARD_Y: usize = 13;

const Z_BACKGROUND: f32 = 0.0;
const Z_PLAYER: f32 = 0.1;
const Z_FIRE: f32 = 0.5;

const TILE_SIZE: f32 = 64.;
const PLAYER_SIZE: f32 = 256.;

fn x_to(x: i32, ratio: f32) -> f32 {
    (x as f32 * TILE_SIZE - (TILE_SIZE * BOARD_X as f32) / 2. + TILE_SIZE as f32 / 2.) * ratio
}

fn y_to(y: i32, ratio: f32) -> f32 {
    (y as f32 * TILE_SIZE - (TILE_SIZE * BOARD_Y as f32) / 2.) * ratio
}

struct Cell {
    entity: Entity,
}

struct Player {
    x: usize,
    y: usize,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: BOARD_X / 4,
            y: BOARD_Y / 2,
        }
    }
}

#[derive(Default)]
struct Game {
    board: Option<Vec<Vec<Cell>>>,
    player: Player,
}

fn walk_animate_sprite_system(
    game_state: Res<crate::GameState>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    if game_state.current_screen == CURRENT_SCREEN {
        for (timer, mut sprite) in &mut query.iter() {
            if timer.finished {
                sprite.index = ((sprite.index as usize + 1) % 8 + 36) as u32;
            }
        }
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

#[derive(Default)]
struct KeyboardState {
    event_reader: EventReader<KeyboardInput>,
}

fn keyboard_event_system(
    mut commands: Commands,
    game_state: Res<crate::GameState>,
    mut state: ResMut<KeyboardState>,
    mut game: ResMut<Game>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    wnds: Res<Windows>,
    mut player_query: Query<Without<PlayerMoving, (Entity, &PlayerComponent, &Transform)>>,
) {
    let move_delay = 200;
    if game_state.current_screen == CURRENT_SCREEN {
        for event in state.event_reader.iter(&keyboard_input_events) {
            if event.state == ElementState::Pressed {
                let mut moved = false;
                let mut teleport_border = false;
                let mut bump_border = false;
                for (entity, _player, transform) in &mut player_query.iter() {
                    let ratio = wnds.get_primary().unwrap().width as f32
                        / BOARD_X as f32
                        / TILE_SIZE as f32;

                    match event.key_code {
                        Some(KeyCode::Right) => {
                            if game.player.x == BOARD_X - 1 {
                                game.player.x = 0;
                                teleport_border = true;
                            } else {
                                game.player.x += 1;
                                moved = true;
                            }
                        }
                        Some(KeyCode::Left) => {
                            if game.player.x == 0 {
                                game.player.x = BOARD_X - 1;
                                teleport_border = true
                            } else {
                                game.player.x -= 1;
                                moved = true;
                            }
                        }
                        Some(KeyCode::Up) => {
                            if game.player.y == BOARD_Y - 1 {
                                bump_border = true;
                            } else {
                                game.player.y += 1;
                                moved = true;
                            }
                        }
                        Some(KeyCode::Down) => {
                            if game.player.y == 0 {
                                bump_border = true
                            } else {
                                game.player.y -= 1;
                                moved = true;
                            }
                        }
                        _ => (),
                    }
                    if moved {
                        commands.insert_one(
                            entity,
                            transform.ease_to(
                                Transform::from_translation(Vec3::new(
                                    x_to(game.player.x as i32, ratio),
                                    y_to(game.player.y as i32, ratio),
                                    Z_PLAYER,
                                )),
                                bevy_easings::EaseFunction::QuadraticInOut,
                                bevy_easings::EasingType::Once {
                                    duration: std::time::Duration::from_millis(move_delay),
                                },
                            ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(move_delay),
                                    false,
                                ),
                            },
                        );
                    }
                    if teleport_border {
                        commands.insert_one(
                            entity,
                            transform
                                .ease_to(
                                    transform.with_scale(0.),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(
                                            2 * move_delay / 5,
                                        ),
                                    },
                                )
                                .ease_to(
                                    Transform::from_translation(Vec3::new(
                                        x_to(game.player.x as i32, ratio),
                                        y_to(game.player.y as i32, ratio),
                                        Z_PLAYER,
                                    ))
                                    .with_scale(0.),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(move_delay / 5),
                                    },
                                )
                                .ease_to(
                                    Transform::from_translation(Vec3::new(
                                        x_to(game.player.x as i32, ratio),
                                        y_to(game.player.y as i32, ratio),
                                        Z_PLAYER,
                                    )),
                                    bevy_easings::EaseFunction::QuadraticInOut,
                                    bevy_easings::EasingType::Once {
                                        duration: std::time::Duration::from_millis(
                                            2 * move_delay / 5,
                                        ),
                                    },
                                ),
                        );
                        commands.insert_one(
                            entity,
                            PlayerMoving {
                                timer: Timer::new(
                                    std::time::Duration::from_millis(move_delay),
                                    false,
                                ),
                            },
                        );
                    }
                }
            }
        }
    }
}

fn clear_moving_marker(
    mut commands: Commands,
    time: Res<Time>,
    mut moving: Mut<PlayerMoving>,
    entity: Entity,
) {
    moving.timer.tick(time.delta_seconds);
    if moving.timer.finished {
        commands.remove_one::<PlayerMoving>(entity);
    }
}
