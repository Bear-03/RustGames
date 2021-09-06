use macroquad::prelude as mq;
use mq::{vec2, KeyCode, Vec2};

use super::collider::Collider;

const POSITION_MARGIN: f32 = 20.0;

pub struct RacketPair(pub Racket, pub Racket);

impl RacketPair {
    pub fn new() -> Self {
        Self(
            Racket::new(RacketSide::Left),
            Racket::new(RacketSide::Right),
        )
    }

    pub fn get_all_controls(&self) -> [KeyCode; 4] {
        let controls0 = &self.0.controls;
        let controls1 = &self.1.controls;

        [controls0.up, controls0.down, controls1.up, controls1.down]
    }
}

pub struct RacketKeys {
    pub up: KeyCode,
    pub down: KeyCode,
}

pub enum RacketSide {
    Left,
    Right,
}

pub struct Racket {
    pub controls: RacketKeys,
    pos: Vec2,
    width: f32,
    height: f32,
    speed: f32,
}

impl Racket {
    pub fn new(side: RacketSide) -> Self {
        let controls = Self::get_controls(&side);
        let default = Self::default_with_controls(controls);

        let x = match side {
            RacketSide::Left => POSITION_MARGIN,
            RacketSide::Right => mq::screen_width() - default.width - POSITION_MARGIN,
        };

        Self {
            pos: vec2(x, mq::screen_height() / 2.0 - default.height / 2.0),
            ..default
        }
    }

    fn default_with_controls(controls: RacketKeys) -> Self {
        Self {
            pos: vec2(0.0, 0.0),
            width: 20.0,
            height: 150.0,
            speed: 400.0,
            controls,
        }
    }

    fn get_controls(side: &RacketSide) -> RacketKeys {
        match side {
            RacketSide::Left => RacketKeys {
                up: KeyCode::W,
                down: KeyCode::S,
            },
            RacketSide::Right => RacketKeys {
                up: KeyCode::Up,
                down: KeyCode::Down,
            },
        }
    }

    pub fn update(&mut self) {
        self.update_pos();
    }

    fn update_pos(&mut self) {
        if mq::is_key_down(self.controls.up) {
            self.pos.y -= self.speed * mq::get_frame_time();
        }

        if mq::is_key_down(self.controls.down) {
            self.pos.y += self.speed * mq::get_frame_time();
        }

        self.pos.y = mq::clamp(self.pos.y, 0.0, mq::screen_height() - self.height);
    }

    pub fn draw(&self) {
        mq::draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, mq::WHITE);
    }
}

impl Collider for Racket {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn dim(&self) -> Vec2 {
        vec2(self.width, self.height)
    }
}
