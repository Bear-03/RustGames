use super::Timer;
use macroquad::prelude as mq;

pub enum FpsCounterType {
    Min,
    Max,
    Average,
}

pub struct FpsCounter {
    counter_type: FpsCounterType,
    displayed_fps: i32,
    display_timer: Timer,
    fps_list: Vec<i32>,
}

impl FpsCounter {
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
            self.displayed_fps = self.reduce_fps_list();
            self.fps_list.clear();
            return;
        }

        self.fps_list.push(mq::get_fps());
    }

    pub fn draw(&self) {
        mq::draw_text(&self.displayed_fps.to_string(), 10.0, 40.0, 40.0, mq::GREEN);
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            counter_type: FpsCounterType::Min,
            displayed_fps: mq::get_fps(),
            display_timer: Timer::new(0.5),
            fps_list: Vec::new(),
        }
    }
}
