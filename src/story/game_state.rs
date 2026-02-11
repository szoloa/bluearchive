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
    pub current_choices: Vec<inkling::Choice>,
    pub is_choosing: bool,
    pub is_choose: bool,
    pub story_ended: bool,
    pub story_end: bool,
    pub should_continue: bool,
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

    pub fn advance_story(&mut self) -> Result<()> {
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
        let line = self.line_buffer.remove(0);
        self.speak_state = SpeakerState::parser_line_content(line);
    }

    pub fn make_choice(&mut self, choice_index: usize) -> Result<()> {
        if choice_index < self.current_choices.len() {
            self.story.make_choice(choice_index)?;
            self.current_choices.clear();
            self.is_choosing = false;
            self.advance_story()?;
        }
        Ok(())
    }
}

pub struct SpeakerState {
    pub name: Option<String>,
    pub animation: Option<String>,
    pub position: Option<Vec2>,
    pub color: Option<Color>,
    pub content: String,
}

impl SpeakerState {
    fn new() -> Self {
        Self {
            name: None,
            animation: None,
            position: None,
            color: None,
            content: "".to_string(),
        }
    }

    fn parser_line_content(line: inkling::Line) -> Self {
        let mut speak_state = Self::new();
        let line_content = line.text;
        let regex = Regex::new(r"(.*?): ").unwrap();

        let character = regex.find(line_content.as_bytes());

        let mut speaker_string = String::new();
        if let Some(speaker) = character {
            speaker
                .as_bytes()
                .read_to_string(&mut speaker_string)
                .unwrap();
            speak_state.name = Some(speaker_string.replace(": ", "")).clone();
            speak_state.content = line_content.replace(&speaker_string, "");
        } else {
            speak_state.content = line_content;
        }

        if !speak_state.content.is_empty() {
            speak_state.content.pop();
        }
        if let Some(animation) = line.tags.get(0) {
            speak_state.animation = Some(animation.clone());
        }
        speak_state
    }
}
