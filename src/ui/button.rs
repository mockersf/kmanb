use bevy::prelude::*;

pub struct Button {
    background: Handle<ColorMaterial>,
    nine_patch: bevy_ninepatch::NinePatch<()>,
}

pub struct ButtonId<T: Into<String>>(pub T);

impl Button {
    pub fn setup(
        materials: &mut ResMut<Assets<ColorMaterial>>,
        textures: &mut ResMut<Assets<Texture>>,
        texture_handle: Handle<Texture>,
    ) -> Button {
        let nine_patch = bevy_ninepatch::NinePatchBuilder::by_margins(7., 7., 7., 7., ()).apply(
            texture_handle,
            textures,
            materials,
        );
        Button {
            background: materials.add(Color::NONE.into()),
            nine_patch,
        }
    }

    pub fn add<T>(
        &self,
        parent: &mut ChildBuilder,
        width: f32,
        height: f32,
        margin: Rect<Val>,
        font: Handle<Font>,
        button: T,
        font_size: f32,
    ) where
        T: Into<String> + Send + Sync + Copy + 'static,
    {
        parent
            .spawn(ButtonComponents {
                style: Style {
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    margin,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                draw: Draw {
                    is_transparent: true,
                    ..Default::default()
                },
                material: self.background,
                ..Default::default()
            })
            .with(ButtonId(button))
            .with_children(|button_parent| {
                self.nine_patch
                    .add(button_parent, width, height, |inside, _| {
                        inside
                            .spawn(NodeComponents {
                                style: Style {
                                    margin: Rect::all(Val::Auto),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                material: self.background,
                                draw: Draw {
                                    is_transparent: true,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with(bevy::ui::FocusPolicy::Pass)
                            .with_children(|centered_inside| {
                                centered_inside
                                    .spawn(TextComponents {
                                        text: Text {
                                            value: button.into(),
                                            font,
                                            style: TextStyle {
                                                font_size,
                                                color: crate::ui::ColorScheme::TEXT_DARK,
                                            },
                                        },
                                        focus_policy: bevy::ui::FocusPolicy::Pass,
                                        ..Default::default()
                                    })
                                    .with(bevy::ui::FocusPolicy::Pass);
                            });
                    });
                button_parent
                    .spawn(ImageComponents {
                        style: Style {
                            position_type: PositionType::Absolute,
                            margin: Rect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            size: Size::new(Val::Px(width), Val::Px(height)),

                            ..Default::default()
                        },
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        material: self.background,
                        ..Default::default()
                    })
                    .with(bevy::ui::FocusPolicy::Pass);
            });
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::NONE.into()),
            hovered: materials.add(Color::rgba(0., 0.2, 0.2, 0.3).into()),
            pressed: materials.add(Color::rgba(0., 0.2, 0.2, 0.6).into()),
        }
    }
}

fn button_effect(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(&bevy::ui::widget::Button, Mutated<Interaction>, &Children)>,
    image_query: Query<&mut Handle<ColorMaterial>>,
) {
    for (_button, interaction, children) in &mut interaction_query.iter() {
        let mut material = image_query
            .get_mut::<Handle<ColorMaterial>>(children[children.len() - 1])
            .unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
            }
        }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system(button_effect.system());
    }
}
