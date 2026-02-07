use macroquad::math::Rect;
use macroquad::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy)]
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
    pub fn draw_sliced_sprite(
        &self,
        name: &str,
        dest_rect: Rect,
        params: SpriteDrawParams,
    ) -> Result<(), String> {
        if let Some(sprite) = self.get_sprite(name) {
            let (left, right, top, bottom) = sprite.border_rect();
            if (left + right + top + bottom) == 0.0 {
                return Ok(self.draw_stretched_sprite(sprite, dest_rect, params));

                // return self.draw_sprite(name, Vec2::new(dest_rect.x, dest_rect.y), params);
            }
            self.draw_sliced_internal(sprite, dest_rect, params);
            Ok(())
        } else {
            Err("Fail to draw sprite sliced".to_string())
        }
    }
    fn draw_sliced_internal(&self, sprite: &Sprite, dest_rect: Rect, params: SpriteDrawParams) {
        let (left, right, top, bottom) = sprite.border_rect();
        let src_rect = sprite.source_rect();

        // 获取纹理尺寸
        let tex_width = self.texture.width();
        let tex_height = self.texture.height();

        // 转换为 UV 坐标（0-1）
        let src_left = src_rect.x / tex_width;
        let src_right = (src_rect.x + src_rect.w) / tex_width;
        let src_top = src_rect.y / tex_height;
        let src_bottom = (src_rect.y + src_rect.h) / tex_height;

        let src_width = src_rect.w;
        let src_height = src_rect.h;

        // 计算源矩形九个部分的 UV 坐标
        // 左侧、中间、右侧的 X 坐标
        let sx_left = src_left;
        let sx_middle = src_left + left / tex_width;
        let sx_right = src_right - right / tex_width;

        // 顶部、中间、底部的 Y 坐标
        let sy_top = src_top;
        let sy_middle = src_top + top / tex_height;
        let sy_bottom = src_bottom - bottom / tex_height;

        // 源矩形的宽度
        let sw_left = left;
        let sw_middle = src_width - left - right;
        let sw_right = right;

        // 源矩形的高度
        let sh_top = top;
        let sh_middle = src_height - top - bottom;
        let sh_bottom = bottom;

        // 目标矩形的尺寸
        let dest_width = dest_rect.w;
        let dest_height = dest_rect.h;

        // 确保中间区域不会为负数
        let dw_middle = dest_width - left - right;
        let dh_middle = dest_height - top - bottom;

        // 如果中间区域为负，调整边框大小
        let actual_left = if dw_middle < 0.0 {
            left.min(dest_width * 0.5)
        } else {
            left
        };

        let actual_right = if dw_middle < 0.0 {
            right.min(dest_width * 0.5)
        } else {
            right
        };

        let actual_top = if dh_middle < 0.0 {
            top.min(dest_height * 0.5)
        } else {
            top
        };

        let actual_bottom = if dh_middle < 0.0 {
            bottom.min(dest_height * 0.5)
        } else {
            bottom
        };

        let adjusted_dw_middle = dest_width - actual_left - actual_right;
        let adjusted_dh_middle = dest_height - actual_top - actual_bottom;

        // 计算目标矩形九个部分的位置
        let dx_left = dest_rect.x;
        let dx_middle = dest_rect.x + actual_left;
        let dx_right = dest_rect.x + dest_width - actual_right;

        let dy_top = dest_rect.y;
        let dy_middle = dest_rect.y + actual_top;
        let dy_bottom = dest_rect.y + dest_height - actual_bottom;

        // 定义绘制参数
        let draw_params = DrawTextureParams {
            rotation: params.rotation,
            flip_x: params.flip_x,
            flip_y: params.flip_y,
            pivot: None,
            ..Default::default()
        };

        // 1. 左上角（不拉伸）
        if actual_left > 0.0 && actual_top > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_left,
                dy_top,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(actual_left, actual_top)),
                    source: Some(Rect::new(
                        sx_left * tex_width,
                        sy_top * tex_height,
                        sw_left,
                        sh_top,
                    )),
                    ..draw_params
                },
            );
        }

        // 2. 上中（水平拉伸，垂直不拉伸）
        if actual_top > 0.0 && adjusted_dw_middle > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_middle,
                dy_top,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(adjusted_dw_middle, actual_top)),
                    source: Some(Rect::new(
                        sx_middle * tex_width,
                        sy_top * tex_height,
                        sw_middle,
                        sh_top,
                    )),
                    ..draw_params
                },
            );
        }

        // 3. 右上角（不拉伸）
        if actual_right > 0.0 && actual_top > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_right,
                dy_top,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(actual_right, actual_top)),
                    source: Some(Rect::new(
                        sx_right * tex_width,
                        sy_top * tex_height,
                        sw_right,
                        sh_top,
                    )),
                    ..draw_params
                },
            );
        }

        // 4. 左中（垂直拉伸，水平不拉伸）
        if actual_left > 0.0 && adjusted_dh_middle > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_left,
                dy_middle,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(actual_left, adjusted_dh_middle)),
                    source: Some(Rect::new(
                        sx_left * tex_width,
                        sy_middle * tex_height,
                        sw_left,
                        sh_middle,
                    )),
                    ..draw_params
                },
            );
        }

        // 5. 中间（水平和垂直都拉伸）
        if adjusted_dw_middle > 0.0 && adjusted_dh_middle > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_middle,
                dy_middle,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(adjusted_dw_middle, adjusted_dh_middle)),
                    source: Some(Rect::new(
                        sx_middle * tex_width,
                        sy_middle * tex_height,
                        sw_middle,
                        sh_middle,
                    )),
                    ..draw_params
                },
            );
        }

        // 6. 右中（垂直拉伸，水平不拉伸）
        if actual_right > 0.0 && adjusted_dh_middle > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_right,
                dy_middle,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(actual_right, adjusted_dh_middle)),
                    source: Some(Rect::new(
                        sx_right * tex_width,
                        sy_middle * tex_height,
                        sw_right,
                        sh_middle,
                    )),
                    ..draw_params
                },
            );
        }

        // 7. 左下角（不拉伸）
        if actual_left > 0.0 && actual_bottom > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_left,
                dy_bottom,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(actual_left, actual_bottom)),
                    source: Some(Rect::new(
                        sx_left * tex_width,
                        sy_bottom * tex_height,
                        sw_left,
                        sh_bottom,
                    )),
                    ..draw_params
                },
            );
        }

        // 8. 下中（水平拉伸，垂直不拉伸）
        if actual_bottom > 0.0 && adjusted_dw_middle > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_middle,
                dy_bottom,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(adjusted_dw_middle, actual_bottom)),
                    source: Some(Rect::new(
                        sx_middle * tex_width,
                        sy_bottom * tex_height,
                        sw_middle,
                        sh_bottom,
                    )),
                    ..draw_params
                },
            );
        }

        // 9. 右下角（不拉伸）
        if actual_right > 0.0 && actual_bottom > 0.0 {
            draw_texture_ex(
                &self.texture,
                dx_right,
                dy_bottom,
                params.color,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(actual_right, actual_bottom)),
                    source: Some(Rect::new(
                        sx_right * tex_width,
                        sy_bottom * tex_height,
                        sw_right,
                        sh_bottom,
                    )),
                    ..draw_params
                },
            );
        }
    }
    fn draw_stretched_sprite(&self, sprite: &Sprite, dest_rect: Rect, params: SpriteDrawParams) {
        draw_texture_ex(
            &self.texture,
            dest_rect.x,
            dest_rect.y,
            params.color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(dest_rect.w, dest_rect.h)),
                source: Some(Rect::new(sprite.x, sprite.y, sprite.width, sprite.height)),
                rotation: params.rotation,
                flip_x: params.flip_x,
                flip_y: params.flip_y,
                ..Default::default()
            },
        );
    }
}

mod test {
    #[test]
    fn test1() {}
}
