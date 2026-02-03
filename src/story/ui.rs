use macroquad::prelude::*;

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
