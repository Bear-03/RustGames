use super::{Text, Timer};
use macroquad::prelude as mq;
use mq::vec2;

#[allow(dead_code)]
pub enum FpsCounterType {
    Min,
    Max,
    Average,
}

pub struct FpsCounter {
    counter_type: FpsCounterType,
    displayed_text: Text,
    display_timer: Timer,
    fps_list: Vec<i32>,
}

impl FpsCounter {
    #[allow(dead_code)]
    pub fn new(refresh_seconds: f32, fps_counter_type: FpsCounterType) -> Self {
        Self {
            counter_type: fps_counter_type,
            display_timer: Timer::new(refresh_seconds),
            ..Self::default()
        }
    }

    fn reduce_fps_list(&self) -> i32 {
        if self.fps_list.is_empty() {
            return 0;
        }

        match self.counter_type {
            // List will never be empty so there will always be a min and a max
            FpsCounterType::Min => *self.fps_list.iter().min().unwrap(),
            FpsCounterType::Max => *self.fps_list.iter().max().unwrap(),
            FpsCounterType::Average => {
                self.fps_list.iter().sum::<i32>() / self.fps_list.len() as i32
            }
        }
    }

    pub fn update(&mut self) {
        if self.display_timer.is_over() {
            self.displayed_text.content = self.reduce_fps_list().to_string();
            self.fps_list.clear();
            return;
        }

        self.fps_list.push(mq::get_fps());
    }

    pub fn draw(&self) {
        self.displayed_text.draw();
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            counter_type: FpsCounterType::Min,
            displayed_text: Text::new(&mq::get_fps().to_string(), vec2(30.0, 30.0), 40, mq::GREEN),
            display_timer: Timer::new(0.5),
            fps_list: Vec::new(),
        }
    }
}
