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
    menu_selected: Option<i32>,
}
impl Default for Screen {
    fn default() -> Self {
        Screen {
            loaded: false,
            first_load: true,
            menu_selected: None,
        }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Screen::default())
            .add_system(keyboard_input_system.system())
            .add_system(setup.system())
            .add_system(button_system.system())
            .add_system(walk_animate_sprite_system.system())
            .add_system(remove_emote.system())
            .add_system(display_menu_item_selector.system())
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
    mut nine_patches: ResMut<Assets<bevy_ninepatch::NinePatchBuilder<()>>>,
    mut buttons: ResMut<Assets<crate::ui::button::Button>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && !screen.loaded {
        info!("Loading screen");

        let panel_handles = asset_handles.get_panel_handle(&asset_server, &mut nine_patches);

        let button_handle = asset_handles.get_button_handle(
            &asset_server,
            &mut materials,
            &mut nine_patches,
            &mut buttons,
        );
        let button = buttons.get(&button_handle).unwrap();

        let character_handle =
            asset_handles.get_character_handle(&asset_server, &mut texture_atlases);
        let _ = asset_handles.get_emote_handles(&asset_server, &mut materials);

        let font: Handle<Font> = asset_handles.get_font_main_handle(&asset_server);
        let menu_indicator: Handle<ColorMaterial> =
            asset_handles.get_ui_selection_handle(&asset_server, &mut materials);

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
                        font: font.clone(),
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
                        font: font.clone(),
                        style: TextStyle {
                            color: crate::ui::ColorScheme::TEXT_DIM,
                            font_size: 20.,
                        },
                    },
                    ..Default::default()
                });
            })
            .with(ScreenTag);

        let panel_style = Style {
            position_type: PositionType::Absolute,
            position: Rect::<Val> {
                left: Val::Percent(53.),
                right: Val::Undefined,
                bottom: Val::Percent(15.),
                top: Val::Undefined,
            },
            margin: Rect::all(Val::Px(0.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(400.), Val::Px(300.)),
            align_content: AlignContent::Stretch,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        };

        let button_shift_start = 15.;
        let button_shift = 45.;
        let buttons = &[MenuButton::NewGame, MenuButton::About, MenuButton::Quit]
            .iter()
            .enumerate()
            .map(|(i, button_item)| {
                commands.spawn(NodeComponents {
                    style: Style {
                        margin: Rect {
                            left: Val::Px(button_shift_start + i as f32 * button_shift),
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        flex_direction: FlexDirection::RowReverse,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    material: color_none.clone(),
                    ..Default::default()
                });
                let entity = commands.current_entity().unwrap();
                let button = button.add(
                    &mut commands,
                    225.,
                    50.,
                    Rect::all(Val::Auto),
                    font.clone(),
                    *button_item,
                    25.,
                );
                commands
                    .spawn(ImageComponents {
                        style: Style {
                            size: Size {
                                height: Val::Px(17.),
                                width: Val::Px(17.),
                            },
                            margin: Rect {
                                right: Val::Px(15.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        draw: Draw {
                            is_transparent: true,
                            is_visible: false,
                            ..Default::default()
                        },
                        material: menu_indicator.clone(),
                        ..Default::default()
                    })
                    .with(MenuItemSelector(i));
                let indicator = commands.current_entity().unwrap();
                commands.push_children(entity, &[button, indicator]);
                entity
            })
            .collect::<Vec<_>>();
        let inner_content = commands
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
            .current_entity()
            .unwrap();
        commands.push_children(inner_content, buttons.as_slice());

        commands
            .spawn(bevy_ninepatch::NinePatchComponents {
                style: panel_style.clone(),
                nine_patch_data: bevy_ninepatch::NinePatchData::with_single_content(
                    panel_handles.1,
                    panel_handles.0,
                    inner_content,
                ),
                ..Default::default()
            })
            .with(ScreenTag)
            .current_entity()
            .unwrap();
        if screen.first_load {
            commands.with(
                Style {
                    position: Rect::<Val> {
                        left: Val::Percent(120.),
                        right: Val::Undefined,
                        bottom: Val::Percent(15.),
                        top: Val::Undefined,
                    },
                    ..panel_style
                }
                .ease_to(
                    panel_style,
                    bevy_easings::EaseFunction::BounceOut,
                    bevy_easings::EasingType::Once {
                        duration: std::time::Duration::from_millis(800),
                    },
                ),
            );
        } else {
            commands.with(panel_style);
        }

        screen.loaded = true;
        screen.first_load = false;
    }
}

fn tear_down(
    mut commands: Commands,
    game_screen: Res<crate::GameScreen>,
    mut screen: ResMut<Screen>,
    query: Query<(Entity, &ScreenTag)>,
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
    mut screen: ResMut<Screen>,
    keyboard_input: Res<Input<KeyCode>>,
    mut wnds: ResMut<Windows>,
) {
    if game_screen.current_screen == CURRENT_SCREEN && screen.loaded {
        if keyboard_input.just_released(KeyCode::Escape) {
            game_screen.current_screen = crate::Screen::Exit;
        } else if keyboard_input.just_released(KeyCode::F) {
            let window = wnds.get_primary_mut().unwrap();
            match window.mode() {
                bevy::window::WindowMode::Windowed => {
                    window.set_mode(bevy::window::WindowMode::BorderlessFullscreen)
                }
                _ => window.set_mode(bevy::window::WindowMode::Windowed),
            }
        } else if keyboard_input.just_released(KeyCode::Down) {
            screen.menu_selected = Some(
                screen
                    .menu_selected
                    .map(|i| i32::min(2, i + 1))
                    .unwrap_or(0),
            );
        } else if keyboard_input.just_released(KeyCode::Up) {
            screen.menu_selected = Some(
                screen
                    .menu_selected
                    .map(|i| i32::max(0, i - 1))
                    .unwrap_or(0),
            );
        } else if keyboard_input.just_released(KeyCode::Space)
            || keyboard_input.just_released(KeyCode::Return)
        {
            match screen.menu_selected {
                Some(0) => game_screen.current_screen = crate::Screen::Game,
                Some(1) => game_screen.current_screen = crate::Screen::About,
                Some(2) => game_screen.current_screen = crate::Screen::Exit,
                _ => (),
            }
        }
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
    for (_button, interaction, button_id) in interaction_query.iter_mut() {
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
        for (timer, mut sprite) in query.iter_mut() {
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
                            transform: Transform {
                                translation: Vec3::new(-200., -75. + 192. / 2., 0.),
                                scale: Vec3::splat(1.2),
                                ..Default::default()
                            },
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

struct MenuItemSelector(usize);

fn display_menu_item_selector(
    screen: Res<Screen>,
    mut query: Query<(&MenuItemSelector, &mut Draw)>,
) {
    if let Some(index_selected) = screen.menu_selected {
        for (selector, mut draw) in query.iter_mut() {
            if selector.0 == index_selected as usize {
                draw.is_visible = true;
            } else {
                draw.is_visible = false;
            }
        }
    }
}
