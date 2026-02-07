use macroquad::prelude::*;

use crate::{
    spine::{SkeletonPath, SpineDemo},
    story::{GameState, TextureManager, sprite::SpriteAtlas},
};
use anyhow::Result;

pub async fn load_resource<'a>() -> Result<GameState<'a>> {
    let mut state = GameState::new()?;
    let mut textures = TextureManager::new();

    let font = load_ttf_font("assets/NotoSansSC-Medium.otf").await.unwrap();
    textures
        .load_texture_auto("assets/BG_AbandonedCorridor.png", None)
        .await
        .expect("background loaded error");
    println!("load success");

    // let sound = load_sound("assets/Track_64_Mitsukiyo_Pixel_time.ogg").await.unwrap();
    // play_sound(&sound, PlaySoundParams {looped:true, volume:1.0});

    let character = "NP0100";
    let path = format!(
        "/home/kina/temp/code/rust/spine/data/{}/{}",
        character, character
    );
    let texture_path = format!("{}_spr.png", path);

    let texture = load_texture(&texture_path)
        .await
        .unwrap_or_else(|e| panic!("Can not load character texture. {:?}", e));
    let atlas_path = format!("{}_spr.atlas", path);
    let skel_path = format!("{}_spr.skel", path);

    let spine_demo = SpineDemo::new(
        atlas_path,
        SkeletonPath::Binary(skel_path),
        "00".to_string(),
    );
    state
        .character_manager
        .register(character.to_string(), texture, spine_demo)
        .await
        .unwrap();
    textures
        .load_texture_auto(
            "/home/kina/temp/code/rust/spine/Texture2D/Growth_Bg.png",
            Some("chioce_box"),
        )
        .await
        .unwrap_or_else(|e| panic!("Can not load chioce box.{:?}", e));

    let atlas = SpriteAtlas::load("assets/Common.png", "assets/CommonAtlas.yaml").await;
    println!("Loaded atlas with {} sprites.", atlas.sprites.len());
    state.background = Some(String::from("assets/BG_AbandonedCorridor.png"));
    state.textures = Some(textures);

    state.font = Some(font);

    Ok(state)
}
