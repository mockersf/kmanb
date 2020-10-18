use bevy::prelude::{
    AppBuilder, AssetServer, Assets, ColorMaterial, Commands, DespawnRecursiveExt, Entity,
    IntoQuerySystem, Query, Res, ResMut, SpriteComponents, Time, Timer, Transform, Vec3,
};
use rand::Rng;

use tracing::info;

const CURRENT_SCREEN: crate::Screen = crate::Screen::Splash;

struct ScreenTag;

struct Screen {
    loaded: bool,
    done: Option<Timer>,
}
impl Default for Screen {
    fn default() -> Self {
        Screen {
            loaded: false,
            done: None,
        }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Screen::default())
            .add_system(setup.system())
            .add_system(done.system())
            .add_system(animate_logo.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

fn setup(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut screen: ResMut<Screen>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");
        screen.loaded = true;

        let logo = include_bytes!("../assets/logo.png");
        let texture_handle = asset_server.load_from(Box::new(logo.as_ref())).unwrap();

        commands
            .spawn(SpriteComponents {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with(ScreenTag)
            .with(Timer::from_seconds(0.05, true));

        screen.done = Some(Timer::from_seconds(0.7, false));
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

fn done(time: Res<Time>, mut screen: ResMut<Screen>, mut state: ResMut<crate::GameScreen>) {
    if let Some(ref mut timer) = screen.done {
        timer.tick(time.delta_seconds);
        if timer.just_finished {
            state.current_screen = crate::Screen::Menu;
        }
    }
}

fn animate_logo(mut query: Query<(&Timer, &mut Transform, &ScreenTag)>) {
    for (timer, mut transform, _tag) in &mut query.iter() {
        if timer.finished {
            let translation = transform.translation();
            if translation.x() != 0. || translation.y() != 0. {
                *transform = Transform::identity();
                continue;
            }

            let scale = transform.scale();
            // `scale.0 != 1.` for floating numbers
            if (scale.x() - 1.) > 0.01 {
                *transform = Transform::identity();
                continue;
            }

            let mut rng = rand::thread_rng();
            let act = rng.gen_range(0, 100);

            if act < 20 {
                let span = 1.;
                let x: f32 = rng.gen_range(-span, span);
                let y: f32 = rng.gen_range(-span, span);
                *transform = Transform::from_translation(Vec3::new(x, y, 0.));
            }
            if act > 80 {
                let scale_diff = 0.02;
                let new_scale: f32 = rng.gen_range(1. - scale_diff, 1. + scale_diff);
                *transform = Transform::from_scale(new_scale);
            }
        }
    }
}
