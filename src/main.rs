use std::panic;

use anyhow::Ok;
use bluearchive::story::*;
use macroquad::{
    audio::{PlaySoundParams, play_sound},
    prelude::*,
};

#[macroquad::main("GalGame")]
async fn main() -> Result<()> {
    // 初始化游戏状态
    let mut state = load_resource().await?;

    let mut last_frame = get_time();
    let material = ui::draw_chioce_material();
    state.material = Some(&material);
    if let Some(sound) = &state.sound {
        play_sound(
            sound,
            PlaySoundParams {
                looped: true,
                volume: 1.0,
            },
        );
    }
    // 游戏主循环
    while !state.story_ended {
        // clear_background(WHITE);
        let current_time = get_time();
        let delta_time = (current_time - last_frame) as f32;
        last_frame = current_time;
        if let Some(character) = state.speak_state.name.as_ref() {
            state
                .character_manager
                .update(character, delta_time, screen_width() / 2., screen_height())
                .unwrap_or_else(|e| panic!("Can not update character. {:?}", e));
        }
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
