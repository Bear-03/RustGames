use macroquad::prelude as mq;
use mq::vec2;

use common::Text;

const MARKER_GAP: f32 = 50.0;
const TOP_MARGIN: f32 = 50.0;
const FONT_SIZE: u16 = 100;

pub struct ScoreManager {
    pub player1: ScoreMarker,
    pub player2: ScoreMarker,
}

impl ScoreManager {
    pub fn new() -> Self {
        let horizontal_middle = mq::screen_width() / 2.0;

        Self {
            // The gap is appied to the X axis, separating he markers from the center
            player1: ScoreMarker::new(horizontal_middle - MARKER_GAP),
            player2: ScoreMarker::new(horizontal_middle + MARKER_GAP),
        }
    }

    pub fn draw(&self) {
        self.player1.draw();
        self.player2.draw();
    }
}

pub struct ScoreMarker {
    text: Text,
    score: u32,
}

impl ScoreMarker {
    pub fn new(x: f32) -> Self {
        Self {
            score: 0,
            text: Text::new("0", vec2(x, TOP_MARGIN), FONT_SIZE, mq::WHITE),
        }
    }

    pub fn increase_score(&mut self) -> u32 {
        self.score += 1;

        self.text.content = self.score.to_string();

        self.score
    }

    pub fn draw(&self) {
        self.text.draw();
    }
}
