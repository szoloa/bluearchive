use crate::story::ui::*;
use crate::story::{TextureManager, character::CharacterManager};
pub use anyhow::Result;
pub use inkling::{Prompt, Story, read_story_from_string};
use macroquad::prelude::*;
use regex::bytes::Regex;
pub use std::fs;
use std::io::Read;

pub async fn draw_frame(state: &GameState<'_>) {
    if let Some(background) = &state.get_background() {
        draw_texture(background, 0.0, 0.0, WHITE);
    } else {
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
    }
    let current_speaker = state.current_speaker.as_ref().unwrap();
    for i in state.character_manager.get_meshs(&current_speaker) {
        draw_mesh(&i);
    }

    let gradient_height = screen_height() * 0.25; // 遮罩占屏幕1/4高度

    let texture = state
        .textures
        .as_ref()
        .unwrap()
        .get("chioce_box")
        .unwrap_or_else(|| panic!("can not got chiocebox texture. "));

    let rect = Rect::new(
        screen_width() * 0.5 - 600. * 0.5,
        screen_height() * 0.5 - 50. * 0.5,
        600.,
        50.,
    );

    draw_vertical_gradient(
        0.0,
        screen_height() - gradient_height - gradient_height / 4.0,
        screen_width(),
        gradient_height + gradient_height / 4.0,
        Color::new(0.05, 0.05, 0.1, 0.8),
        Color::new(0.05, 0.05, 0.1, 0.8),
    );

    draw_vertical_gradient(
        0.0,
        screen_height() - gradient_height - gradient_height / 4.0 - gradient_height / 2.0,
        screen_width(),
        gradient_height / 2.0,
        Color::new(0.05, 0.05, 0.1, 0.0),
        Color::new(0.05, 0.05, 0.1, 0.8),
    );

    draw_line(
        120.0,
        screen_height() - gradient_height / 2.0 - gradient_height / 4.0 - gradient_height / 8.0,
        screen_width() - 120.0,
        screen_height() - gradient_height / 2.0 - gradient_height / 4.0 - gradient_height / 8.0,
        2.0,
        GRAY,
    );

    if let Some(character) = &state.current_speaker {
        draw_text_ex(
            character,
            120.0,
            screen_height() - gradient_height,
            TextParams {
                font: state.font.as_ref(), // 指定字体
                font_size: 54,             // 字体大小
                font_scale: 1.0,           // 缩放因子
                font_scale_aspect: 1.0,
                color: WHITE,         // 颜色
                ..Default::default()  // 其他参数保持默认
            },
        );
    }

    if !state.current_text.is_empty() {
        // 定义文本框的边界
        let text_x = 120.0;
        let text_y = screen_height() - gradient_height / 2.0 - gradient_height / 6.0;
        let max_text_width = screen_width() - 280.0; // 屏幕宽度减去左右边距
        let font_size = 36.0;

        // 绘制带自动换行的文本
        draw_text_wrapped(
            &state.current_text,
            text_x,
            text_y,
            font_size,
            max_text_width,
            WHITE,
            state.font.as_ref(),
        );
    }

    // 显示选项（如果有）
    if state.is_choosing {
        for (i, choice) in state.current_choices.iter().enumerate() {
            let y_pos = screen_height() / 2.0 - (i as f32 * 84.0);
            let x_pos = screen_width() / 2.0 - 200.;
            if let Some(material) = state.material {
                gl_use_material(material);
            }
            draw_texture_ex(
                texture,
                screen_width() * 0.1 - (i as f32 * 16.),
                y_pos - 34.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: screen_width() * 0.9,
                        y: screen_height() * 0.08,
                    }),
                    ..Default::default()
                },
            );
            gl_use_default_material();

            // 选项文本
            let text = format!("{}. {}", i + 1, choice.text);

            draw_text_ex(
                &text,
                x_pos + 160.,
                y_pos + 2.,
                TextParams {
                    font: state.font.as_ref(), // 指定字体
                    font_size: 30,             // 字体大小
                    font_scale: 1.0,           // 缩放因子
                    font_scale_aspect: 1.0,
                    color: BLACK,         // 颜色
                    ..Default::default()  // 其他参数保持默认
                },
            );

            // 鼠标悬停效果
            // let mouse_pos = mouse_position();
            // let rect = Rect::new(60.0, y_pos - 48.0, 700.0, 64.0);
            // if rect.contains(vec2(mouse_pos.0, mouse_pos.1)) {
            //     draw_rectangle_lines(55.0, y_pos - 50.0, 710.0, 64.0, 2.0, WHITE);
            // }
        }
    }
}
pub struct GameState<'a> {
    story: Story,
    line_buffer: Vec<inkling::Line>,
    current_choices: Vec<inkling::Choice>,
    current_text: String,
    is_choosing: bool,
    is_choose: bool,
    pub story_ended: bool,
    pub story_end: bool,
    should_continue: bool,
    pub background: Option<String>,
    // pub material: Material,
    pub character_manager: CharacterManager,
    pub current_speaker: Option<String>, // 当前正在说话的角色名
    pub speaker_position: (f32, f32),    // 立绘绘制基准坐标（例如屏幕左侧或右侧）
    pub font: Option<Font>,
    pub textures: Option<TextureManager>,
    pub material: Option<&'a Material>,
}

impl<'a> std::fmt::Debug for GameState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "choose {}, choosing {}, end {}, ended {}",
            self.is_choose, self.is_choosing, self.story_end, self.story_ended
        )
    }
}

impl<'a> GameState<'a> {
    pub fn new() -> Result<Self> {
        // 读取故事文件
        let story_content = fs::read_to_string("assets/story.ink")?;
        let mut story = read_story_from_string(&story_content).unwrap();
        let line_buffer = Vec::new();
        let character_manager = CharacterManager::new();

        // 开始故事
        story.start()?;

        // 获取初始内容
        let mut state = Self {
            story,
            line_buffer,
            current_choices: Vec::new(),
            current_text: String::new(),
            is_choosing: false,
            is_choose: false,
            story_ended: false,
            story_end: false,
            should_continue: true,
            background: None,
            // material: material,
            character_manager: character_manager,
            current_speaker: None,
            speaker_position: (0.0, 0.0),
            font: None,
            textures: None,
            material: None,
        };

        // 处理初始内容
        state.advance_story()?;

        Ok(state)
    }
    fn get_background(&self) -> Option<&Texture2D> {
        self.textures
            .as_ref()
            .unwrap()
            .get(self.background.as_ref().unwrap().as_str())
    }

    fn advance_story(&mut self) -> Result<()> {
        // 推进故事
        match self.story.resume(&mut self.line_buffer)? {
            Prompt::Done => {
                println!("Story will be Done");
                if !self.line_buffer.is_empty() {
                    // self.update_display_text();
                    self.should_continue = true;
                }
                self.story_end = true;
            }
            Prompt::Choice(choices) => {
                println!("Story will have choices");
                if !self.line_buffer.is_empty() {
                    self.should_continue = true;
                }
                self.is_choose = true;
                self.current_choices = choices;
            }
        }

        if !self.line_buffer.is_empty() {
            self.update_display_text();
            self.should_continue = true;
            return Ok(());
        } else if self.is_choose {
            println!("is choosed");
            self.is_choosing = true;
            self.is_choose = false;
            self.should_continue = true;
            return Ok(());
        } else if self.story_end {
            self.story_ended = true;
            self.story_end = false;
            return Ok(());
        }
        Ok(())
    }

    fn update_display_text(&mut self) {
        let regex = Regex::new(r"(.*?): ").unwrap();
        self.current_text.clear();

        println!("{:?}", self.line_buffer);
        println!("{:?}", self);
        let line = self.line_buffer.remove(0).text;
        let character = regex.find(line.as_bytes());
        if let Some(speaker) = character {
            let mut speaker_string = String::new();
            speaker
                .as_bytes()
                .read_to_string(&mut speaker_string)
                .unwrap();
            self.current_speaker = Some(speaker_string.replace(": ", ""));
            self.current_text = line.replace(&speaker_string, "");
        } else {
            self.current_text = line;
        }

        if !self.current_text.is_empty() {
            self.current_text.pop();
        }
    }

    fn make_choice(&mut self, choice_index: usize) -> Result<()> {
        if choice_index < self.current_choices.len() {
            self.story.make_choice(choice_index)?;
            self.current_choices.clear();
            self.is_choosing = false;
            self.advance_story()?;
        }
        Ok(())
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

mod test {
    #[test]
    fn test1() {}
}
