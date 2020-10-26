use bevy::{
    input::keyboard::{ElementState, KeyboardInput},
    prelude::*,
};
use tracing::info;

mod board_setup;
mod bomb;
mod emote;
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
            .init_resource::<Game>()
            .init_resource::<keyboard_systems::KeyboardState>()
            .init_resource::<keyboard_systems::PlayerActionListenerState>()
            .init_resource::<ui::GameEventsListenerState>()
            .init_resource::<emote::GameEventsListenerState>()
            .add_event::<GameEvents>()
            .add_event::<keyboard_systems::PlayerAction>()
            .add_event::<InterestingEvent>()
            .add_system(keyboard_systems::keyboard_event_system.system())
            .add_system(keyboard_systems::keyboard_input_system.system())
            .add_system(keyboard_systems::player_command.system())
            .add_system(keyboard_systems::player_move_timer.system())
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
            .add_system(laser::update_obstacle_sprite.system())
            .add_system(ui::ui_event_update.system())
            .add_system(ui::score.system())
            .add_system(ui::display_bombs_available.system())
            .add_system(ui::death_animation.system())
            .add_system(ui::button_system.system())
            .add_system(bomb::fire.system())
            .add_system(bomb::flash_bombs.system())
            .add_system(bomb::destroyed_obstacles.system())
            .add_system(bomb::player_powerups.system())
            .add_system(emote::emote_setter.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

pub struct EmoteHolder;

fn setup(
    mut commands: Commands,
    (game_screen, game, mut screen): (Res<crate::GameScreen>, Res<Game>, ResMut<Screen>),
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    wnds: Res<Windows>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let ratio = wnds.get_primary().unwrap().width() as f32 / BOARD_X as f32 / TILE_SIZE as f32;

        let board_handles = asset_handles.get_board_handles(&asset_server, &mut materials);
        let emotes = asset_handles.get_emote_handles_unsafe();

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut texture_atlases);
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
                        transform: Transform::from_scale(Vec3::splat(
                            ratio * TILE_SIZE / PLAYER_SIZE,
                        )),
                        sprite: TextureAtlasSprite {
                            index: 36,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_bundle((Animation::Walk, Timer::from_seconds(0.1, true)));
                placed_player
                    .spawn((
                        Transform::from_translation(Vec3::new(
                            0.,
                            ratio * (TILE_SIZE / 2.) * 1.1,
                            10.,
                        )),
                        GlobalTransform::default(),
                        EmoteHolder,
                    ))
                    .with_children(|emote| {
                        emote
                            .spawn(SpriteComponents {
                                transform: Transform::from_scale(Vec3::splat(ratio * 0.7)),
                                material: emotes.exclamations,
                                ..Default::default()
                            })
                            .with(crate::menu::Emote(Timer::from_seconds(1., false)));
                    });
            })
            .with(PlayerComponent(None))
            .with(ScreenTag);

        commands.spawn((
            Transform::from_translation(Vec3::new(
                x_to(game.laser.x as i32 - 1, ratio),
                1.,
                Z_LASER,
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
                        material: board_handles.laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(1., y_to(y as i32 - 2, ratio), 1.),
                            scale: Vec3::new(3. * ratio, ratio, 1.),
                            ..Default::default()
                        },
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

pub struct PlayerComponent(Option<Timer>);

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
    from_player: bool,
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
const Z_OBSTACLE: f32 = 0.1;
const Z_BOMB: f32 = 0.2;
const Z_POWERUP: f32 = 0.2;
const Z_PLAYER: f32 = 0.3;
const Z_FIRE: f32 = 0.4;
const Z_INDICATION: f32 = 0.5;
const Z_LASER: f32 = 0.5;

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
            nb_bombs: crate::CONFIG.player_nb_bombs,
            bomb_range: crate::CONFIG.player_bomb_range,
            bomb_damage: crate::CONFIG.player_bomb_damage,
            bomb_speed: crate::CONFIG.player_bomb_speed,
            speed: crate::CONFIG.player_speed,
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
            speed: crate::CONFIG.laser_speed,
            spawn_obstacles_delay: crate::CONFIG.laser_spawn_obstacles_delay,
            nb_obstacles: crate::CONFIG.laser_nb_obstacles,
            obstacle_strength: crate::CONFIG.laser_obstacle_strength,
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
    pub score: u32,
    time_last_move: f64,
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

#[derive(PartialEq, Clone, Copy)]
pub enum CauseOfDeath {
    Laser,
    LaserFire,
    BombFire,
}

impl std::fmt::Display for CauseOfDeath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CauseOfDeath::Laser => write!(f, "laser"),
            CauseOfDeath::LaserFire => write!(f, "laser trail"),
            CauseOfDeath::BombFire => write!(f, "your own bomb"),
        }
    }
}

#[derive(PartialEq)]
pub enum GameEvents {
    NewRound,
    Lost(CauseOfDeath),
    Pause,
    NewHighscore,
    NewHighround,
    PlayerBonus,
}

pub enum InterestingEvent {
    BombPlaced,
    ObstacleDestroyedByPlayer,
    ObstacleDestroyedByLaser,
    BombChainDetonated,
}
