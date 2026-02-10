// 创建输入处理模块
use macroquad::prelude::*;

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
