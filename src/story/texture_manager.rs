use anyhow::Result;
use image::ImageReader;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;

pub struct TextureManager {
    pub textures: HashMap<String, Texture2D>,
}

impl TextureManager {
    pub async fn load_texture_auto(
        &mut self,
        path: &str,
        texture_name: Option<&str>,
    ) -> Result<()> {
        let extension = path.split('.').last().unwrap_or("").to_lowercase();

        match extension.as_str() {
            "png" | "bmp" | "tga" | "gif" => {
                let texture = load_texture(path)
                    .await
                    .map_err(|e| format!("加载失败 {}: {:?}", path, e))
                    .unwrap();
                if let Some(name) = texture_name {
                    self.textures.insert(name.to_string(), texture);
                } else {
                    self.textures.insert(path.to_string(), texture);
                }
                Ok(())
            }
            "jpg" | "jpeg" | "webp" => {
                let bytes = std::fs::read(path)?;

                let img = ImageReader::new(Cursor::new(&bytes))
                    .with_guessed_format()?
                    .decode()?
                    .to_rgba8();

                let (width, height) = img.dimensions();
                let texture = Texture2D::from_rgba8(width as u16, height as u16, &img);
                if let Some(name) = texture_name {
                    self.textures.insert(name.to_string(), texture);
                } else {
                    self.textures.insert(path.to_string(), texture);
                }
                Ok(())
            }
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
