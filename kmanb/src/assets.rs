use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct AssetHandles {
    panel_handle: Option<Handle<bevy_ninepatch::NinePatch<()>>>,
    button_handle: Option<Handle<crate::ui::button::Button>>,
    character_handle: Option<Handle<TextureAtlas>>,
    font_main_handle: Option<Handle<Font>>,
    font_sub_handle: Option<Handle<Font>>,
    board: Option<GameBoardHandles>,
}

#[derive(Clone)]
pub struct GameBoardHandles {
    pub ground_handle: Handle<ColorMaterial>,
    pub ground_top_handle: Handle<ColorMaterial>,
    pub border_top_handle: Handle<ColorMaterial>,
    pub ground_bottom_handle: Handle<ColorMaterial>,
    pub border_bottom_handle: Handle<ColorMaterial>,
    pub ground_left_handle: Handle<ColorMaterial>,
    pub ground_right_handle: Handle<ColorMaterial>,
    pub corner_top_left_handle: Handle<ColorMaterial>,
    pub corner_top_right_handle: Handle<ColorMaterial>,
    pub corner_bottom_left_handle: Handle<ColorMaterial>,
    pub corner_bottom_right_handle: Handle<ColorMaterial>,
    pub water_handle: Handle<ColorMaterial>,
    pub laser_handle: Handle<ColorMaterial>,
}

impl AssetHandles {
    pub fn get_panel_handle(
        &mut self,
        asset_server: &Res<AssetServer>,
        mut textures: &mut ResMut<Assets<Texture>>,
        nine_patches: &mut ResMut<Assets<bevy_ninepatch::NinePatch<()>>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Handle<bevy_ninepatch::NinePatch<()>> {
        if self.panel_handle.is_none() {
            let panel = include_bytes!("../assets/ui/panel_blue.png");

            let panel_texture_handle = asset_server
                .load_sync_from(&mut textures, &mut panel.as_ref())
                .unwrap();
            let np = bevy_ninepatch::NinePatchBuilder::by_margins(10., 10., 10., 10., ()).apply(
                panel_texture_handle,
                &mut textures,
                &mut materials,
            );
            self.panel_handle = Some(nine_patches.add(np));
        };
        self.panel_handle.unwrap()
    }

    pub fn get_button_handle(
        &mut self,
        asset_server: &Res<AssetServer>,
        mut textures: &mut ResMut<Assets<Texture>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        buttons: &mut ResMut<Assets<crate::ui::button::Button>>,
    ) -> Handle<crate::ui::button::Button> {
        if self.button_handle.is_none() {
            let button = include_bytes!("../assets/ui/buttonLong_beige.png");

            let button_texture_handle = asset_server
                .load_sync_from(&mut textures, &mut button.as_ref())
                .unwrap();
            let button = crate::ui::button::Button::setup(
                &mut materials,
                &mut textures,
                button_texture_handle,
            );
            self.button_handle = Some(buttons.add(button));
        };
        self.button_handle.unwrap()
    }

    pub fn get_character_handle(
        &mut self,
        asset_server: &Res<AssetServer>,
        mut textures: &mut ResMut<Assets<Texture>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        if self.character_handle.is_none() {
            let character = include_bytes!("../assets/game/character_femalePerson_sheetHD.png");
            let character_texture_handle = asset_server
                .load_sync_from(&mut textures, &mut character.as_ref())
                .unwrap();

            let texture = textures.get(&character_texture_handle).unwrap();
            let texture_atlas =
                TextureAtlas::from_grid(character_texture_handle, texture.size, 9, 5);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            self.character_handle = Some(texture_atlas_handle);
        };
        self.character_handle.unwrap()
    }

    pub fn get_font_main_handle(&mut self, asset_server: &Res<AssetServer>) -> Handle<Font> {
        if self.font_main_handle.is_none() {
            let font = include_bytes!("../assets/fonts/kenvector_future.ttf");

            let font: Handle<Font> = asset_server
                .load_from(Box::new(font.as_ref()))
                .expect("was able to load font");
            self.font_main_handle = Some(font);
        }
        self.font_main_handle.unwrap()
    }

    pub fn get_font_sub_handle(&mut self, asset_server: &Res<AssetServer>) -> Handle<Font> {
        if self.font_sub_handle.is_none() {
            let font = include_bytes!("../assets/fonts/mandrill.ttf");

            let font: Handle<Font> = asset_server
                .load_from(Box::new(font.as_ref()))
                .expect("was able to load font");
            self.font_sub_handle = Some(font);
        }
        self.font_sub_handle.unwrap()
    }

    pub fn get_board_handles(
        &mut self,
        asset_server: &Res<AssetServer>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> GameBoardHandles {
        if self.board.is_none() {
            let ground = include_bytes!("../assets/game/rpgTile024.png");
            let ground_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(ground.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            let ground_top = include_bytes!("../assets/game/rpgTile006.png");
            let ground_top_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(ground_top.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );
            let border_top = include_bytes!("../assets/game/rpgTile045.png");
            let border_top_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(border_top.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            let ground_bottom = include_bytes!("../assets/game/rpgTile042.png");
            let ground_bottom_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(ground_bottom.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );
            let border_bottom = include_bytes!("../assets/game/rpgTile011.png");
            let border_bottom_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(border_bottom.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            let ground_left = include_bytes!("../assets/game/rpgTile023.png");
            let ground_left_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(ground_left.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );
            let ground_right = include_bytes!("../assets/game/rpgTile025.png");
            let ground_right_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(ground_right.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            let corner_top_left = include_bytes!("../assets/game/rpgTile005.png");
            let corner_top_left_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(corner_top_left.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );
            let corner_top_right = include_bytes!("../assets/game/rpgTile007.png");
            let corner_top_right_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(corner_top_right.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );
            let corner_bottom_left = include_bytes!("../assets/game/rpgTile041.png");
            let corner_bottom_left_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(corner_bottom_left.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );
            let corner_bottom_right = include_bytes!("../assets/game/rpgTile043.png");
            let corner_bottom_right_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(corner_bottom_right.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            let water = include_bytes!("../assets/game/rpgTile029.png");
            let water_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(water.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            let laser = include_bytes!("../assets/game/spark_06.png");
            let laser_handle: Handle<ColorMaterial> = materials.add(
                asset_server
                    .load_from(Box::new(laser.as_ref()))
                    .expect("was able to load texture")
                    .into(),
            );

            self.board = Some(GameBoardHandles {
                ground_handle,
                ground_bottom_handle,
                border_bottom_handle,
                ground_top_handle,
                border_top_handle,
                ground_left_handle,
                ground_right_handle,
                corner_bottom_left_handle,
                corner_bottom_right_handle,
                corner_top_left_handle,
                corner_top_right_handle,
                water_handle,
                laser_handle,
            })
        }
        self.board.as_ref().unwrap().clone()
    }
}
