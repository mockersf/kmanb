use bevy::prelude::*;

macro_rules! load {
    ($assets:ident, $path:expr) => {
        $assets
            .load_from(Box::new(include_bytes!($path).as_ref()))
            .expect("was able to load font");
    };
}

macro_rules! colormaterial {
    ($mats:ident, $assets:ident, $path:expr) => {
        $mats.add(
            $assets
                .load_from(Box::new(include_bytes!($path).as_ref()))
                .expect("was able to load texture")
                .into(),
        )
    };
    ($mats:ident, $assets:ident, $path:expr, $color:ident) => {
        $mats.add(ColorMaterial {
            texture: Some(
                $assets
                    .load_from(Box::new(include_bytes!($path).as_ref()))
                    .expect("was able to load texture"),
            ),
            color: $color,
        });
    };
}

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
    pub ground: Handle<ColorMaterial>,
    pub ground_top: Handle<ColorMaterial>,
    pub border_top: Handle<ColorMaterial>,
    pub ground_bottom: Handle<ColorMaterial>,
    pub border_bottom: Handle<ColorMaterial>,
    pub ground_left: Handle<ColorMaterial>,
    pub ground_right: Handle<ColorMaterial>,
    pub corner_top_left: Handle<ColorMaterial>,
    pub corner_top_right: Handle<ColorMaterial>,
    pub corner_bottom_left: Handle<ColorMaterial>,
    pub corner_bottom_right: Handle<ColorMaterial>,
    pub water: Handle<ColorMaterial>,
    pub grass: Handle<ColorMaterial>,
    pub laser: Handle<ColorMaterial>,
    pub obstacle: Handle<ColorMaterial>,
    pub bomb: Handle<ColorMaterial>,
    pub bomb_icon: Handle<ColorMaterial>,
    pub fire: Handle<ColorMaterial>,
    pub powerup_score: Handle<ColorMaterial>,
    pub powerup_bomb_count: Handle<ColorMaterial>,
    pub powerup_bomb_range: Handle<ColorMaterial>,
    pub powerup_bomb_damage: Handle<ColorMaterial>,
    pub powerup_bomb_speed: Handle<ColorMaterial>,
    pub arrow_left: Handle<ColorMaterial>,
    pub arrow_right: Handle<ColorMaterial>,
}

impl AssetHandles {
    pub fn get_panel_handle(
        &mut self,
        assets: &AssetServer,
        mut textures: &mut ResMut<Assets<Texture>>,
        nine_patches: &mut Assets<bevy_ninepatch::NinePatch<()>>,
        mut mats: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Handle<bevy_ninepatch::NinePatch<()>> {
        if self.panel_handle.is_none() {
            let panel = include_bytes!("../assets/ui/panel_blue.png");

            let panel_texture_handle = assets
                .load_sync_from(&mut textures, &mut panel.as_ref())
                .unwrap();
            let np = bevy_ninepatch::NinePatchBuilder::by_margins(10., 10., 10., 10., ()).apply(
                panel_texture_handle,
                &mut textures,
                &mut mats,
            );
            self.panel_handle = Some(nine_patches.add(np));
        };
        self.panel_handle.unwrap()
    }

    pub fn get_button_handle(
        &mut self,
        assets: &AssetServer,
        mut textures: &mut ResMut<Assets<Texture>>,
        mut mats: &mut ResMut<Assets<ColorMaterial>>,
        buttons: &mut Assets<crate::ui::button::Button>,
    ) -> Handle<crate::ui::button::Button> {
        if self.button_handle.is_none() {
            let button = include_bytes!("../assets/ui/buttonLong_beige.png");

            let button_texture_handle = assets
                .load_sync_from(&mut textures, &mut button.as_ref())
                .unwrap();
            let button =
                crate::ui::button::Button::setup(&mut mats, &mut textures, button_texture_handle);
            self.button_handle = Some(buttons.add(button));
        };
        self.button_handle.unwrap()
    }

    pub fn get_character_handle(
        &mut self,
        assets: &AssetServer,
        mut textures: &mut Assets<Texture>,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Handle<TextureAtlas> {
        if self.character_handle.is_none() {
            let character = include_bytes!("../assets/game/character_femalePerson_sheetHD.png");
            let character_texture_handle = assets
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

    pub fn get_font_main_handle(&mut self, assets: &AssetServer) -> Handle<Font> {
        if self.font_main_handle.is_none() {
            self.font_main_handle = Some(load!(assets, "../assets/fonts/kenvector_future.ttf"));
        }
        self.font_main_handle.unwrap()
    }

    pub fn get_font_sub_handle(&mut self, assets: &AssetServer) -> Handle<Font> {
        if self.font_sub_handle.is_none() {
            self.font_sub_handle = Some(load!(assets, "../assets/fonts/mandrill.ttf"));
        }
        self.font_sub_handle.unwrap()
    }

    pub fn get_board_handles(
        &mut self,
        assets: &AssetServer,
        mats: &mut Assets<ColorMaterial>,
    ) -> GameBoardHandles {
        if self.board.is_none() {
            let red_fire = Color::rgb(0.9, 0.3, 0.3);

            self.board = Some(GameBoardHandles {
                ground: colormaterial!(mats, assets, "../assets/game/rpgTile024.png"),
                ground_bottom: colormaterial!(mats, assets, "../assets/game/rpgTile042.png"),
                border_bottom: colormaterial!(mats, assets, "../assets/game/rpgTile011.png"),
                ground_top: colormaterial!(mats, assets, "../assets/game/rpgTile006.png"),
                border_top: colormaterial!(mats, assets, "../assets/game/rpgTile045.png"),
                ground_left: colormaterial!(mats, assets, "../assets/game/rpgTile023.png"),
                ground_right: colormaterial!(mats, assets, "../assets/game/rpgTile025.png"),
                corner_bottom_left: colormaterial!(mats, assets, "../assets/game/rpgTile041.png"),
                corner_bottom_right: colormaterial!(mats, assets, "../assets/game/rpgTile043.png"),
                corner_top_left: colormaterial!(mats, assets, "../assets/game/rpgTile005.png"),
                corner_top_right: colormaterial!(mats, assets, "../assets/game/rpgTile007.png"),
                water: colormaterial!(mats, assets, "../assets/game/rpgTile029.png"),
                grass: colormaterial!(mats, assets, "../assets/game/rpgTile019.png"),
                laser: colormaterial!(mats, assets, "../assets/game/spark_06.png", red_fire),
                obstacle: colormaterial!(mats, assets, "../assets/game/rpgTile163.png", red_fire),
                bomb: colormaterial!(mats, assets, "../assets/game/bomb.png"),
                bomb_icon: colormaterial!(mats, assets, "../assets/game/bomb.png", red_fire),
                fire: colormaterial!(mats, assets, "../assets/game/fire_01.png", red_fire),
                powerup_score: colormaterial!(mats, assets, "../assets/game/coinGold.png"),
                powerup_bomb_count: colormaterial!(mats, assets, "../assets/game/gemBlue.png"),
                powerup_bomb_damage: colormaterial!(mats, assets, "../assets/game/gemRed.png"),
                powerup_bomb_range: colormaterial!(mats, assets, "../assets/game/gemGreen.png"),
                powerup_bomb_speed: colormaterial!(mats, assets, "../assets/game/gemYellow.png"),
                arrow_left: colormaterial!(mats, assets, "../assets/game/arrowLeft.png"),
                arrow_right: colormaterial!(mats, assets, "../assets/game/arrowRight.png"),
            })
        }
        self.board.as_ref().unwrap().clone()
    }

    pub fn get_board_handles_unsafe(&self) -> GameBoardHandles {
        self.board.as_ref().unwrap().clone()
    }
}
