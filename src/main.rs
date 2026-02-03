use blueachive::spine::*;
use blueachive::story::*;
use macroquad::prelude::*;

#[macroquad::main("GalGame")]
async fn main() -> Result<()> {
    // 初始化游戏状态
    let mut state = GameState::new()?;

    // 加载字体（如果需要中文支持）
    let font = load_ttf_font("assets/DouyinSansBold.ttf").await?;
    //request_new_screen_size(1280.0, 900.0);
    let mut textures = TextureManager::new();
    textures
        .load_texture_auto("assets/BG_GameDevRoom.webp")
        .await
        .expect("background loaded error");
    println!("load success");

    // let background= load_texture().await.unwrap();
    // let sound = load_sound("assets/Track_64_Mitsukiyo_Pixel_time.ogg").await.unwrap();
    // play_sound(&sound, PlaySoundParams {looped:true, volume:1.0});

    let character = "NP0100";
    let path = format!(
        "/home/kina/temp/code/rust/spine/data/{}/{}",
        character, character
    );
    let texture_path = format!("{}_spr.png", path);

    let texture = load_texture(&texture_path).await.unwrap();
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

    let background = textures.get("assets/BG_GameDevRoom.webp").expect("msg");

    println!("{:?}", background);

    state.background = Some(background);
    state.current_speaker = Some(character.to_string());
    state.font = Some(&font);

    let mut camera = Camera2D::default();
    camera.zoom = vec2(1.0 / 650.0, 1.0 / 300.0);
    camera.target = Vec2 {
        x: screen_width(),
        y: screen_height(),
    };
    let mut last_frame = get_time();

    // 游戏主循环
    while !state.story_ended {
        let current_time = get_time();
        let delta_time = (current_time - last_frame) as f32;
        last_frame = current_time;
        state
            .character_manager
            .update(character, delta_time)
            .unwrap();

        camera.target = Vec2 {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        };
        set_camera(&camera);
        // 处理输入
        handle_input(&mut state)?;

        draw_frame(&state).await;

        next_frame().await;
    }

    // 故事结束后的显示循环
    loop {
        draw_frame(&state).await;
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
    Ok(())
}
