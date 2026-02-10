use macroquad::math::Rect;
use macroquad::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
pub struct SpriteDrawParams {
    pub scale: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub color: Color,
}

impl SpriteDrawParams {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            color: WHITE,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub border_left: f32,
    pub border_right: f32,
    pub border_top: f32,
    pub border_bottom: f32,
}

impl Sprite {
    pub fn uv_rect(&self, textrue_width: f32, textrue_height: f32) -> Rect {
        let left = self.x / textrue_width;
        let right = (self.x + self.width) / textrue_width;
        let top = self.y / textrue_height;
        let bottom = (self.y + self.height) / textrue_height;
        Rect::new(left, top, right - left, bottom - top)
    }
    pub fn source_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
    pub fn border_rect(&self) -> (f32, f32, f32, f32) {
        (
            self.border_left,
            self.border_right,
            self.border_top,
            self.border_bottom,
        )
    }
}

pub struct SpriteAtlas {
    pub texture: Texture2D,
    pub sprites: HashMap<String, Sprite>,
}

impl SpriteAtlas {
    pub async fn load(texture_path: &str, yaml_path: &str) -> Self {
        let texture = load_texture(texture_path).await.unwrap();
        let yaml_content = fs::read_to_string(yaml_path).unwrap();
        #[derive(Debug, Deserialize)]
        struct YamlFile {
            m_sprites: Vec<Sprite>,
        }
        let yaml_data: YamlFile = serde_yaml::from_str(&yaml_content).unwrap();
        let mut sprites = HashMap::new();
        for sprite in yaml_data.m_sprites {
            sprites.insert(sprite.name.clone(), sprite);
        }
        SpriteAtlas {
            texture: texture,
            sprites: sprites,
        }
    }
    pub fn get_sprite(&self, name: &str) -> Option<&Sprite> {
        self.sprites.get(name)
    }
    pub fn draw_sprite(
        &self,
        name: &str,
        postion: Vec2,
        params: SpriteDrawParams,
    ) -> Result<(), String> {
        if let Some(sprite) = self.get_sprite(name) {
            self.draw_sprite_definition(sprite, postion, params);
            Ok(())
        } else {
            Err("Failed to draw sprite".to_string())
        }
    }
    pub fn draw_sprite_definition(&self, sprite: &Sprite, postion: Vec2, params: SpriteDrawParams) {
        let src_rect = sprite.source_rect();
        let dest_rect = Rect::new(
            postion.x,
            postion.y,
            sprite.width * params.scale,
            sprite.height * params.scale,
        );
        draw_texture_ex(
            &self.texture,
            dest_rect.x,
            dest_rect.y,
            params.color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(dest_rect.x, dest_rect.y)),
                source: Some(Rect::new(src_rect.x, src_rect.y, src_rect.w, src_rect.h)),
                rotation: params.rotation,
                flip_x: params.flip_x,
                flip_y: params.flip_y,
                ..Default::default()
            },
        );
    }
}
