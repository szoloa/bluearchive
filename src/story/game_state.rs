use crate::story::sprite::SpriteAtlas;
use crate::story::ui::*;
use crate::story::{TextureManager, character::CharacterManager};
pub use anyhow::Result;
pub use inkling::{Prompt, Story, read_story_from_string};
use macroquad::audio::Sound;
use macroquad::prelude::*;
use regex::bytes::Regex;
pub use std::fs;
use std::io::Read;

fn draw_dialog_box(
    gradient_height: f32,
    speaker: Option<&String>,
    current_text: &str,
    font: Option<&Font>,
) {
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
    let text_x = 120.0;
    let text_y = screen_height() - gradient_height / 2.0 - gradient_height / 6.0;
    let max_text_width = screen_width() - 240.0; // 屏幕宽度减去左右边距
    let font_size = 32.0;
    if let Some(speaker_name) = speaker {
        draw_text_ex(
            speaker_name,
            120.0,
            screen_height() - gradient_height,
            TextParams {
                font: font,      // 指定字体
                font_size: 48,   // 字体大小
                font_scale: 1.0, // 缩放因子
                // font_scale_aspect: 1.0,
                color: WHITE,
                ..Default::default() // 其他参数保持默认
            },
        );
    }
    // 绘制带自动换行的文本
    draw_text_wrapped(
        current_text,
        text_x,
        text_y,
        font_size,
        max_text_width,
        WHITE,
        font,
    );
}

fn draw_chioce(
    current_choices: &Vec<inkling::Choice>,
    texture: &Texture2D,
    material: Option<&Material>,
    font: Option<&Font>,
) {
    for (i, choice) in current_choices.iter().enumerate() {
        let y_pos = screen_height() / 3.0 + (i as f32 * 84.0);
        let x_pos = screen_width() / 2.0 - 200.;
        if let Some(material) = material {
            gl_use_material(material);
        }
        draw_texture_ex(
            texture,
            screen_width() * 0.1 + (i as f32 * 16.),
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
                font: font,      // 指定字体
                font_size: 30,   // 字体大小
                font_scale: 1.0, // 缩放因子
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

fn draw_background(texture: &Texture2D) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // 3. 绘制图片，尺寸设置为整个窗口
    draw_texture_ex(
        texture,
        0.0, // 从左上角(0,0)开始绘制
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_width, screen_height)), // 关键：设置目标尺寸为全屏
            ..Default::default()
        },
    );
}

pub async fn draw_frame(state: &GameState<'_>) {
    if let Some(background) = &state.get_background() {
        draw_background(background);
    } else {
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
    }

    let gradient_height = screen_height() * 0.25; // 遮罩占屏幕1/4高度

    let texture = state
        .textures
        .as_ref()
        .unwrap()
        .get("chioce_box")
        .unwrap_or_else(|| panic!("can not got chiocebox texture. "));

    if let Some(character) = &state.speak_state.name {
        for i in state.character_manager.get_meshs(character) {
            draw_mesh(&i);
        }
    }

    if !state.speak_state.content.is_empty() {
        draw_dialog_box(
            gradient_height,
            state.speak_state.name.as_ref(),
            &state.speak_state.content,
            state.font.as_ref(),
        );
    }

    // 显示选项（如果有）
    if state.is_choosing {
        draw_chioce(
            &state.current_choices,
            texture,
            state.material,
            state.font.as_ref(),
        );
    }
}
pub struct GameState<'a> {
    story: Story,
    pub speak_state: SpeakerState,
    line_buffer: Vec<inkling::Line>,
    current_choices: Vec<inkling::Choice>,
    is_choosing: bool,
    is_choose: bool,
    pub story_ended: bool,
    pub story_end: bool,
    should_continue: bool,
    pub background: Option<String>,
    pub character_manager: CharacterManager,
    pub font: Option<Font>,
    pub textures: Option<TextureManager>,
    pub material: Option<&'a Material>,
    pub sound: Option<Sound>,
    pub atlas: Option<SpriteAtlas>,
}

impl<'a> std::fmt::Debug for GameState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "is_choose = {}, is_choosing = {}, is_end = {}, is_ended = {}, should_continue = {}",
            self.is_choose,
            self.is_choosing,
            self.story_end,
            self.story_ended,
            self.should_continue
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
            speak_state: SpeakerState::new(),
            line_buffer,
            current_choices: Vec::new(),
            is_choosing: false,
            is_choose: false,
            story_ended: false,
            story_end: false,
            should_continue: true,
            background: None,
            // material: material,
            character_manager: character_manager,
            font: None,
            textures: None,
            material: None,
            sound: None,
            atlas: None,
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
        if self.line_buffer.is_empty() {
            if self.story_end {
                self.story_ended = true;
                return Ok(());
            }
            match self.story.resume(&mut self.line_buffer)? {
                Prompt::Done => {
                    println!("Story will be Done");
                    self.story_end = true;
                }
                Prompt::Choice(choices) => {
                    println!("Story will have choices");
                    self.is_choose = true;
                    self.current_choices = choices;
                }
            }
        }

        if !self.line_buffer.is_empty() {
            self.update_display_text();
            self.should_continue = true;
        } else if self.is_choose {
            println!("is choosed");
            self.is_choosing = true;
            self.is_choose = false;
        }
        Ok(())
    }

    fn update_display_text(&mut self) {
        self.speak_state.content.clear();

        debug!("{:?}", self.line_buffer);
        debug!("{:?}", self);
        let line = self.line_buffer.remove(0).text;
        self.speak_state = SpeakerState::parser_line_content(line);
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

pub struct SpeakerState {
    pub name: Option<String>,
    pub position: Option<Vec2>,
    pub color: Option<Color>,
    pub content: String,
}

impl SpeakerState {
    fn new() -> Self {
        Self {
            name: None,
            position: None,
            color: None,
            content: "".to_string(),
        }
    }

    fn parser_line_content(line: String) -> Self {
        let mut speak_state = Self::new();
        let regex = Regex::new(r"(.*?): ").unwrap();

        let character = regex.find(line.as_bytes());
        if let Some(speaker) = character {
            let mut speaker_string = String::new();
            speaker
                .as_bytes()
                .read_to_string(&mut speaker_string)
                .unwrap();
            speak_state.name = Some(speaker_string.replace(": ", ""));
            speak_state.content = line.replace(&speaker_string, "");
        } else {
            speak_state.content = line;
        }

        if !speak_state.content.is_empty() {
            speak_state.content.pop();
        }
        speak_state
    }
}
