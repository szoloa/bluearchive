pub use macroquad::{audio::{PlaySoundParams, load_sound, play_sound}, prelude::*};
pub use inkling::{read_story_from_string, Story, Prompt};
pub use std::fs;
pub use anyhow::Result;
pub use std::{collections::HashMap, io::Cursor};
pub use image::ImageReader;

pub struct TextureManager {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub async fn load_texture_auto(&mut self, path: &str) -> Result<()> {
        let extension = path.split('.').last().unwrap_or("").to_lowercase();
        
        match extension.as_str() {
            "png" | "bmp" | "tga" | "gif" => {
                let texture = load_texture(path).await
                    .map_err(|e| format!("加载失败 {}: {:?}", path, e)).unwrap();
                self.textures.insert(path.to_string(), texture);
                Ok(())
            },
            "jpg" | "jpeg" | "webp" => {
                let bytes = std::fs::read(path)?;
                
                let img = ImageReader::new(Cursor::new(&bytes))
                    .with_guessed_format()?
                    .decode()?
                    .to_rgba8();
                    
                let (width, height) = img.dimensions();
                let texture = Texture2D::from_rgba8(width as u16, height as u16, &img);
                self.textures.insert(path.to_string(), texture);
                Ok(())
            },
            _ => Err(anyhow::anyhow!("不支持的格式: .{}", extension)),
        }
    }
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
    pub fn get(&self, path: &str) -> Option<&Texture2D> {
        self.textures.get(path)
    }
}

#[derive(Debug)]
pub struct Character<'a> {
    pub name: Option<&'a str>,
    pub texture: Option<&'a Texture2D>, 
}

#[derive(Debug)]
pub struct CharacterManager<'a> {
    pub characters: HashMap<&'a str, Character<'a>>,
}

// CharacterManager 可以获取 TextureLoader 的实现
impl<'a> CharacterManager<'a> {
    pub async fn register(
        &mut self, 
        name: &'a str, 
        texture_path: &str,
        texture_manager: &'a TextureManager
    ) -> Result<(), String> {
        let texture = texture_manager.get(texture_path);
        self.characters.insert(name, Character { name: Some(name), texture: texture});
        Ok(())
    }
}

impl<'a> CharacterManager<'a> {
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.characters.get(name).and_then(|c| c.texture)
    }

    pub fn get_decrible(&self, name: &str) -> Option<&str> {
        self.characters.get(name).and_then(|c| c.name)
    }
}

#[derive(Debug)]
pub struct GameState<'a> {
    story: Story,
    line_buffer: Vec<inkling::Line>,
    current_choices: Vec<inkling::Choice>,
    current_text: String,
    is_choosing: bool,
    pub story_ended: bool,
    should_continue: bool,
    pub background: Option<&'a Texture2D>,
    // pub material: Material,
    pub character_manager: CharacterManager<'a>,
    pub current_speaker: Option<String>, // 当前正在说话的角色名
    pub speaker_position: (f32, f32),    // 立绘绘制基准坐标（例如屏幕左侧或右侧）
    pub font: Option<&'a Font>,
}

impl<'a> GameState<'a> {
    pub fn new() -> Result<Self> {
        // 读取故事文件
        let story_content = fs::read_to_string("assets/story.ink")?;
        let mut story = read_story_from_string(&story_content).unwrap();
        let line_buffer = Vec::new();
        let character_manager = CharacterManager::new();

        // let material = load_material(
        // ShaderSource::Glsl {
        //     vertex: VERTEX_SHADER, // 你需要提供一个基本的顶点着色器
        //     fragment: FRAGMENT_SHADER, // 上面编写的片段着色器代码
        // },
        //  MaterialParams {
        //     // 这里必须是 uniforms: vec![UniformDesc { ... }] 的形式
        //     uniforms: vec![
        //         UniformDesc::new("top_color", UniformType::Float4),
        //         UniformDesc::new("bottom_color", UniformType::Float4),
        //     ],
        //     ..Default::default()
        // },
        // ).unwrap();
        
        // 开始故事
        story.start()?;
        
        // 获取初始内容
        let mut state = Self {
            story,
            line_buffer,
            current_choices: Vec::new(),
            current_text: String::new(),
            is_choosing: false,
            story_ended: false,
            should_continue: true,
            background: None,
            // material: material, 
            character_manager: character_manager, 
            current_speaker: None, 
            speaker_position: (0.0, 0.0), 
            font: None,
        };
        
        // 处理初始内容
        state.advance_story()?;
        
        Ok(state)
    }
    
    fn advance_story(&mut self) -> Result<()> {
        if self.story_ended {
            return Ok(());
        }
        
        // 清空行缓存
        self.line_buffer.clear();
        
        // 推进故事
        match self.story.resume(&mut self.line_buffer)? {
            Prompt::Done => {
                if !self.line_buffer.is_empty() {
                    self.update_display_text();
                    self.should_continue = true;
                }
                // 故事结束
                self.story_ended = true;
                self.current_text.push_str("\n\n[story end]");
            }
            Prompt::Choice(choices) => {

                if !self.line_buffer.is_empty() {
                    self.update_display_text();
                    self.should_continue = true;
                } else {
                    self.current_choices = choices;
                    self.is_choosing = true;
                }
            }
            // _ => {
            //     // 普通文本行
            //     self.update_display_text();
            //     self.should_continue = true;
            // }
        }
        
        Ok(())
    }
    
    fn update_display_text(&mut self) {
        self.current_text.clear();
        
        
        // for line in &self.line_buffer {
        //     // for fragment in &line.text {
        //     self.current_text.push_str(&line.text);
        //     // println!("{}", &line.text);
        //     // }
        //     self.current_text.push('\n');
        // }
        self.current_text = self.line_buffer.remove(0).text; 
        println!("{}", self.current_text);
        
        // 移除最后一个多余的换行
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

                let y_pos = 300.0 + (i as f32 * 40.0);
                
                // 简单的点击区域检测
                if mouse_y >= y_pos && mouse_y < y_pos + 30.0 {
                    if mouse_x >= 40.0 && mouse_x < 600.0 {
                        state.make_choice(i)?;
                        break;
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn draw_vertical_gradient(x: f32, y: f32, width: f32, height: f32, start_color: Color, end_color: Color) {
    let steps = 64; // 数值越大越平滑
    let step_height = height / steps as f32;
    
    for i in 0..steps {
        let t = i as f32 / (steps as f32 - 1.0);
        let r = start_color.r + (end_color.r - start_color.r) * t;
        let g = start_color.g + (end_color.g - start_color.g) * t;
        let b = start_color.b + (end_color.b - start_color.b) * t;
        let a = start_color.a + (end_color.a - start_color.a) * t;
        
        let current_color = Color::new(r, g, b, a);
        draw_rectangle(
            x,
            y + i as f32 * step_height,
            width,
            step_height,
            current_color,
        );
    }
}

fn draw_text_wrapped(text: &str, x: f32, y: f32, font_size: f32, max_width: f32, color: Color, font: Option<&Font>) {
    let text_param = TextParams {
                font: font,    // 指定字体
                font_size: font_size as u16,       // 字体大小
                font_scale: 1.0,     // 缩放因子
                font_scale_aspect: 1.0,
                color: color,          // 颜色
                ..Default::default() // 其他参数保持默认
            };
    // 初始行位置
    let mut current_y = y;
    // 当前行累积的文本
    let mut current_line = String::new();
    // 当前行已使用的宽度
    let mut current_width = 0.0;
    
    // 按单词分割（英文以空格分隔，中文等通常每个字符都是“单词”）
    // 这是一个简单的实现，对于中英文混合文本，你可能需要更复杂的分词
    let words: Vec<&str> = text.split_whitespace().collect();
    
    for word in words {
        // 检查当前行是否为空
        let word_with_space = if current_line.is_empty() {
            word.to_string()
        } else {
            format!(" {}", word)
        };
        
        // 计算添加这个词后的宽度
        let word_width = measure_text(word_with_space.as_str(), None, font_size as u16, 1.0).width;
        
        // 如果当前行是空的，或者加上这个词后不超过最大宽度，就添加到当前行
        if current_line.is_empty() || current_width + word_width <= max_width {
            current_line.push_str(&word_with_space);
            current_width += word_width;
        } else {
            // 否则，绘制当前行，并开始新的一行
            draw_text_ex(current_line.as_str(), x, current_y, text_param.clone());
            current_y += font_size * 1.2; // 行高，通常是字体大小的1.2倍
            
            // 新行以当前单词开始
            current_line = word.to_string();
            current_width = measure_text(word, None, font_size as u16, 1.0).width;
        }
    }
    
    // 绘制最后一行
    if !current_line.is_empty() {
        draw_text_ex(current_line.as_str(), x, current_y, text_param.clone());
    }
}

pub async fn draw_frame(state: &GameState<'_>) {

    if let Some(background) = &state.background {
        draw_texture(&background, 0.0, 0.0, WHITE);
    } else {
        clear_background(Color::new(0.1, 0.1, 0.2, 1.0));
    } 

    if let Some(character) = &state.current_speaker {
        let character_texture = state.character_manager.get_texture(character).unwrap();
        draw_texture(&character_texture, 100.0, 100.0, GRAY);
    } 

    // let material = &state.material;

    // material.set_uniform("top_color", &[0.3_f32, 0.3, 0.3, 0.7][..]);
    // material.set_uniform("bottom_color", &[0.3_f32, 0.3, 0.3, 0.0][..]);

    let gradient_height = screen_height() * 0.25; // 遮罩占屏幕1/4高度 

    draw_vertical_gradient(
        0.0,
        screen_height() - gradient_height - gradient_height/4.0,
        screen_width(),
        gradient_height + gradient_height/4.0,
        Color::new(0.05, 0.05, 0.1, 0.8), 
        Color::new(0.05, 0.05, 0.1, 0.8), 
    );

    draw_vertical_gradient(
        0.0,
        screen_height() - gradient_height - gradient_height/4.0 - gradient_height/2.0,
        screen_width(),
        gradient_height/2.0,
        Color::new(0.05, 0.05, 0.1, 0.0), 
        Color::new(0.05, 0.05, 0.1, 0.8), 
    ); 

    draw_line(
        120.0, 
        screen_height() - gradient_height/2.0 - gradient_height/4.0 - gradient_height/8.0, 
        screen_width() - 120.0, 
        screen_height() - gradient_height/2.0 - gradient_height/4.0 - gradient_height/8.0, 
        2.0 ,  
        GRAY
    ); 

    if let Some(character) = &state.current_speaker {
        draw_text_ex(character, 120.0 , screen_height() - gradient_height, TextParams {
                font: state.font,    // 指定字体
                font_size: 54,       // 字体大小
                font_scale: 1.0,     // 缩放因子
                font_scale_aspect: 1.0,
                color: WHITE,          // 颜色
                ..Default::default() // 其他参数保持默认
            });
    } 

    if !state.current_text.is_empty() {
        // 定义文本框的边界
        let text_x = 120.0;
        let text_y = screen_height() - gradient_height/2.0 - gradient_height/6.0;
        let max_text_width = screen_width() - 280.0; // 屏幕宽度减去左右边距
        let font_size = 36.0;
        
        // 绘制带自动换行的文本
        draw_text_wrapped(&state.current_text, text_x, text_y, font_size, max_text_width, WHITE, state.font);
    }
    
    // 显示选项（如果有）
    if state.is_choosing {
        
        for (i, choice) in state.current_choices.iter().enumerate() {
            let y_pos = 320.0 + (i as f32 * 84.0);
            
            // 选项文本
            let text = format!("{}. {}", i + 1, choice.text);
            draw_text_ex(&text, 60.0, y_pos, TextParams {
                font: state.font,    // 指定字体
                font_size: 48,       // 字体大小
                font_scale: 1.0,     // 缩放因子
                font_scale_aspect: 1.0,
                color: WHITE,          // 颜色
                ..Default::default() // 其他参数保持默认
            });
            
            // 鼠标悬停效果
            let mouse_pos = mouse_position();
            let rect = Rect::new(60.0, y_pos - 48.0, 700.0, 64.0);
            if rect.contains(vec2(mouse_pos.0, mouse_pos.1)) {
                draw_rectangle_lines(55.0, y_pos - 50.0, 710.0, 64.0, 2.0, WHITE);
            }
        }
    } 

} 
