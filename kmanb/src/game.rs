use bevy::prelude::*;
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
            .add_system(setup.system())
            .add_system(walk_animate_sprite_system.system())
            .add_system(keyboard_input_system.system())
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
) {
    if game_state.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let board_handles = asset_handles.get_board_handles(&asset_server, materials);

        if game.board.is_none() {
            let material = board_handles.ground_handle;
            let mut board = vec![];
            for y in 0..BOARD_Y {
                let mut line = vec![];
                for x in 0..BOARD_X {
                    commands
                        .spawn(SpriteComponents {
                            material,
                            transform: Transform::from_translation(Vec3::new(
                                x_to(x),
                                y_to(y),
                                Z_BACKGROUND,
                            )),
                            ..Default::default()
                        })
                        .with(ScreenTag);
                    let entity = commands.current_entity().unwrap();
                    line.push(Cell { entity });
                }
                board.push(line);
            }
            game.board = Some(board);
        }

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut textures, &mut texture_atlases);
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: character_handle,
                transform: Transform::from_translation(Vec3::new(
                    x_to(game.player.x),
                    y_to(game.player.y),
                    Z_PLAYER,
                ))
                .with_scale(TILE_SIZE / PLAYER_SIZE),
                sprite: TextureAtlasSprite {
                    index: 36,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(0.1, true))
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

const BOARD_X: usize = 30;
const BOARD_Y: usize = 18;

const Z_BACKGROUND: f32 = 0.0;
const Z_PLAYER: f32 = 0.1;
const Z_FIRE: f32 = 0.5;

const TILE_SIZE: f32 = 64.;
const PLAYER_SIZE: f32 = 256.;

fn x_to(x: usize) -> f32 {
    x as f32 * TILE_SIZE - (TILE_SIZE * BOARD_X as f32) / 2.
}

fn y_to(y: usize) -> f32 {
    y as f32 * TILE_SIZE - (TILE_SIZE * BOARD_Y as f32) / 2.
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
        Player { x: 8, y: 9 }
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
