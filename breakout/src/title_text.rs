use macroquad::prelude::*;

pub fn draw_title_text(text: &str, font: Font) {
    let text_size = measure_text(text, Some(font), 60u16, 1.0);

    draw_text_ex(
        text,
        (screen_width() - text_size.width) * 0.5f32,
        (screen_height() - text_size.height) * 0.5f32,
        TextParams {
            font,
            font_size: 60u16,
            color: BLACK,
            ..Default::default()
        },
    );
}
