use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use tracing::info;

use bevy_easings::Ease;

const CURRENT_SCREEN: crate::Screen = crate::Screen::Menu;

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
            .init_resource::<Assets<bevy_ninepatch::NinePatch<()>>>()
            .init_resource::<Assets<crate::ui::button::Button>>()
            .add_system(setup.system())
            .add_system(button_system.system())
            .add_system(keyboard_input_system.system())
            .add_system(walk_animate_sprite_system.system())
            .add_system(remove_emote.system())
            .add_system_to_stage(crate::custom_stage::TEAR_DOWN, tear_down.system());
    }
}

#[derive(Clone, Copy)]
enum MenuButton {
    NewGame,
    About,
    Quit,
}

impl Into<String> for MenuButton {
    fn into(self) -> String {
        match self {
            MenuButton::NewGame => "New Game".to_string(),
            MenuButton::About => "About".to_string(),
            MenuButton::Quit => "Quit".to_string(),
        }
    }
}

fn setup(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut screen: ResMut<Screen>,
    mut asset_handles: ResMut<crate::AssetHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut nine_patches: ResMut<Assets<bevy_ninepatch::NinePatch<()>>>,
    mut buttons: ResMut<Assets<crate::ui::button::Button>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let panel_handle = asset_handles.get_panel_handle(
            &asset_server,
            &mut textures,
            &mut nine_patches,
            &mut materials,
        );
        let np_panel = nine_patches.get(&panel_handle).unwrap();

        let button_handle = asset_handles.get_button_handle(
            &asset_server,
            &mut textures,
            &mut materials,
            &mut buttons,
        );
        let button = buttons.get(&button_handle).unwrap();

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut textures, &mut texture_atlases);
        let _ = asset_handles.get_emote_handles(&asset_server, &mut materials);

        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);

        let color_none = materials.add(Color::NONE.into());

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: character_handle,
                transform: Transform::from_translation(Vec3::new(-200., -75., 0.)),
                sprite: TextureAtlasSprite {
                    index: 36,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(0.1, true))
            .with(ScreenTag);

        commands
            .spawn(NodeComponents {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(20.),
                        right: Val::Undefined,
                        bottom: Val::Undefined,
                        top: Val::Percent(25.),
                    },
                    size: Size::<Val> {
                        height: Val::Px(95.),
                        width: Val::Auto,
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|title_parent| {
                title_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(75.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        value: "Keep moving".to_string(),
                        font,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT,
                            font_size: 75.,
                        },
                    },
                    ..Default::default()
                });
                title_parent.spawn(TextComponents {
                    style: Style {
                        size: Size {
                            height: Val::Px(20.),
                            ..Default::default()
                        },
                        margin: Rect {
                            right: Val::Px(20.),
                            ..Default::default()
                        },
                        align_self: AlignSelf::FlexEnd,
                        ..Default::default()
                    },
                    text: Text {
                        value: "and nobody burns".to_string(),
                        font,
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT_DIM,
                            font_size: 20.,
                        },
                    },
                    ..Default::default()
                });
            })
            .with(ScreenTag);

        commands.spawn(NodeComponents {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect::<Val> {
                    left: Val::Percent(120.),
                    right: Val::Undefined,
                    bottom: Val::Percent(15.),
                    top: Val::Undefined,
                },
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        });
        if screen.first_load {
            commands.with(
                Style {
                    position_type: PositionType::Absolute,
                    position: Rect::<Val> {
                        left: Val::Percent(120.),
                        right: Val::Undefined,
                        bottom: Val::Percent(15.),
                        top: Val::Undefined,
                    },
                    ..Default::default()
                }
                .ease_to(
                    Style {
                        position_type: PositionType::Absolute,
                        position: Rect::<Val> {
                            left: Val::Percent(53.),
                            right: Val::Undefined,
                            bottom: Val::Percent(15.),
                            top: Val::Undefined,
                        },
                        ..Default::default()
                    },
                    bevy_easings::EaseFunction::BounceOut,
                    bevy_easings::EasingType::Once {
                        duration: std::time::Duration::from_millis(800),
                    },
                ),
            );
        } else {
            commands.with(Style {
                position_type: PositionType::Absolute,
                position: Rect::<Val> {
                    left: Val::Percent(53.),
                    right: Val::Undefined,
                    bottom: Val::Percent(15.),
                    top: Val::Undefined,
                },
                ..Default::default()
            });
        }
        commands
            .with_children(|global_parent| {
                np_panel.add(global_parent, 400., 300., |inside, _| {
                    inside
                        .spawn(NodeComponents {
                            material: color_none,
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                ..Default::default()
                            },
                            draw: Draw {
                                is_transparent: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            let mut current_button_shift = 30.;
                            let button_shift = 45.;
                            button.add(
                                commands,
                                225.,
                                50.,
                                Rect {
                                    left: Val::Px(current_button_shift),
                                    right: Val::Auto,
                                    top: Val::Auto,
                                    bottom: Val::Auto,
                                },
                                font,
                                MenuButton::NewGame,
                                20.,
                            );
                            current_button_shift += button_shift;
                            button.add(
                                commands,
                                225.,
                                50.,
                                Rect {
                                    left: Val::Px(current_button_shift),
                                    right: Val::Auto,
                                    top: Val::Auto,
                                    bottom: Val::Auto,
                                },
                                font,
                                MenuButton::About,
                                20.,
                            );
                            current_button_shift += button_shift;
                            button.add(
                                commands,
                                225.,
                                50.,
                                Rect {
                                    left: Val::Px(current_button_shift),
                                    right: Val::Auto,
                                    top: Val::Auto,
                                    bottom: Val::Auto,
                                },
                                font,
                                MenuButton::Quit,
                                20.,
                            );
                        });
                });
            })
            .with(ScreenTag);

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
fn keyboard_input_system(
    mut game_screen: ResMut<crate::GameScreen>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && keyboard_input.just_released(KeyCode::Escape)
    {
        game_screen.current_screen = crate::Screen::Exit;
    }
}

fn button_system(
    mut game_screen: ResMut<crate::GameScreen>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &crate::ui::button::ButtonId<MenuButton>,
    )>,
) {
    for (_button, interaction, button_id) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => match button_id.0 {
                MenuButton::Quit => game_screen.current_screen = crate::Screen::Exit,
                MenuButton::About => game_screen.current_screen = crate::Screen::About,
                MenuButton::NewGame => game_screen.current_screen = crate::Screen::Game,
            },
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}

pub struct Emote(pub Timer);

fn walk_animate_sprite_system(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    asset_handles: Res<crate::AssetHandles>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>,
) {
    if game_screen.current_screen == CURRENT_SCREEN {
        for (timer, mut sprite) in &mut query.iter() {
            if timer.finished {
                sprite.index = ((sprite.index as usize + 1) % 8 + 36) as u32;
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.005) {
                    let emotes = asset_handles.get_emote_handles_unsafe();
                    commands
                        .spawn(SpriteComponents {
                            material: [
                                emotes.exclamation,
                                emotes.face_happy,
                                emotes.heart,
                                emotes.idea,
                                emotes.laugh,
                                emotes.sleep,
                                emotes.sleeps,
                                emotes.star,
                            ]
                            .choose(&mut rng)
                            .unwrap()
                            .clone(),
                            transform: Transform::from_translation(Vec3::new(
                                -200.,
                                -75. + 192. / 2.,
                                0.,
                            ))
                            .with_scale(1.2),
                            ..Default::default()
                        })
                        .with(Emote(Timer::from_seconds(2., false)))
                        .with(ScreenTag);
                }
            }
        }
    }
}

fn remove_emote(mut commands: Commands, time: Res<Time>, mut emote: Mut<Emote>, entity: Entity) {
    emote.0.tick(time.delta_seconds);
    if emote.0.just_finished {
        commands.despawn(entity);
    }
}
