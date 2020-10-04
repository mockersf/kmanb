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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings: Settings = config::read_from("settings.conf")?;

    let _subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_env_filter(
            "info,bevy_log_diagnostic=debug,corylus=debug,gfx_backend_metal=warn,wgpu_core=warn,bevy_render=warn",
        )
        .init();

    App::build()
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
        .add_resource(ClearColor(Color::rgb(0., 0., 0.01)))
        // default plugins
        .add_default_plugins()
        .add_plugin(::bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(::bevy_diagnostic_entity_count::EntityCountDiagnosticsPlugin)
        .add_plugin(::bevy_log_diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(::bevy_easings::EasingsPlugin)
        // game management
        .add_startup_system(general_setup.system())
        .add_system(handle_state.system())
        .add_resource(GameState::default())
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
pub struct GameState {
    pub current_screen: Screen,
}
impl Default for GameState {
    fn default() -> Self {
        GameState {
            current_screen: Screen::Splash,
        }
    }
}

fn general_setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
    commands.spawn(UiCameraComponents::default());
}

fn handle_state(
    game_state: ResMut<crate::GameState>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if game_state.current_screen == Screen::Exit {
        app_exit_events.send(AppExit);
    }
}
