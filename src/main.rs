use bevy::{app::AppExit, prelude::*, window::WindowMode};
use serde::{Deserialize, Serialize};

mod assets;
pub mod ui;
use assets::AssetHandles;

mod about;
mod game;
mod lost;
mod menu;
mod splash;

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    width: u32,
    height: u32,
    fullscreen: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            width: 2560,
            height: 1600,
            fullscreen: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    score_bronze_medal: u32,
    score_silver_medal: u32,
    score_gold_medal: u32,
    player_nb_bombs: usize,
    player_bomb_range: usize,
    player_bomb_damage: usize,
    player_bomb_speed: u64,
    player_speed: u64,
    player_powerup_chance: f64,
    player_powerup_bomb_damage: usize,
    player_powerup_bomb_range: usize,
    player_powerup_bomb_count: usize,
    player_powerup_bomb_speed: f64,
    player_powerup_score: u32,
    player_bomb_fire_timer: f32,
    powerup_timer: f32,
    laser_fire_timer: f32,
    laser_fire_damage: usize,
    laser_speed: u64,
    laser_spawn_obstacles_delay: u16,
    laser_nb_obstacles: usize,
    laser_obstacle_strength: usize,
    laser_powerup_speed: f64,
    laser_powerup_obstacle_delay: f32,
    laser_powerup_obstacle_strength: usize,
    laser_powerup_nb_obstacles: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            score_bronze_medal: 10000,
            score_silver_medal: 20000,
            score_gold_medal: 35000,
            player_nb_bombs: 2,
            player_bomb_range: 1,
            player_bomb_damage: 2,
            player_bomb_speed: 2000,
            player_speed: 200,
            player_powerup_chance: 0.2,
            player_powerup_bomb_damage: 2,
            player_powerup_bomb_range: 1,
            player_powerup_bomb_count: 1,
            player_powerup_bomb_speed: 0.9,
            player_powerup_score: 200,
            player_bomb_fire_timer: 0.25,
            powerup_timer: 20.,
            laser_fire_timer: 1.5,
            laser_fire_damage: 1,
            laser_speed: 1000,
            laser_spawn_obstacles_delay: 10000,
            laser_nb_obstacles: 5,
            laser_obstacle_strength: 2,
            laser_powerup_speed: 0.9,
            laser_powerup_obstacle_delay: 0.8,
            laser_powerup_obstacle_strength: 2,
            laser_powerup_nb_obstacles: 2,
        }
    }
}

use lazy_static::lazy_static;
lazy_static! {
    static ref CONFIG: Config = config::read_from("config.conf").unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = config::read_from("settings.conf")?;

    let _subscriber = if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
        .with_env_filter(
             "info,bevy_log_diagnostic=debug,kmanb=debug,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=warn",
            )
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .init();
    };

    let mut builder = App::build();

    builder
        // resources
        .add_resource(WindowDescriptor {
            title: "kmanb".to_string(),
            // width: settings.width,
            // height: settings.height,
            vsync: true,
            resizable: false,
            mode: if settings.fullscreen {
                WindowMode::Fullscreen { use_size: true }
            } else {
                WindowMode::Windowed
            },
            ..Default::default()
        })
        .add_resource(settings)
        .add_resource(ClearColor(Color::rgb(0., 0., 0.01)));

    #[cfg(feature = "bundled")]
    {
        let mut default_plugins = bevy::DefaultPluginBuilder::default();
        default_plugins.with_custom::<bevy::asset::AssetPlugin>(|builder| {
            let task_pool = builder
                .resources()
                .get::<bevy::tasks::IoTaskPool>()
                .expect("IoTaskPool resource not found")
                .0
                .clone();

            let in_memory = asset_io::InMemoryAssetIo::preloaded();
            let asset_server = bevy::asset::AssetServer::new(in_memory, task_pool);

            builder
                .add_stage_before(
                    bevy::app::stage::PRE_UPDATE,
                    bevy::asset::stage::LOAD_ASSETS,
                )
                .add_stage_after(
                    bevy::app::stage::POST_UPDATE,
                    bevy::asset::stage::ASSET_EVENTS,
                )
                .add_resource(asset_server)
                .register_property::<bevy::asset::HandleId>()
                .add_system_to_stage(
                    bevy::app::stage::PRE_UPDATE,
                    bevy::asset::free_unused_assets_system.system(),
                );
        });
        builder.add_default_plugins_with_builder(default_plugins);
    }
    #[cfg(not(feature = "bundled"))]
    builder.add_default_plugins();

    builder
        .add_plugin(::bevy_easings::EasingsPlugin)
        .add_plugin(bevy_ninepatch::NinePatchPlugin::<()>::default());

    if cfg!(debug_assertions) {
        builder
            .add_plugin(::bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(::bevy_diagnostic_entity_count::EntityCountDiagnosticsPlugin)
            .add_plugin(::bevy_log_diagnostic::LogDiagnosticsPlugin::filtered(vec![
                ::bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS,
                ::bevy_diagnostic_entity_count::EntityCountDiagnosticsPlugin::ENTITY_COUNT,
            ]));
    }

    builder
        // game management
        .add_startup_system(general_setup.system())
        .add_system(handle_state.system())
        .add_resource(GameScreen::default())
        .add_stage_after(bevy::app::stage::UPDATE, custom_stage::TEAR_DOWN)
        // ui
        .add_plugin(crate::ui::button::Plugin)
        .add_resource(AssetHandles::default())
        // screens
        .add_plugin(crate::splash::Plugin)
        .add_plugin(crate::menu::Plugin)
        .add_plugin(crate::about::Plugin)
        .add_plugin(crate::game::Plugin)
        .add_plugin(crate::lost::Plugin)
        .run();

    Ok(())
}

pub mod custom_stage {
    pub const TEAR_DOWN: &str = "kmanb:tear_down";
}

#[derive(Debug, PartialEq, Clone)]
pub enum Screen {
    Splash,
    Menu,
    About,
    Game,
    Exit,
    Lost,
}

#[derive(Debug)]
pub struct GameScreen {
    pub current_screen: Screen,
    pub highscore: u32,
    pub highround: u16,
}

impl Default for GameScreen {
    fn default() -> Self {
        GameScreen {
            current_screen: Screen::Splash,
            highscore: 0,
            highround: 0,
        }
    }
}

impl GameScreen {
    pub fn is_new_highscore(&self, score: u32) -> bool {
        self.highscore != 0 && score > self.highscore
    }
    pub fn is_new_highround(&self, round: u16) -> bool {
        self.highround != 0 && round > self.highround
    }
}

fn general_setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
    commands.spawn(UiCameraComponents::default());
}

fn handle_state(game_screen: Res<crate::GameScreen>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if game_screen.current_screen == Screen::Exit {
        app_exit_events.send(AppExit);
    }
}
