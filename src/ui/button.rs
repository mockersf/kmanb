use bevy::prelude::*;

#[derive(bevy::type_registry::TypeUuid)]
#[uuid = "5114f317-f6a6-4436-bd2a-cb380f5eb551"]
pub struct Button {
    background: Handle<ColorMaterial>,
    nine_patch: Handle<bevy_ninepatch::NinePatchBuilder<()>>,
    texture: Handle<Texture>,
}

pub struct ButtonId<T: Into<String>>(pub T);

impl Button {
    pub fn setup(
        materials: &mut Assets<ColorMaterial>,
        nine_patches: &mut Assets<bevy_ninepatch::NinePatchBuilder>,
        texture_handle: Handle<Texture>,
    ) -> Button {
        let nine_patch = bevy_ninepatch::NinePatchBuilder::by_margins(7, 7, 7, 7);
        Button {
            background: materials.add(Color::NONE.into()),
            nine_patch: nine_patches.add(nine_patch),
            texture: texture_handle,
        }
    }

    pub fn add<T>(
        &self,
        commands: &mut Commands,
        width: f32,
        height: f32,
        margin: Rect<Val>,
        font: Handle<Font>,
        button: T,
        font_size: f32,
    ) -> Entity
    where
        T: Into<String> + Send + Sync + Copy + 'static,
    {
        let button_entity = commands
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
                material: self.background.clone(),
                ..Default::default()
            })
            .with(ButtonId(button))
            .current_entity()
            .unwrap();

        let button_content = commands
            .spawn(TextComponents {
                style: Style {
                    size: Size {
                        height: Val::Px(font_size),
                        ..Default::default()
                    },
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
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
            .with(bevy::ui::FocusPolicy::Pass)
            .current_entity()
            .unwrap();

        let patch_entity = commands
            .spawn(bevy_ninepatch::NinePatchComponents::<()> {
                style: Style {
                    margin: Rect::all(Val::Auto),
                    size: Size::new(Val::Px(width), Val::Px(height)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                nine_patch_data: bevy_ninepatch::NinePatchData::with_single_content(
                    self.texture.clone(),
                    self.nine_patch.clone(),
                    button_content,
                ),
                ..Default::default()
            })
            .current_entity()
            .unwrap();

        let interaction_overlay = commands
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
                material: self.background.clone(),
                ..Default::default()
            })
            .with(bevy::ui::FocusPolicy::Pass)
            .current_entity()
            .unwrap();

        commands.push_children(button_entity, &[patch_entity, interaction_overlay]);

        button_entity
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
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

pub struct Plugin;
impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_asset::<Button>()
            .add_system(button_effect.system());
    }
}
