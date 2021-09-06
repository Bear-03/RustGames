use std::fmt;

use macroquad::prelude as mq;
use mq::{Color, Vec2};

pub struct Text {
    pub content: String,
    pos: Vec2,
    font_size: u16,
    color: Color,
}

impl Text {
    pub fn new(content: &str, pos: Vec2, font_size: u16, color: Color) -> Self {
        Self {
            content: content.to_owned(),
            pos,
            font_size,
            color,
        }
    }

    pub fn dimensions(&self) -> mq::TextDimensions {
        mq::measure_text(&self.content, None, self.font_size, 1.0)
    }

    pub fn draw(&self) {
        let dimensions = self.dimensions();

        mq::draw_text(
            &self.content,
            self.pos.x - dimensions.width / 2.0,
            self.pos.y + dimensions.height / 2.0,
            self.font_size as f32,
            self.color,
        );
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
