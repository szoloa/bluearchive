use std::{collections::HashMap, io::Read, panic};

use macroquad::{audio::load_sound, prelude::*};

use crate::{
    spine::{SkeletonPath, SpineDemo},
    story::{GameState, TextureManager, character::CharacterManager, sprite::SpriteAtlas},
};
use anyhow::Result;
use std::fs::File;

fn get_characters(inkling_path: &str) -> Vec<String> {
    let mut characters_name = Vec::new();
    let mut f = File::open(inkling_path).expect(&format!("Can not found {}.", inkling_path));
    let mut content = String::new();
    f.read_to_string(&mut content)
        .unwrap_or_else(|e| panic!("Can not read file {}. {:?}", inkling_path, e));
    for i in content.lines() {
        let line: Vec<&str> = i.split(":").collect();
        if line.len() == 2 {
            if let Some(character) = line.get(0) {
                match characters_name.iter().find(|&x| x == character) {
                    None => characters_name.push(character.to_string()),
                    _ => (),
                }
            }
        }
    }
    characters_name
}

async fn load_characters(
    character_manager: &mut CharacterManager,
    character_name: &str,
    character_path: &str,
) {
    println!("Start load character {}.", character_name);
    let path = format!(
        "/home/kina/temp/code/rust/spine/data/{}/{}",
        character_path, character_path
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
    character_manager
        .register(character_name.to_string(), texture, spine_demo)
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Can not load character {} which local in {}. {:?}",
                character_name, character_path, e
            )
        });
    println!("Load character {} texture success.", character_name);
}

pub async fn load_resource<'a>() -> Result<GameState<'a>> {
    let mut state = GameState::new()?;
    let mut textures = TextureManager::new();
    println!("Resource load start.");

    let font = load_ttf_font("assets/NotoSansSC-Medium.otf").await.unwrap();
    println!("Load font success.");
    textures
        .load_texture_auto("assets/BG_AbandonedCorridor.png", None)
        .await
        .expect("background loaded error");
    println!("load backgrund success.");
    // let sound = load_sound("assets/Track_64_Mitsukiyo_Pixel_time.ogg")
    //     .await
    //     .unwrap_or_else(|_| panic!("load sound failed"));
    println!("Load sound success.");
    let mut characters_path = HashMap::new();
    characters_path.insert(String::from("圣园未花"), String::from("NP0100"));
    characters_path.insert(String::from("亚子"), String::from("ako"));
    characters_path.insert(String::from("阿罗娜"), String::from("arona"));
    characters_path.insert(String::from("普拉娜"), String::from("NP0035"));
    characters_path.insert(String::from("早濑优香"), String::from("yuuka"));
    characters_path.insert(String::from("黑见芹香"), String::from("serika"));
    let characters = get_characters("assets/story.ink");
    for name in characters {
        if let Some(find) = characters_path.iter().find(|&x| x.0 == &name) {
            load_characters(&mut state.character_manager, find.0, find.1).await;
        } else {
            load_characters(&mut state.character_manager, &name, &name).await;
        }
    }
    textures
        .load_texture_auto(
            "/home/kina/temp/code/rust/spine/Texture2D/Growth_Bg.png",
            Some("chioce_box"),
        )
        .await
        .unwrap_or_else(|e| panic!("Can not load chioce box.{:?}", e));
    println!("Load chioce box texture success.");

    let atlas = SpriteAtlas::load("assets/Common.png", "assets/CommonAtlas.yaml").await;
    println!("Loaded atlas with {} sprites.", atlas.sprites.len());
    state.background = Some(String::from("assets/BG_AbandonedCorridor.png"));
    state.textures = Some(textures);

    state.font = Some(font);
    // state.sound = Some(sound);
    state.atlas = Some(atlas);

    Ok(state)
}
