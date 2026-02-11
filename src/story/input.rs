// 创建输入处理模块
use macroquad::prelude::*;

use crate::story::GameState;
use anyhow::Result;

pub struct InputHandler {
    debounce_time: f32,
    last_click_time: f32,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            debounce_time: 0.3, // 300ms 防抖
            last_click_time: 0.0,
        }
    }

    pub fn should_advance(&mut self, current_time: f32) -> bool {
        let mouse_click = is_mouse_button_pressed(MouseButton::Left);
        let space_press = is_key_pressed(KeyCode::Space);

        if mouse_click {
            // 防抖处理
            if current_time - self.last_click_time > self.debounce_time {
                self.last_click_time = current_time;
                true
            } else {
                false
            }
        } else {
            space_press
        }
    }
}

pub fn handle_input(state: &mut GameState) -> Result<()> {
    // 空格键推进故事（当不在选择状态时）
    if is_key_pressed(KeyCode::Space) && !state.is_choosing && state.should_continue {
        state.should_continue = false;
        state.advance_story()?;
    }

    // 鼠标点击继续（当不在选择状态时）
    if is_mouse_button_pressed(MouseButton::Left) && !state.is_choosing && state.should_continue {
        state.should_continue = false;
        state.advance_story()?;
    }

    // 处理选择（数字键1-9）
    if state.is_choosing {
        for i in 0..state.current_choices.len().min(9) {
            let key_code = match i {
                0 => KeyCode::Key1,
                1 => KeyCode::Key2,
                2 => KeyCode::Key3,
                3 => KeyCode::Key4,
                4 => KeyCode::Key5,
                5 => KeyCode::Key6,
                6 => KeyCode::Key7,
                7 => KeyCode::Key8,
                8 => KeyCode::Key9,
                _ => continue,
            };

            if is_key_pressed(key_code) {
                state.make_choice(i)?;
                break;
            }
        }

        // 鼠标点击选择
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            for (i, choice) in state.current_choices.iter().enumerate() {
                println!("{:?}", choice);

                let y_pos = screen_height() / 2.0 - (i as f32 * 84.0) - 34.;

                // 简单的点击区域检测
                if mouse_y >= y_pos && mouse_y < y_pos + 30.0 {
                    if mouse_x >= 40.0 && mouse_x < screen_width() - 40. {
                        state.make_choice(i)?;
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
