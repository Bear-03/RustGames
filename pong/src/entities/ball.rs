use macroquad::prelude as mq;
use mq::{vec2, Vec2};
use std::f32::consts::PI;

use rand::distributions::Uniform;
use rand::{self, Rng};

use super::collider::Collider;

const QUADRANT_MIDDLE: f32 = PI / 4.0;

pub struct Ball {
    pub direction: Vec2,
    pub pos: Vec2,
    pub size: f32,
    speed: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            pos: vec2(mq::screen_width() / 2.0, mq::screen_height() / 2.0),
            direction: Self::get_random_direction(),
            size: 20.0,
            speed: 400.0,
        }
    }

    fn get_random_direction() -> Vec2 {
        let mut rng = rand::thread_rng();

        let angle = rng.sample(Uniform::new(-QUADRANT_MIDDLE, QUADRANT_MIDDLE));
        let flip_angle_range = rng.gen_bool(0.5);

        let angle = if flip_angle_range { angle + PI } else { angle };

        vec2(f32::cos(angle), f32::sin(angle))
    }

    fn velocity(&self) -> Vec2 {
        self.direction * self.speed
    }

    pub fn update(&mut self) {
        self.update_pos();
    }

    fn update_pos(&mut self) {
        self.pos += self.velocity() * mq::get_frame_time();
    }

    pub fn draw(&self) {
        mq::draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, mq::WHITE);
    }
}

impl Collider for Ball {
    fn pos(&self) -> Vec2 {
        self.pos
    }

    fn dim(&self) -> Vec2 {
        vec2(self.size, self.size)
    }
}
