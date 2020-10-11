use bevy::{
    input::keyboard::{ElementState, KeyboardInput},
    prelude::*,
};
use tracing::info;

mod board_setup;
mod bomb;
mod keyboard_systems;
mod laser;
mod ui;

use bevy_easings::Ease;

const CURRENT_SCREEN: crate::Screen = crate::Screen::Game;

struct ScreenTag;

pub struct Screen {
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
            .init_resource::<keyboard_systems::KeyboardState>()
            .init_resource::<ui::GameEventsListenerState>()
            .add_event::<GameEvents>()
            .add_system(keyboard_systems::event_system.system())
            .add_system(board_setup::setup.system())
            .add_system(ui::setup.system())
            .add_system(laser::setup.system())
            .add_system(setup.system())
            .add_system(walk_animate_sprite_system.system())
            .add_system(board_setup::remove_indications.system())
            .add_system(clear_moving_marker.system())
            .add_system(laser::jitter_laser.system())
            .add_system(laser::move_laser.system())
            .add_system(laser::spawn_obstacles.system())
            .add_system(ui::ui_event_update.system())
            .add_system(ui::score.system())
            .add_system(ui::display_bombs_available.system())
            .add_system(ui::death_animation.system())
            .add_system(ui::button_system.system())
            .add_system(bomb::fire.system())
            .add_system(bomb::flash_bombs.system())
            .add_system(bomb::destroyed_obstacles.system())
            .add_system(bomb::player_powerups.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

fn setup(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    game: Res<Game>,
    mut screen: ResMut<Screen>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    wnds: Res<Windows>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");
        let ratio = wnds.get_primary().unwrap().width as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        let board_handles = asset_handles.get_board_handles(&asset_server, &mut materials);

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
                    .with_bundle((Animation::Walk, Timer::from_seconds(0.1, true)));
            })
            .with(PlayerComponent)
            .with(ScreenTag);

        commands.spawn((
            Transform::from_translation(Vec3::new(
                x_to(game.laser.x as i32 - 1, ratio),
                1.,
                Z_FIRE,
            )),
            GlobalTransform::identity(),
            LaserComponent(Timer::new(
                std::time::Duration::from_millis(game.laser.speed),
                true,
            )),
        ));
        for y in 0..BOARD_Y + 5 {
            commands
                .with_children(|laser_parent| {
                    laser_parent.spawn(SpriteComponents {
                        material: board_handles.laser,
                        transform: Transform::from_translation(Vec3::new(
                            1.,
                            y_to(y as i32 - 2, ratio),
                            1.,
                        ))
                        .with_non_uniform_scale(Vec3::new(
                            3. * ratio,
                            ratio,
                            1.,
                        )),
                        ..Default::default()
                    });
                })
                .with(ScreenTag);
        }

        screen.loaded = true;
        screen.first_load = false;
    }
}

fn tear_down(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut screen: ResMut<Screen>,
    mut query: Query<(Entity, &ScreenTag)>,
) {
    if game_screen.current_screen != CURRENT_SCREEN && screen.loaded {
        info!("tear down");

        for (entity, _tag) in &mut query.iter() {
            commands.despawn_recursive(entity);
        }

        screen.loaded = false;
    }
}

pub struct LaserComponent(Timer);

pub struct PlayerComponent;

#[derive(PartialEq)]
pub enum BombState {
    Fuse,
    Flash,
}

pub struct FireSprite;
pub struct FireComponent {
    damage: usize,
    x: usize,
    y: usize,
    timer: Timer,
}

pub struct BombSprite;
pub struct BombComponent {
    damage: usize,
    range: usize,
    state: BombState,
    x: usize,
    y: usize,
    timer: Timer,
}

pub struct PlayerMoving {
    timer: Timer,
}

pub struct Occupied;

const BOARD_X: usize = 21;
const BOARD_Y: usize = 10;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, enum_utils::IterVariants)]
enum PlayerPowerUp {
    Score,
    BombCount,
    BombDamage,
    BombRange,
    BombSpeed,
}

struct Player {
    x: usize,
    y: usize,
    direction: FacingDirection,
    nb_bombs: usize,
    bomb_range: usize,
    bomb_damage: usize,
    bomb_speed: u64,
    speed: u64,
}

#[derive(Clone, Copy)]
enum FacingDirection {
    Left,
    Right,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: BOARD_X / 4,
            y: BOARD_Y / 2,
            direction: FacingDirection::Right,
            nb_bombs: 2,
            bomb_range: 1,
            bomb_damage: 2,
            bomb_speed: 2000,
            speed: 200,
        }
    }
}

#[derive(Debug)]
struct Laser {
    x: usize,
    speed: u64,
    spawn_obstacles_delay: u16,
    nb_obstacles: usize,
    obstacle_strength: usize,
}

impl Default for Laser {
    fn default() -> Self {
        Laser {
            x: 1,
            speed: 1000,
            spawn_obstacles_delay: 10000,
            nb_obstacles: 5,
            obstacle_strength: 2,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Play,
    Pause(Entity),
    Death,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Play
    }
}

#[derive(Default)]
pub struct Game {
    board: Option<Vec<Vec<Cell>>>,
    player: Player,
    laser: Laser,
    pub round: u16,
    pub score: u16,
    state: GameState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Animation {
    Walk,
    Die,
}

fn walk_animate_sprite_system(
    game_screen: Res<crate::GameScreen>,
    game: Res<Game>,
    timer: &Timer,
    mut sprite: Mut<TextureAtlasSprite>,
    animation: &Animation,
) {
    if game_screen.current_screen == CURRENT_SCREEN
        && (game.state == GameState::Play || game.state == GameState::Death)
    {
        if timer.just_finished {
            sprite.index = match animation {
                Animation::Walk => {
                    if sprite.index < 36 || sprite.index > 44 {
                        36
                    } else {
                        ((sprite.index as usize + 1) % 8 + 36) as u32
                    }
                }
                Animation::Die => {
                    if sprite.index == 0 {
                        4
                    } else {
                        0
                    }
                }
            };
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

#[derive(PartialEq)]
pub enum GameEvents {
    NewRound,
    Lost,
    Pause,
    NewHighscore,
    NewHighround,
}
