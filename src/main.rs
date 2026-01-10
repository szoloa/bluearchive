use blueachive::*;

use std::sync::{Arc, Mutex};

use glam::{Mat4, Vec2, Vec3};
use rusty_spine::{
    atlas::{AtlasFilter, AtlasFormat, AtlasWrap},
    controller::{SkeletonController, SkeletonControllerSettings},
    draw::{ColorSpace, CullDirection},
    AnimationEvent, AnimationStateData, Atlas, BlendMode, SkeletonBinary, SkeletonJson,
};


use rusty_spine::Skeleton;
use rusty_spine::AnimationState;


#[macroquad::main("GalGame")]
async fn main() -> Result<()> {
    // 初始化游戏状态
    let mut state = GameState::new()?;
    
    // 加载字体（如果需要中文支持）
    let font = load_ttf_font("assets/DouyinSansBold.ttf").await?;
    request_new_screen_size(1280.0, 900.0); 
    let mut textures  = TextureManager::new();
    textures.load_texture_auto("assets/BG_GameDevRoom.webp").await.expect("background loaded error");
    println!("load success");
    
    // let background= load_texture().await.unwrap();
    // let sound = load_sound("assets/Track_64_Mitsukiyo_Pixel_time.ogg").await.unwrap();
    // play_sound(&sound, PlaySoundParams {looped:true, volume:1.0});
    textures.load_texture_auto("assets/Nagisa_00.png").await.expect("background loaded error"); 

    println!("{:?}", textures.textures); 

    textures.load_texture_auto("assets/nagisa_spr.png").await.unwrap();

    let background = textures.get("assets/BG_GameDevRoom.webp").expect("msg");
    state.character_manager.register("Nagisa", "assets/Nagisa_00.png", &textures).await.unwrap();
    println!("{:?}", background);

    state.background = Some(background);
    state.current_speaker = Some("Nagisa".to_string());
    state.font = Some(&font); 

    // 1. 加载纹理（Spine的图集图片）
    let texture = load_texture("assets/nagisa_spr.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest); // 像素风保持锐利

    let atlas_data = std::fs::read("assets/nagisa_spr.atlas").unwrap(); 

    let atlas = Atlas::new(&atlas_data, "assets").unwrap();

    // 使用解析好的图集，读取并解析骨架数据文件
    let skeleton_json = std::fs::read("assets/nagisa_spr.json").unwrap();
    let skeleton_json_parser = SkeletonJson::new(atlas.into());
    // skeleton_json_parser.set_scale(0.5); // 可选：缩放骨架尺寸
    let skeleton_data = skeleton_json_parser
        .read_skeleton_data(&skeleton_json)
        .expect("this is wrong");
    // 游戏主循环
    while !state.story_ended {

        // 处理输入
        handle_input(&mut state)?;
        
        // 渲染
        draw_frame(&state).await;
        
        next_frame().await;
    }
    
    // 故事结束后的显示循环
    loop {
        draw_frame(&state).await;
        
        // 按ESC退出
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        next_frame().await;
    }
    
    Ok(())
}