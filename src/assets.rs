use bevy::prelude::*;

// macro_rules! load {
//     ($assets:ident, $path:expr) => {
//         $assets
//             .load_from(Box::new(include_bytes!($path).as_ref()))
//             .expect("was able to load font");
//     };
// }

macro_rules! load {
    ($assets:ident, $path:expr) => {
        $assets.load($path);
    };
}

macro_rules! colormaterial {
    ($mats:ident, $assets:ident, $path:expr) => {
        $mats.add($assets.load($path).into())
    };
    ($mats:ident, $assets:ident, $path:expr, $color:ident) => {
        $mats.add(ColorMaterial {
            texture: Some($assets.load($path)),
            color: $color,
        });
    };
}
// macro_rules! colormaterial {
//     ($mats:ident, $assets:ident, $path:expr) => {
//         $mats.add(
//             $assets
//                 .load_from(Box::new(include_bytes!($path).as_ref()))
//                 .expect("was able to load texture")
//                 .into(),
//         )
//     };
//     ($mats:ident, $assets:ident, $path:expr, $color:ident) => {
//         $mats.add(ColorMaterial {
//             texture: Some(
//                 $assets
//                     .load_from(Box::new(include_bytes!($path).as_ref()))
//                     .expect("was able to load texture"),
//             ),
//             color: $color,
//         });
//     };
// }

#[derive(Default, Clone)]
pub struct AssetHandles {
    panel_handle: Option<(
        Handle<bevy_ninepatch::NinePatchBuilder<()>>,
        Handle<Texture>,
    )>,
    button_handle: Option<Handle<crate::ui::button::Button>>,
    character_handle: Option<Handle<TextureAtlas>>,
    font_main_handle: Option<Handle<Font>>,
    font_sub_handle: Option<Handle<Font>>,
    board: Option<GameBoardHandles>,
    emotes: Option<EmoteHandles>,
    medals: Option<MedalHandles>,
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
    pub obstacle_100: Handle<ColorMaterial>,
    pub obstacle_75: Handle<ColorMaterial>,
    pub obstacle_50: Handle<ColorMaterial>,
    pub obstacle_25: Handle<ColorMaterial>,
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
    pub star: Handle<ColorMaterial>,
}

#[derive(Clone)]
pub struct EmoteHandles {
    pub alert: Handle<ColorMaterial>,
    pub anger: Handle<ColorMaterial>,
    pub exclamation: Handle<ColorMaterial>,
    pub exclamations: Handle<ColorMaterial>,
    pub face_angry: Handle<ColorMaterial>,
    pub face_happy: Handle<ColorMaterial>,
    pub face_sad: Handle<ColorMaterial>,
    pub heart: Handle<ColorMaterial>,
    pub heart_broken: Handle<ColorMaterial>,
    pub hearts: Handle<ColorMaterial>,
    pub idea: Handle<ColorMaterial>,
    pub laugh: Handle<ColorMaterial>,
    pub sleep: Handle<ColorMaterial>,
    pub sleeps: Handle<ColorMaterial>,
    pub star: Handle<ColorMaterial>,
}

#[derive(Clone)]
pub struct MedalHandles {
    pub bronze: Handle<ColorMaterial>,
    pub silver: Handle<ColorMaterial>,
    pub gold: Handle<ColorMaterial>,
}

impl AssetHandles {
    pub fn get_panel_handle(
        &mut self,
        assets: &AssetServer,
        nine_patches: &mut Assets<bevy_ninepatch::NinePatchBuilder<()>>,
    ) -> (
        Handle<bevy_ninepatch::NinePatchBuilder<()>>,
        Handle<Texture>,
    ) {
        if self.panel_handle.is_none() {
            // let panel = include_bytes!("../assets/ui/panel_blue.png");

            // let panel_texture_handle = assets.load_from(Box::new(panel.as_ref())).unwrap();
            let panel_texture_handle = assets.load("ui/panel_blue.png");
            let np = bevy_ninepatch::NinePatchBuilder::by_margins(10, 30, 10, 10);
            self.panel_handle = Some((nine_patches.add(np), panel_texture_handle));
        };
        self.panel_handle.as_ref().unwrap().clone()
    }

    pub fn get_button_handle(
        &mut self,
        assets: &AssetServer,
        mut mats: &mut ResMut<Assets<ColorMaterial>>,
        mut nine_patches: &mut Assets<bevy_ninepatch::NinePatchBuilder<()>>,

        buttons: &mut Assets<crate::ui::button::Button>,
    ) -> Handle<crate::ui::button::Button> {
        if self.button_handle.is_none() {
            // let button = include_bytes!("../assets/ui/buttonLong_beige.png");

            // let button_texture_handle = assets.load_from(Box::new(button.as_ref())).unwrap();
            let button_texture_handle = assets.load("ui/buttonLong_beige.png");
            let button = crate::ui::button::Button::setup(
                &mut mats,
                &mut nine_patches,
                button_texture_handle,
            );
            self.button_handle = Some(buttons.add(button));
        };
        self.button_handle.as_ref().unwrap().clone()
    }

    pub fn get_character_handle(
        &mut self,
        assets: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Handle<TextureAtlas> {
        if self.character_handle.is_none() {
            // let character = include_bytes!("../assets/game/character_femalePerson_sheetHD.png");
            // let character_texture_handle = assets.load_from(Box::new(character.as_ref())).unwrap();
            let character_texture_handle = assets.load("game/character_femalePerson_sheetHD.png");

            let texture_atlas =
                TextureAtlas::from_grid(character_texture_handle, Vec2::new(192., 256.), 9, 5);
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            self.character_handle = Some(texture_atlas_handle);
        };
        self.character_handle.as_ref().unwrap().clone()
    }

    pub fn get_font_main_handle(&mut self, assets: &AssetServer) -> Handle<Font> {
        if self.font_main_handle.is_none() {
            self.font_main_handle = Some(load!(assets, "../assets/fonts/kenvector_future.ttf"));
        }
        self.font_main_handle.as_ref().unwrap().clone()
    }

    pub fn get_font_sub_handle(&mut self, assets: &AssetServer) -> Handle<Font> {
        if self.font_sub_handle.is_none() {
            self.font_sub_handle = Some(load!(assets, "../assets/fonts/mandrill.ttf"));
        }
        self.font_sub_handle.as_ref().unwrap().clone()
    }

    pub fn get_board_handles(
        &mut self,
        assets: &AssetServer,
        mats: &mut Assets<ColorMaterial>,
    ) -> GameBoardHandles {
        if self.board.is_none() {
            let red_fire = Color::rgb(0.9, 0.3, 0.3);
            let red_0 = Color::rgb(1., 0.6, 0.6);
            let red_1 = Color::rgb(1., 0.4, 0.4);
            let red_2 = Color::rgb(1., 0.2, 0.2);
            let red_3 = Color::rgb(1., 0., 0.);
            let yellow = crate::ui::ColorScheme::TEXT_HIGHLIGHT;

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
                obstacle_100: colormaterial!(mats, assets, "../assets/game/rpgTile163.png", red_0),
                obstacle_75: colormaterial!(mats, assets, "../assets/game/rpgTile163.png", red_1),
                obstacle_50: colormaterial!(mats, assets, "../assets/game/rpgTile163.png", red_2),
                obstacle_25: colormaterial!(mats, assets, "../assets/game/rpgTile163.png", red_3),
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
                star: colormaterial!(mats, assets, "../assets/game/star.png", yellow),
            })
        }
        self.board.as_ref().unwrap().clone()
    }

    pub fn get_board_handles_unsafe(&self) -> GameBoardHandles {
        self.board.as_ref().unwrap().clone()
    }

    pub fn get_emote_handles(
        &mut self,
        assets: &AssetServer,
        mats: &mut Assets<ColorMaterial>,
    ) -> EmoteHandles {
        if self.emotes.is_none() {
            self.emotes = Some(EmoteHandles {
                alert: colormaterial!(mats, assets, "../assets/emote/emote_alert.png"),
                anger: colormaterial!(mats, assets, "../assets/emote/emote_anger.png"),
                exclamation: colormaterial!(mats, assets, "../assets/emote/emote_exclamation.png"),
                exclamations: colormaterial!(
                    mats,
                    assets,
                    "../assets/emote/emote_exclamations.png"
                ),
                face_angry: colormaterial!(mats, assets, "../assets/emote/emote_faceAngry.png"),
                face_happy: colormaterial!(mats, assets, "../assets/emote/emote_faceHappy.png"),
                face_sad: colormaterial!(mats, assets, "../assets/emote/emote_faceSad.png"),
                heart: colormaterial!(mats, assets, "../assets/emote/emote_heart.png"),
                heart_broken: colormaterial!(mats, assets, "../assets/emote/emote_heartBroken.png"),
                hearts: colormaterial!(mats, assets, "../assets/emote/emote_hearts.png"),
                idea: colormaterial!(mats, assets, "../assets/emote/emote_idea.png"),
                laugh: colormaterial!(mats, assets, "../assets/emote/emote_laugh.png"),
                sleep: colormaterial!(mats, assets, "../assets/emote/emote_sleep.png"),
                sleeps: colormaterial!(mats, assets, "../assets/emote/emote_sleeps.png"),
                star: colormaterial!(mats, assets, "../assets/emote/emote_star.png"),
            });
        }
        self.emotes.as_ref().unwrap().clone()
    }

    pub fn get_emote_handles_unsafe(&self) -> EmoteHandles {
        self.emotes.as_ref().unwrap().clone()
    }

    pub fn get_medal_handles(
        &mut self,
        assets: &AssetServer,
        mats: &mut Assets<ColorMaterial>,
    ) -> MedalHandles {
        if self.medals.is_none() {
            self.medals = Some(MedalHandles {
                bronze: colormaterial!(mats, assets, "../assets/medals/flat_medal2.png"),
                silver: colormaterial!(mats, assets, "../assets/medals/flat_medal3.png"),
                gold: colormaterial!(mats, assets, "../assets/medals/flat_medal3.png"),
            });
        }
        self.medals.as_ref().unwrap().clone()
    }
}
