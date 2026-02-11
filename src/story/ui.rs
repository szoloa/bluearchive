use macroquad::prelude::*;

pub fn draw_dialog_box(
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

pub fn draw_chioce(
    current_choices: &Vec<inkling::Choice>,
    texture: &Texture2D,
    material: Option<&Material>,
    font: Option<&Font>,
) {
    for (i, choice) in current_choices.iter().enumerate() {
        let y_pos = screen_height() / 3.0 + (i as f32 * 84.0);
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

        draw_text_center(
            &text,
            screen_width() / 2.,
            y_pos + 2.,
            font,  // 指定字体
            BLACK, // 颜色
            30.,   // 字体大小
        );

        // 鼠标悬停效果
        // let mouse_pos = mouse_position();
        // let rect = Rect::new(60.0, y_pos - 48.0, 700.0, 64.0);
        // if rect.contains(vec2(mouse_pos.0, mouse_pos.1)) {
        //     draw_rectangle_lines(55.0, y_pos - 50.0, 710.0, 64.0, 2.0, WHITE);
        // }
    }
}

pub fn draw_background(texture: &Texture2D) {
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

pub fn draw_vertical_gradient(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    start_color: Color,
    end_color: Color,
) {
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

pub fn draw_text_center(
    text: &str,
    x: f32,
    y: f32,
    font: Option<&Font>,
    color: Color,
    font_size: f32,
) {
    let text_param = TextParams {
        font: font,                  // 指定字体
        font_size: font_size as u16, // 字体大小
        font_scale: 1.0,             // 缩放因子
        font_scale_aspect: 1.0,
        color: color,         // 颜色
        ..Default::default()  // 其他参数保持默认
    };
    let word_width = measure_text(text, font, font_size as u16, 1.0).width;
    draw_text_ex(text, x - word_width / 2., y, text_param);
}

pub fn draw_text_wrapped(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    max_width: f32,
    color: Color,
    font: Option<&Font>,
) {
    let text_param = TextParams {
        font: font,                  // 指定字体
        font_size: font_size as u16, // 字体大小
        font_scale: 1.0,             // 缩放因子
        font_scale_aspect: 1.0,
        color: color,         // 颜色
        ..Default::default()  // 其他参数保持默认
    };
    // 初始行位置
    let mut current_y = y;
    // 当前行累积的文本
    let mut current_line = String::new();
    // 当前行已使用的宽度
    let mut current_width = 0.0;

    // 按单词分割（英文以空格分隔，中文等通常每个字符都是“单词”）
    // 这是一个简单的实现，对于中英文混合文本，你可能需要更复杂的分词
    // let words: Vec<&str> = text.split_whitespace().collect();

    for word in text.split("") {
        // 检查当前行是否为空
        // let word_with_space = if current_line.is_empty() {
        //     word.to_string()
        // } else {
        //     format!(" {}", word)
        // };

        // 计算添加这个词后的宽度
        let word_width = measure_text(word, font, font_size as u16, 1.0).width;

        // 如果当前行是空的，或者加上这个词后不超过最大宽度，就添加到当前行
        if current_line.is_empty() || current_width + word_width <= max_width {
            current_line.push_str(word);
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

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec2,
    pub uv: Vec2,
}
impl Vertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Vertex {
            pos: vec2(x, y),
            uv: vec2(u, v),
        }
    }
}
// 实现必要的 trait 以用于 `draw_mesh`
impl From<Vertex> for macroquad::models::Vertex {
    fn from(v: Vertex) -> Self {
        Self {
            position: vec3(v.pos.x, v.pos.y, 0.0),
            uv: v.uv,
            color: WHITE.into(),
            normal: Vec4::new(0., 0., 0., 0.),
        }
    }
}
impl From<&Vertex> for macroquad::models::Vertex {
    fn from(v: &Vertex) -> Self {
        Self {
            position: vec3(v.pos.x, v.pos.y, 0.0),
            uv: v.uv,
            color: WHITE.into(),
            normal: Vec4::new(0., 0., 0., 0.),
        }
    }
}

/// 构建一个带纹理的圆角四边形网格
/// # 参数
/// * `rect`: 四边形的外包围矩形 (x, y, width, height)
/// * `radius`: 圆角半径
/// * `segments`: 每个圆角的细分段数（越高越平滑，通常8-16足够）
pub fn build_rounded_rect_mesh(rect: Rect, radius: f32, segments: u8) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 确保半径有效
    let radius = radius.min(rect.w * 0.5).min(rect.h * 0.5);
    let segments = segments.max(1) as usize;

    // 辅助函数：计算相对于矩形左上角的UV坐标
    let calc_uv = |x: f32, y: f32| -> (f32, f32) {
        let u = (x - rect.x) / rect.w;
        let v = (y - rect.y) / rect.h;
        (u.max(0.0).min(1.0), v.max(0.0).min(1.0)) // 确保在[0,1]范围内
    };

    // 定义四个圆角的圆心（内角点）
    let corners = [
        vec2(rect.x + radius, rect.y + radius),          // 左上
        vec2(rect.x + rect.w - radius, rect.y + radius), // 右上
        vec2(rect.x + rect.w - radius, rect.y + rect.h - radius), // 右下
        vec2(rect.x + radius, rect.y + rect.h - radius), // 左下
    ];

    // --- 1. 中心矩形区域 ---
    let center_vert_start_idx = vertices.len() as u16;

    // 计算四个内角点的UV
    let (u1, v1) = calc_uv(rect.x + radius, rect.y + radius);
    let (u2, v2) = calc_uv(rect.x + rect.w - radius, rect.y + radius);
    let (u3, v3) = calc_uv(rect.x + rect.w - radius, rect.y + rect.h - radius);
    let (u4, v4) = calc_uv(rect.x + radius, rect.y + rect.h - radius);

    // 添加中心矩形的四个顶点
    vertices.push(Vertex::new(rect.x + radius, rect.y + radius, u1, v1));
    vertices.push(Vertex::new(
        rect.x + rect.w - radius,
        rect.y + radius,
        u2,
        v2,
    ));
    vertices.push(Vertex::new(
        rect.x + rect.w - radius,
        rect.y + rect.h - radius,
        u3,
        v3,
    ));
    vertices.push(Vertex::new(
        rect.x + radius,
        rect.y + rect.h - radius,
        u4,
        v4,
    ));

    // 中心矩形的两个三角形
    indices.extend_from_slice(&[
        center_vert_start_idx,
        center_vert_start_idx + 1,
        center_vert_start_idx + 2,
    ]);
    indices.extend_from_slice(&[
        center_vert_start_idx,
        center_vert_start_idx + 2,
        center_vert_start_idx + 3,
    ]);

    // --- 2. 四个圆角区域 ---
    // 每个圆角的起始和结束角度（弧度）
    let corner_angles = [
        (std::f32::consts::PI, std::f32::consts::PI * 1.5), // 左上：π 到 1.5π
        (std::f32::consts::PI * 1.5, 2.0 * std::f32::consts::PI), // 右上：1.5π 到 2π
        (0.0, std::f32::consts::PI * 0.5),                  // 右下：0 到 0.5π
        (std::f32::consts::PI * 0.5, std::f32::consts::PI), // 左下：0.5π 到 π
    ];

    for (corner_idx, ((start_angle, end_angle), &center)) in
        corner_angles.iter().zip(corners.iter()).enumerate()
    {
        let vert_start_idx = vertices.len() as u16;

        // 圆角的圆心顶点
        let (cu, cv) = calc_uv(center.x, center.y);
        vertices.push(Vertex::new(center.x, center.y, cu, cv));

        // 生成圆角弧线上的顶点
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + (end_angle - start_angle) * t;

            // 计算圆角弧线上的点
            let dir = vec2(angle.cos(), angle.sin());
            let pos = center + dir * radius;

            // 计算正确的UV坐标
            let (u, v) = calc_uv(pos.x, pos.y);
            vertices.push(Vertex::new(pos.x, pos.y, u, v));

            // 调试输出：检查每个顶点的UV
            if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
                println!(
                    "警告: 顶点{} (圆角{}) UV超出范围: ({:.3}, {:.3}), 位置: ({:.1}, {:.1})",
                    vertices.len() - 1,
                    corner_idx,
                    u,
                    v,
                    pos.x,
                    pos.y
                );
            }
        }

        // 为这个圆角生成三角形索引（扇形）
        for i in 0..(segments as u16) {
            indices.extend_from_slice(&[
                vert_start_idx,         // 圆心
                vert_start_idx + 1 + i, // 当前弧线点
                vert_start_idx + 2 + i, // 下一个弧线点
            ]);
        }
    }

    (vertices, indices)
}

// 创建着色器
use macroquad::miniquad::*;

/// 创建圆角矩形材质
pub fn create_rounded_rect_material() -> Material {
    let vertex_shader = r#"#version 330 core
    in vec3 position;
    in vec2 texcoord;
    in vec4 color0;

    out vec2 uv;
    out vec4 color;

    uniform mat4 Model;
    uniform mat4 Projection;
    uniform float u_skew_x;  // 水平斜度，例如0.2表示倾斜20%
    uniform float u_skew_y;  // 垂直斜度

    void main() {
        mat3 skew_matrix = mat3(
                1.0, u_skew_x, 0.0,
                u_skew_y, 1.0, 0.0,
                0.0, 0.0, 1.0
            );
        vec3 skewed_position = skew_matrix * vec3(position.xy, 1.0);
        gl_Position = Projection * Model * vec4(skewed_position.xy, position.z, 1.0);
        uv = texcoord;
        color = color0;
    }
    "#;

    // 或者使用更简单的片段着色器版本（优化版）
    let fragment_shader = r#"#version 330 core
    in vec2 uv;
    in vec4 color;

    out vec4 FragColor;

    uniform sampler2D Texture;
    uniform vec2 u_tile_size;
    uniform vec2 u_rect_size;    // 矩形大小
    uniform vec2 u_texture_size; // 纹理实际大小
    uniform float u_radius;
    uniform vec4 u_texture_rect; // 纹理显示区域：x,y,width,height (0-1范围)
    uniform vec2 u_offset;

    void main() {
        vec2 tex_coord;
        vec2 tiled_uv = uv * u_texture_size / u_tile_size;
        float tex_x = u_texture_rect.x;
        float tex_y = u_texture_rect.y;
        float tex_w = u_texture_rect.z;
        float tex_h = u_texture_rect.w;
        vec2 rect_ratio = u_rect_size / u_texture_size;
        float scale = min(rect_ratio.x, rect_ratio.y)  ;
        vec2 tile_count = u_rect_size / u_tile_size;
        vec2 final_uv = uv;

        if (u_tile_size.x > 0.0 && u_tile_size.y > 0.0) {
            // 计算矩形中包含多少个平铺单元
            vec2 tile_count = u_rect_size / u_tile_size;
            // 对UV进行平铺
            tiled_uv = fract(uv * tile_count);
        } else {
            // 不使用平铺，直接使用原始UV
            tiled_uv = uv;
        }
        final_uv = fract(tiled_uv + u_offset);

        // 当前像素在矩形中的位置（从0到size）
        vec2 pos = uv * u_rect_size;
        vec2 q = abs(pos - u_rect_size * 0.5) - (u_rect_size * 0.5 - u_radius);
        float dist = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0);
        float alpha = 1.0 - smoothstep(u_radius - 1.0, u_radius + 1.0, dist);

        if (alpha <= 0.0) {
            discard;
        }
        vec2 texture_uv = (3.0 >= 2.0) ?
                fract(final_uv * tile_count) : final_uv;
        vec4 tex_color = texture(Texture, texture_uv);

        FragColor = vec4(tex_color.rgb, tex_color.a * alpha);
    }
    "#;

    load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: fragment_shader,
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                )),
                ..Default::default()
            },
            uniforms: vec![
                UniformDesc::new("u_rect_size", UniformType::Float2),
                UniformDesc::new("u_texture_size", UniformType::Float2),
                UniformDesc::new("u_radius", UniformType::Float1),
                UniformDesc::new("u_texture_rect", UniformType::Float4),
                UniformDesc::new("u_skew_x", UniformType::Float1),
                UniformDesc::new("u_skew_y", UniformType::Float1),
                UniformDesc::new("u_tile_size", UniformType::Float2),
                UniformDesc::new("u_offset", UniformType::Float2),
            ],
            textures: vec!["texture".to_string()],
        },
    )
    .expect("Failed to create rounded rect material")
}

pub fn draw_chioce_material() -> Material {
    let material = create_rounded_rect_material();
    let rect_size = vec2(600.0, 50.0);

    let texture_rect = vec4(0., 0.0, 1.0, 1.0);
    let texture_size = vec2(1280. as f32 / 2., 900. as f32 / 5.);
    let offset = Vec2::new(0., 0.6);

    let radius = 5.0;
    let skew_x = 0.; // 20%的水平斜度
    let skew_y = -0.2; // 垂直斜度

    material.set_uniform("u_rect_size", rect_size);
    material.set_uniform("u_radius", radius as f32);
    material.set_uniform("u_texture_size", texture_size);
    material.set_uniform("u_tile_size", texture_size);
    material.set_uniform("u_texture_rect", texture_rect);
    material.set_uniform("u_skew_x", skew_x as f32);
    material.set_uniform("u_skew_y", skew_y as f32);
    material.set_uniform("u_offset", offset);
    material
}
